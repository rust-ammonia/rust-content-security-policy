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
    |                  | \_________                    |
  (script, style)      |           \                 /===\
                    /========\ /=====\               | / |
                    | script | | js/ |               \===/
                    \========/ \=====/                 \________
                       |   |        \__________                 \
                       |   |                   \              (script, style)
                    /====\ /===\          (script)             
                    | s/ | | / |
                    \====/ \===/
                      |       |
                    (script) (script)
```

* domain names are flipped backwards, on the assumption that the TLD is duplicated
  way more often than the other end. Also, this puts the wildcards at the end,
  instead of the beginning.
* domain names are processed a component at a time, because that's how the spec
  describes the matching algorithm.
* paths, however, are treated as arbitrary strings (except by normalizing the empty path into "/").
* path edges are stored in a compact binary search tree
* host edges are stored in a hash map

You may also notice that there is no use of threads in rust-content-security-policy at all.
However, the parsed tree does implement `Send` and `Sync`, so a document with many URLs to check
can use threads that way, if it proves advantageous.

*/

use check::search;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::cmp::Ordering::*;
use std::collections::HashMap;
use std::mem;

use super::Resource;

#[derive(Debug)]
pub struct HostNode<'a> {
    terminal: PathNode<'a>,
    wildcard: PathNode<'a>,
    children: HashMap<&'a str, HostNode<'a>>,
}

impl<'a> HostNode<'a> {
    pub fn new() -> Self {
        HostNode {
            terminal: PathNode::new(),
            wildcard: PathNode::new(),
            children: HashMap::new(),
        }
    }
    pub fn arrange(&mut self) {
        self.terminal.arrange();
        self.wildcard.arrange();
        for (_, child) in self.children.iter_mut() {
            child.arrange();
        }
    }
    fn check_<'b, I: Iterator<Item=&'b str>>(&self, resource: Resource, parts: &'b mut I, path: &'b str) -> bool {
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
    pub fn check<'b>(&self, resource: Resource, host: &'b str, path: &'b str) -> bool {
        self.check_(resource, &mut host.split('.').rev(), path)
    }
    pub fn insert(&mut self, resource: Resource, host: &'a str, path: &'a str) {
        self.insert_(resource, &mut host.split('.').rev(), path)
    }
    fn insert_<'b, I: Iterator<Item=&'a str>>(&mut self, resource: Resource, parts: &'b mut I, path: &'a str) {
        if let Some(part) = parts.next() {
            if part == "*" {
                self.wildcard.insert(resource, path)
            } else {
                self.children.entry(part)
                    .or_insert_with(|| HostNode::new())
                    .insert_(resource, parts, path)
            }
        } else {
            self.terminal.insert(resource, path)
        }
    }
}

#[derive(Debug)]
pub struct PathNode<'a> {
    flags: PathNodeFlags,
    children: Vec<PathEdge<'a>>,
}

#[derive(Debug)]
pub struct PathEdge<'a> {
    prefix: &'a str,
    node: PathNode<'a>,
}

impl<'a> PathNode<'a> {
    pub fn new() -> Self {
        PathNode {
            flags: PathNodeFlags::empty(),
            children: Vec::new(),
        }
    }
    pub fn insert(&mut self, resource: Resource, mut path: &'a str) {
        if path.as_bytes().get(0) == Some(&b'/') {
            path = &path[1..];
        }
        self.insert_(resource, path);
    }
    fn insert_(&mut self, resource: Resource, path: &'a str) {
        let flag = resource.flag();
        if path == "" {
            self.flags |= flag;
            return;
        }
        for child in &mut self.children {
            debug_assert!(child.prefix.len() > 0);
            if path.len() >= child.prefix.len() {
                if path.starts_with(child.prefix) {
                    return child.node.insert_(resource, &path[child.prefix.len()..]);
                }
                for i in 1 .. child.prefix.len() {
                    let sub = &child.prefix[0..i];
                    if path.starts_with(sub) {
                        let internal_node = PathNode {
                            flags: PathNodeFlags::empty(),
                            children: Vec::new(),
                        };
                        let internal_edge = PathEdge {
                            node: internal_node,
                            prefix: sub,
                        };
                        let mut old_edge = mem::replace(child, internal_edge);
                        old_edge.prefix = &old_edge.prefix[i..];
                        child.node.children.push(old_edge);
                        let new_node = PathNode {
                            flags: flag,
                            children: Vec::new(),
                        };
                        let new_edge = PathEdge {
                            node: new_node,
                            prefix: &path[i..],
                        };
                        child.node.children.push(new_edge);
                        return;
                    }
                }
            } else {
                if child.prefix.starts_with(path) {
                    let new_child = PathNode {
                        flags: child.node.flags,
                        children: mem::replace(&mut child.node.children, Vec::new()),
                    };
                    let new_edge = PathEdge {
                        node: new_child,
                        prefix: &child.prefix[path.len()..],
                    };
                    child.prefix = path;
                    child.node = PathNode {
                        flags: flag,
                        children: vec![new_edge],
                    };
                    return;
                }
                for i in 1 .. path.len() {
                    let sub = &path[0..i];
                    if child.prefix.starts_with(sub) {
                        let internal_node = PathNode {
                            flags: PathNodeFlags::empty(),
                            children: Vec::new(),
                        };
                        let internal_edge = PathEdge {
                            node: internal_node,
                            prefix: sub,
                        };
                        let mut old_edge = mem::replace(child, internal_edge);
                        old_edge.prefix = &old_edge.prefix[i..];
                        child.node.children.push(old_edge);
                        let new_node = PathNode {
                            flags: flag,
                            children: Vec::new(),
                        };
                        let new_edge = PathEdge {
                            node: new_node,
                            prefix: &path[i..],
                        };
                        child.node.children.push(new_edge);
                        return;
                    }
                }
            }
        }
        let new_child = PathNode {
            flags: flag,
            children: Vec::new(),
        };
        let new_edge = PathEdge {
            node: new_child,
            prefix: path,
        };
        self.children.push(new_edge);
    }
    pub fn arrange(&mut self) {
        self.children.sort_by_key(|child| child.prefix);
        search::arrange(&mut self.children);
        for child in &mut self.children {
            child.node.arrange();
        }
    }
    fn check_<'b>(&self, resource: Resource, path: &'b str) -> bool {
        self.check_resource(resource)
        || search::find(&self.children[..], |child| {
            if path.starts_with(child.prefix) {
                Equal
            } else {
                child.prefix.cmp(path)
            }
        }).map(|child| child.node.check_(resource, &path[child.prefix.len()..]))
          .unwrap_or(false)
    }
    pub fn check<'b>(&self, resource: Resource, mut path: &'b str) -> bool {
        if path.as_bytes().get(0) == Some(&b'/') {
            path = &path[1..];
        }
        self.check_(resource, path)
    }
    fn check_resource(&self, resource: Resource) -> bool {
        self.flags.contains(resource.flag())
    }
}

impl Resource {
    fn flag(self) -> PathNodeFlags {
        match self {
            Resource::ChildSrc => PathNodeFlags::RESOURCE_CHILD_SRC,
            Resource::ConnectSrc => PathNodeFlags::RESOURCE_CONNECT_SRC,
            Resource::DefaultSrc => PathNodeFlags::RESOURCE_DEFAULT_SRC,
            Resource::FontSrc => PathNodeFlags::RESOURCE_FONT_SRC,
            Resource::FrameSrc => PathNodeFlags::RESOURCE_FRAME_SRC,
            Resource::ImgSrc => PathNodeFlags::RESOURCE_IMG_SRC,
            Resource::ManifestSrc => PathNodeFlags::RESOURCE_MANIFEST_SRC,
            Resource::MediaSrc => PathNodeFlags::RESOURCE_MEDIA_SRC,
            Resource::ObjectSrc => PathNodeFlags::RESOURCE_OBJECT_SRC,
            Resource::ScriptSrc => PathNodeFlags::RESOURCE_SCRIPT_SRC,
            Resource::StyleSrc => PathNodeFlags::RESOURCE_STYLE_SRC,
            Resource::WorkerSrc => PathNodeFlags::RESOURCE_WORKER_SRC,
            Resource::BaseUri => PathNodeFlags::RESOURCE_BASE_URI,
            Resource::FormAction => PathNodeFlags::RESOURCE_FORM_ACTION,
            Resource::FrameAncestors => PathNodeFlags::RESOURCE_FRAME_ANCESTORS,
        }
    }
}

// If PathNodeFlags is all-zero, then no permissions are granted
// This policy can be effectively dropped with no behavioral changes.
bitflags!{
    struct PathNodeFlags: u16 {
        const RESOURCE_CHILD_SRC       = 0b00000000_00000001;
        const RESOURCE_CONNECT_SRC     = 0b00000000_00000010;
        const RESOURCE_DEFAULT_SRC     = 0b00000000_00000100;
        const RESOURCE_FONT_SRC        = 0b00000000_00001000;
        const RESOURCE_FRAME_SRC       = 0b00000000_00010000;
        const RESOURCE_IMG_SRC         = 0b00000000_00100000;
        const RESOURCE_MANIFEST_SRC    = 0b00000000_01000000;
        const RESOURCE_MEDIA_SRC       = 0b00000000_10000000;
        const RESOURCE_OBJECT_SRC      = 0b00000001_00000000;
        const RESOURCE_SCRIPT_SRC      = 0b00000010_00000000;
        const RESOURCE_STYLE_SRC       = 0b00000100_00000000;
        const RESOURCE_WORKER_SRC      = 0b00001000_00000000;
        const RESOURCE_BASE_URI        = 0b00010000_00000000;
        const RESOURCE_FORM_ACTION     = 0b00100000_00000000;
        const RESOURCE_FRAME_ANCESTORS = 0b01000000_00000000;
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
                tree.arrange();
                println!("{:?}", tree);
                assert_eq!(tree.check(Resource::ScriptSrc, $find), $mode);
            }
        }
    }
    do_tree_test!{empty_simple, false, ""; }
    do_tree_test!{empty_text, false, "abc"; }
    do_tree_test!{empty_rooted, false, "/abc"; }
    do_tree_test!{empty_root, false, "/"; }
    do_tree_test!{root_match, true, "/"; "/"}
    do_tree_test!{root_equiv_match, true, ""; "/"}
    do_tree_test!{root_equiv2_match, true, "/"; ""}
    do_tree_test!{rooted_match, true, "/test"; "/"}
    do_tree_test!{rooted_nomatch, false, "/test"; "/xxx"}
    do_tree_test!{rooted_nomatch_prefix, false, "/"; "/xxx"}
    do_tree_test!{two_nomatch, false, "/xxx"; "/test", "/test2"}

    #[test]
    fn prefixed_mixed_match() {
        let mut tree = PathNode::new();
        tree.insert(Resource::StyleSrc, "/a");
        tree.insert(Resource::StyleSrc, "/ab");
        tree.insert(Resource::StyleSrc, "/abc");
        tree.insert(Resource::ScriptSrc, "/abc");
        tree.arrange();
        println!("{:?}", tree);
        assert_eq!(tree.check(Resource::ScriptSrc, "/a"), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "/ab"), false);
        assert_eq!(tree.check(Resource::StyleSrc, "/abc"), true);
    }

    #[test]
    fn prefixed_mixed_one_match() {
        let mut tree = PathNode::new();
        tree.insert(Resource::StyleSrc, "/a");
        tree.insert(Resource::StyleSrc, "/ab");
        tree.insert(Resource::ScriptSrc, "/abc");
        tree.arrange();
        println!("{:?}", tree);
        assert_eq!(tree.check(Resource::ScriptSrc, "/a"), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "/ab"), false);
        assert_eq!(tree.check(Resource::StyleSrc, "/abc"), true);
    }

    #[test]
    fn prefixed_mixed_parent_match() {
        let mut tree = PathNode::new();
        tree.insert(Resource::StyleSrc, "/a");
        tree.insert(Resource::ScriptSrc, "/ab");
        tree.insert(Resource::ScriptSrc, "/abc");
        tree.arrange();
        println!("{:?}", tree);
        assert_eq!(tree.check(Resource::StyleSrc, "/a"), true);
        assert_eq!(tree.check(Resource::StyleSrc, "/ab"), true);
        assert_eq!(tree.check(Resource::StyleSrc, "/abc"), true);
    }

    #[test]
    fn host_tree_empty() {
        let mut tree = HostNode::new();
        tree.arrange();
        println!("{:?}", tree);
        assert_eq!(tree.check(Resource::ScriptSrc, "", ""), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com", "script"), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com", "script.js"), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "cdn.google.com", "script.js"), false);
    }

    #[test]
    fn host_tree_basic() {
        let mut tree = HostNode::new();
        tree.insert(Resource::ScriptSrc, "google.com", "script");
        tree.arrange();
        println!("{:?}", tree);
        assert_eq!(tree.check(Resource::ScriptSrc, "", ""), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com", "script"), true);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com", "script.js"), true);
        assert_eq!(tree.check(Resource::ScriptSrc, "cdn.google.com", "script.js"), false);
    }

    #[test]
    fn host_tree_wildcard() {
        let mut tree = HostNode::new();
        tree.insert(Resource::ScriptSrc, "*.google.com", "script");
        tree.arrange();
        println!("{:?}", tree);
        assert_eq!(tree.check(Resource::ScriptSrc, "", ""), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com", "script"), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com", "script.js"), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "cdn.google.com", "script.js"), true);
    }

    #[test]
    fn host_tree_mixed() {
        let mut tree = HostNode::new();
        tree.insert(Resource::ScriptSrc, "google.com", "script");
        tree.insert(Resource::ScriptSrc, "*.google.com", "script");
        tree.arrange();
        println!("{:?}", tree);
        assert_eq!(tree.check(Resource::ScriptSrc, "", ""), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com", "script"), true);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com", "script.js"), true);
        assert_eq!(tree.check(Resource::ScriptSrc, "cdn.google.com", "script.js"), true);
    }

    #[test]
    fn host_tree_mixed_scheme() {
        let mut tree = HostNode::new();
        tree.insert(Resource::StyleSrc, "google.com", "script");
        tree.insert(Resource::ScriptSrc, "*.google.com", "script");
        tree.arrange();
        println!("{:?}", tree);
        assert_eq!(tree.check(Resource::ScriptSrc, "", ""), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com", "script"), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "google.com", "script.js"), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "cdn.google.com", "script.js"), true);
    }

    #[test]
    fn host_tree_fallback_after_wildcard() {
        let mut tree = HostNode::new();
        tree.insert(Resource::ScriptSrc, "*.google.com", "style");
        tree.insert(Resource::ScriptSrc, "cdn.google.com", "script");
        tree.arrange();
        println!("{:?}", tree);
        assert_eq!(tree.check(Resource::ScriptSrc, "", ""), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "users.google.com", "style"), true);
        assert_eq!(tree.check(Resource::ScriptSrc, "users.google.com", "script"), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "cdn.google.com", "style"), true);
        assert_eq!(tree.check(Resource::ScriptSrc, "cdn.google.com", "script"), true);
    }

    #[test]
    fn host_tree_mixed_resource_type() {
        let mut tree = HostNode::new();
        tree.insert(Resource::StyleSrc, "*.google.com", "style");
        tree.insert(Resource::ScriptSrc, "cdn.google.com", "script");
        tree.arrange();
        println!("{:?}", tree);
        assert_eq!(tree.check(Resource::StyleSrc, "users.google.com", "style"), true);
        assert_eq!(tree.check(Resource::ScriptSrc, "users.google.com", "style"), false);
        assert_eq!(tree.check(Resource::StyleSrc, "cdn.google.com", "script"), false);
        assert_eq!(tree.check(Resource::ScriptSrc, "cdn.google.com", "script"), true);
    }
}