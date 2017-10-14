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
https://example.com/ *.examplecdn.com/scripts/ *.examplecdn.com/script/ *.examplecdn.com/js/ cdn.framework.org
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
  (scheme: https)      |           \                 /===\
                    /========\ /=====\               | / |
                    | script | | js/ |               \===/
                    \========/ \=====/                 \________
                       |   |        \__________                 \
                       |   |                   \              (scheme: https, http)
                    /====\ /===\          (scheme: https, http)
                    | s/ | | / |
                    \====/ \===/
```

The "flags" thing at the end actually has a few other things besides the scheme,
but they're not really relevant to understanding the important concepts:

* domain names are flipped backwards, on the assumption that the TLD is duplicated
  way more often than the other end.
* domain names are processed a component at a time, because that's how the spec
  describes the matching algorithm.
* paths, however, are treated as arbitrary strings (except by normalizing the empty path into "/").

You may also notice that there is no use of threads in rust-content-security-policy at all.
However, the parsed tree does implement `Send` and `Sync`, so a document with many URLs to check
can use threads that way, if it proves advantageous.

*/

use std::collections::HashMap;

struct HostNode<'a> {
    terminal: PathNode<'a>,
    wildcard: PathNode<'a>,
    children: HashMap<&'a str, HostNode<'a>>,
}

impl<'a> HostNode<'a> {
    fn new() -> Self {
        HostNode {
            terminal: PathNode::new(),
            wildcard: PathNode::new(),
            children: HashMap::new(),
        }
    }
    fn check_<'b, I: Iterator<Item=&'b str>>(&self, parts: &'b mut I, path: &'b str) -> bool {
        if let Some(part) = parts.next() {
            if let Some(child) = self.children.get(part) {
                child.check_(parts, path)
            } else {
                self.wildcard.check(path)
            }
        } else {
            self.terminal.check(path)
        }
    }
    fn check<'b>(&self, host: &'b str, path: &'b str) -> bool {
        self.check_(&mut host.split('.').rev(), path)
    }
}

struct PathNode<'a> {
    flags: PathNodeFlags,
    children: HashMap<&'a str, PathNode<'a>>,
}

impl<'a> PathNode<'a> {
    fn new() -> Self {
        PathNode {
            flags: PathNodeFlags::empty(),
            children: HashMap::new(),
        }
    }
    fn check<'b>(&self, path: &'b str) -> bool {
        unimplemented!();
    }
}

// If PathNodeFlags is all-zero, then no permissions are granted
// This policy can be effectively dropped with no behavioral changes.
bitflags!{
    struct PathNodeFlags: u16 {
        // Non-`file` special URL schemes.
        // Since file URLs have no host, CSP cannot whitelist them.
        // TODO: handle non-special URL schemes (string interning).
        const PATH_SCHEME_FTP    = 0b00000000_00000001;
        const PATH_SCHEME_GOTHER = 0b00000000_00000010;
        const PATH_SCHEME_HTTP   = 0b00000000_00000100;
        const PATH_SCHEME_HTTPS  = 0b00000000_00001000;
        const PATH_SCHEME_WS     = 0b00000000_00010000;
        const PATH_SCHEME_WSS    = 0b00000000_00100000;
    }
}

#[cfg(test)]
mod test {
    use super::*;
}