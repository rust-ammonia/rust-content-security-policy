pub(crate) mod tree;
pub(crate) mod search;

use std::collections::HashMap;

/**

A single Content-Security-Policy is a set of directives plus the resource to report failures to.

*/
pub(crate) struct ContentSecurityPolicy<'a> {
    host_default_proto: tree::HostNode<'a>,
    host_custom_proto: HashMap<(String, u16), tree::HostNode<'a>>,
}
