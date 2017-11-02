/*!

# An algorithm for efficiently checking a URL against a CSP

## How the standard defines it

According to <https://www.w3.org/TR/CSP/>, you're supposed to run this pseudo-code:

```ignore
for policy in policy_list {
    for source_expression in policy[request.type] {
        if source_expression.blocks(request.url) {
            return Err(policy.disposition);
        }
    }
}
return Ok;
```

This is, of course, unperformant when your page has a large policy.

## How this implementation tries to implement something that behaves exactly the same, but faster

The algorithm implemented here is intended to eliminate the inner `for/if` construct,
replacing it with a lookup in a [radix tree]-like structure.

```ignore
for policy in policy_list {
    if policy[request.type].find_matching_host(request.url.host).find_matching_path(request.url.path).is_none() {
        return Err(policy.disposition);
    }
}
return Ok;
```

A future version of this will probably eliminate the outer loop, too,
turning the whole thing into a tree traversal,
but we are required to report the first policy that fails,
meaning that order would have to be tracked within the tree somehow.

[radix tree]: https://en.wikipedia.org/wiki/Radix_tree

But, anyway, the data structure for a source expression list like this:

```ignore
script-src example.com/ *.examplecdn.com/scripts/ *.examplecdn.com/script/ *.examplecdn.com/js/ cdn.framework.org;
style-src cdn.framework.org example.com
```

Will be turned into a tree that looks like this:

```ignore
     /--------\
     | [root] |
     \--------/
     /        \
/-----\    /-----\
| com |    | org |
\-----/    \-----/
   |   \___    \_______________________________
   |       \__                                 \
/---------\   \                                 \
| example |   /------------\                 /-----------\
\---------/   | examplecdn |                 | framework |
    |         \------------/                 \-----------/
    |                   |                         |
/--------------\   /------------\              /-----\
| [terminator] |   | [wildcard] |              | cdn |
\--------------/   \------------/              \-----/
    |                   |                          |
 /===\               /===\                      /------------\
 | / |               | / |                      | [wildcard] |
 \===/               \===/                      \------------/
    |               /  | \_________                    |
  (script, style) _/   |           \                 /===\
                 /  /==========\ /=====\             | / |
       /=========\  | scripts/ | | js/ |             \===/
       | script/ |  \==========/ \=====/               \________
       \=========/         |        |                           \
           |            (script)  (script)                     (script, style)
        (script)
```

* domain names are flipped backwards, on the assumption that the TLD is duplicated
  way more often than the other end. Also, this puts the wildcards at the end,
  instead of the beginning, which is way easier to implement.
* this tree is build a-component-at-a-time, not treating domains or paths as strings
  because of some ways that the spec makes that easier to implement correctly

You may also notice that there is no use of threads in rust-content-security-policy at all.
However, the parsed tree does implement `Send` and `Sync`, so a document with many URLs to check
can use threads that way, if it proves advantageous.

*/

use check::search;
use std::borrow::Cow;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::cmp::Ordering::*;
use std::collections::HashMap;
use std::mem;
use url::percent_encoding::percent_decode;

use super::{Resource, ResourceFlags};

#[derive(Debug)]
pub struct HostNode<'a> {
    terminal: PathNode<'a>,
    wildcard: PathNode<'a>,
    children: HashMap<&'a [u8], HostNode<'a>>,
}

impl<'a> HostNode<'a> {
    pub fn new() -> Self {
        HostNode {
            terminal: PathNode::new(),
            wildcard: PathNode::new(),
            children: HashMap::new(),
        }
    }
    fn check_<'b, I: Iterator<Item=&'b [u8]>>(&self, resource: Resource, parts: &'b mut I, path: &'b [u8]) -> bool {
        if let Some(part) = parts.next() {
            (if let Some(child) = self.children.get(part) {
                child.check_(resource, parts, path)
            } else {
                false
            }) || self.wildcard.check(resource, path)
        } else {
            self.terminal.check(resource, path)
        }
    }
    pub fn check<'b>(&self, resource: Resource, host: &'b [u8], path: &'b [u8]) -> bool {
        self.check_(resource, &mut host.split(|&x| x == b'.').rev(), path)
    }
    pub fn insert(&mut self, resource: Resource, host: &'a [u8], path: &'a [u8]) {
        self.insert_(resource, &mut host.split(|&x| x == b'.').rev(), path)
    }
    fn insert_<'b, I: Iterator<Item=&'a [u8]>>(&mut self, resource: Resource, parts: &'b mut I, path: &'a [u8]) {
        if let Some(part) = parts.next() {
            if part == b"*" {
                self.wildcard.insert(resource, path)
            } else {
                self.children.entry(part)
                    .or_insert_with(HostNode::new)
                    .insert_(resource, parts, path)
            }
        } else {
            self.terminal.insert(resource, path)
        }
    }
}

#[derive(Debug)]
pub struct PathNode<'a> {
    exact_match_flags: ResourceFlags,
    inexact_match_flags: ResourceFlags,
    children: HashMap<Cow<'a, [u8]>, PathNode<'a>>,
}

impl<'a> PathNode<'a> {
    pub fn new() -> Self {
        PathNode {
            exact_match_flags: ResourceFlags::empty(),
            inexact_match_flags: ResourceFlags::empty(),
            children: HashMap::new(),
        }
    }
    pub fn insert(&mut self, resource: Resource, mut path: &'a [u8]) {
        let exact_match = path.len() != 0 && path.get(path.len() - 1) != Some(&b'/');
        self.insert_(resource, &mut path.split(|&x| x == b'/'), exact_match);
    }
    fn insert_<'b, I: Iterator<Item=&'a [u8]>>(&mut self, resource: Resource, parts: &'b mut I, exact_match: bool) {
        if let Some(part) = parts.next() {
            if part == b"" {
                return self.insert_(resource, parts, exact_match);
            }
            self.children.entry(percent_decode(part).into())
                .or_insert_with(PathNode::new)
                .insert_(resource, parts, exact_match);
        } else {
            let flag = resource.flag();
            if exact_match {
                self.exact_match_flags |= flag;
            } else {
                self.inexact_match_flags |= flag;
            }
        }
    }
    pub fn check<'b>(&self, resource: Resource, mut path: &'b [u8]) -> bool {
        if path.get(0) == Some(&b'/') {
            path = &path[1..];
        }
        self.check_(resource, &mut path.split(|&x| x == b'/'))
    }
    fn check_<'b, I: Iterator<Item=&'b [u8]>>(&self, resource: Resource, parts: &'b mut I) -> bool {
        let flag = resource.flag();
        self.inexact_match_flags.contains(flag)
        || (if let Some(part) = parts.next() {
            let part: Cow<[u8]> = percent_decode(part).into();
            if let Some(child) = self.children.get(part.as_ref()) {
                child.check_(resource, parts)
            } else {
                false
            }
        } else {
            self.exact_match_flags.contains(flag)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    macro_rules! do_tree_test {
        ($i:ident, $mode:expr, $find:expr; $( $item:expr),*) => {
            #[test]
            fn $i() {
                let mut tree = PathNode::new();
                $(
                    tree.insert(Resource::ScriptSrc, $item);
                )*
                assert_eq!(tree.check(Resource::ScriptSrc, $find), $mode);
            }
        }
    }
    do_tree_test!{empty_simple, false, "".as_bytes(); }
    do_tree_test!{empty_text, false, "abc".as_bytes(); }
    do_tree_test!{empty_rooted, false, "/abc".as_bytes(); }
    do_tree_test!{empty_root, false, "/".as_bytes(); }
    do_tree_test!{root_match, true, "/".as_bytes(); "/".as_bytes()}
    do_tree_test!{root_equiv_match, true, "".as_bytes(); "/".as_bytes()}
    do_tree_test!{root_equiv2_match, true, "/".as_bytes(); "".as_bytes()}
    do_tree_test!{rooted_match, true, "/test".as_bytes(); "/".as_bytes()}
    do_tree_test!{rooted_nomatch, false, "/test".as_bytes(); "/xxx".as_bytes()}
    do_tree_test!{rooted_nomatch_prefix, false, "/".as_bytes(); "/xxx".as_bytes()}
    do_tree_test!{two_nomatch, false, "/xxx".as_bytes(); "/test".as_bytes(), "/test2".as_bytes()}

    #[test]
    fn prefixed_mixed_match() {
        let mut tree = PathNode::new();
        tree.insert(Resource::StyleSrc, "/a".as_bytes());
        tree.insert(Resource::StyleSrc, "/a/b".as_bytes());
        tree.insert(Resource::StyleSrc, "/a/b/c".as_bytes());
        tree.insert(Resource::ScriptSrc, "/a/b/c".as_bytes());
        assert_eq!(tree.check(Resource::ScriptSrc, "/a".as_bytes()), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "/a/b".as_bytes()), false);
        assert_eq!(tree.check(Resource::StyleSrc, "/a/b/c".as_bytes()), true);
        assert_eq!(tree.check(Resource::ScriptSrc, "/a/b/c".as_bytes()), true);
    }

    #[test]
    fn prefixed_mixed_one_match() {
        let mut tree = PathNode::new();
        tree.insert(Resource::StyleSrc, "/a/".as_bytes());
        tree.insert(Resource::StyleSrc, "/a/b/".as_bytes());
        tree.insert(Resource::ScriptSrc, "/a/b/c/".as_bytes());
        assert_eq!(tree.check(Resource::ScriptSrc, "/a/".as_bytes()), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "/a/b/".as_bytes()), false);
        assert_eq!(tree.check(Resource::StyleSrc, "/a/b/c/".as_bytes()), true);
    }//
//
    #[test]
    fn prefixed_mixed_parent_match() {
        let mut tree = PathNode::new();
        tree.insert(Resource::StyleSrc, "/a/".as_bytes());
        tree.insert(Resource::ScriptSrc, "/a/b/".as_bytes());
        tree.insert(Resource::ScriptSrc, "/a/b/c/".as_bytes());
        assert_eq!(tree.check(Resource::StyleSrc, "/a/".as_bytes()), true);
        assert_eq!(tree.check(Resource::StyleSrc, "/a/b/".as_bytes()), true);
        assert_eq!(tree.check(Resource::StyleSrc, "/a/b/c/".as_bytes()), true);
    }

    #[test]
    fn host_tree_empty() {
        let mut tree = HostNode::new();
        assert_eq!(tree.check(Resource::ScriptSrc, "".as_bytes(), "".as_bytes()), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com".as_bytes(), "script".as_bytes()), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com".as_bytes(), "script.js".as_bytes()), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "cdn.google.com".as_bytes(), "script.js".as_bytes()), false);
    }

    #[test]
    fn host_tree_basic() {
        let mut tree = HostNode::new();
        tree.insert(Resource::ScriptSrc, "google.com".as_bytes(), "script".as_bytes());
        assert_eq!(tree.check(Resource::ScriptSrc, "".as_bytes(), "".as_bytes()), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com".as_bytes(), "script".as_bytes()), true);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com".as_bytes(), "script.js".as_bytes()), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "cdn.google.com".as_bytes(), "script.js".as_bytes()), false);
    }

    #[test]
    fn host_tree_wildcard() {
        let mut tree = HostNode::new();
        tree.insert(Resource::ScriptSrc, "*.google.com".as_bytes(), "script/".as_bytes());
        assert_eq!(tree.check(Resource::ScriptSrc, "".as_bytes(), "".as_bytes()), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com".as_bytes(), "script/".as_bytes()), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com".as_bytes(), "script/js".as_bytes()), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "cdn.google.com".as_bytes(), "script/js".as_bytes()), true);
    }

    #[test]
    fn host_tree_mixed() {
        let mut tree = HostNode::new();
        tree.insert(Resource::ScriptSrc, "google.com".as_bytes(), "script/".as_bytes());
        tree.insert(Resource::ScriptSrc, "*.google.com".as_bytes(), "script/".as_bytes());
        assert_eq!(tree.check(Resource::ScriptSrc, "".as_bytes(), "".as_bytes()), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com".as_bytes(), "script/".as_bytes()), true);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com".as_bytes(), "script/js".as_bytes()), true);
        assert_eq!(tree.check(Resource::ScriptSrc, "cdn.google.com".as_bytes(), "script/js".as_bytes()), true);
    }

    #[test]
    fn host_tree_mixed_scheme() {
        let mut tree = HostNode::new();
        tree.insert(Resource::StyleSrc, "google.com".as_bytes(), "script/".as_bytes());
        tree.insert(Resource::ScriptSrc, "*.google.com".as_bytes(), "script/".as_bytes());
        assert_eq!(tree.check(Resource::ScriptSrc, "".as_bytes(), "".as_bytes()), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com".as_bytes(), "script/".as_bytes()), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com".as_bytes(), "script/js".as_bytes()), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "cdn.google.com".as_bytes(), "script/js".as_bytes()), true);
    }

    #[test]
    fn host_tree_fallback_after_wildcard() {
        let mut tree = HostNode::new();
        tree.insert(Resource::ScriptSrc, "*.google.com".as_bytes(), "style".as_bytes());
        tree.insert(Resource::ScriptSrc, "cdn.google.com".as_bytes(), "script".as_bytes());
        assert_eq!(tree.check(Resource::ScriptSrc, "".as_bytes(), "".as_bytes()), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "users.google.com".as_bytes(), "style".as_bytes()), true);
        assert_eq!(tree.check(Resource::ScriptSrc, "users.google.com".as_bytes(), "script".as_bytes()), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "cdn.google.com".as_bytes(), "style".as_bytes()), true);
        assert_eq!(tree.check(Resource::ScriptSrc, "cdn.google.com".as_bytes(), "script".as_bytes()), true);
    }

    #[test]
    fn host_tree_mixed_resource_type() {
        let mut tree = HostNode::new();
        tree.insert(Resource::StyleSrc, "*.google.com".as_bytes(), "style".as_bytes());
        tree.insert(Resource::ScriptSrc, "cdn.google.com".as_bytes(), "script".as_bytes());
        assert_eq!(tree.check(Resource::StyleSrc, "users.google.com".as_bytes(), "style".as_bytes()), true);
        assert_eq!(tree.check(Resource::ScriptSrc, "users.google.com".as_bytes(), "style".as_bytes()), false);
        assert_eq!(tree.check(Resource::StyleSrc, "cdn.google.com".as_bytes(), "script".as_bytes()), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "cdn.google.com".as_bytes(), "script".as_bytes()), true);
    }

    #[test]
    fn url_decodes() {
        let mut tree = HostNode::new();
        tree.insert(Resource::ScriptSrc, "cdn.com".as_bytes(), "sc%72ipt".as_bytes());
        assert_eq!(tree.check(Resource::ScriptSrc, "cdn.com".as_bytes(), "scri%70t".as_bytes()), true);
    }
}