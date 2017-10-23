pub mod tree;
pub mod search;

use std::ascii::AsciiExt;
use std::collections::HashMap;

/**

A single Content-Security-Policy is a set of directives plus the resource to report failures to.

This data structure does not store the origin; it needs to be provided to every method that uses it.
There is a wrapper, at the top level of the crate, that stores it along with a vector of CSPs.
The vector of CSPs is why we don't want to store a copy of the origin for each one.

*/
pub struct Csp<'a> {
    host_any_network_scheme: tree::HostNode<'a>,
    host_by_port: HashMap<Port, tree::HostNode<'a>>,
    host_by_scheme: HashMap<(String, Port), tree::HostNode<'a>>,
}

impl<'a> Csp<'a> {
    pub fn new() -> Self {
        Default::default()
    }
    // https://www.w3.org/TR/CSP/#match-url-to-source-expression
    pub fn insert_wildcard(&mut self, origin: Origin, resource: Resource) {
        self.host_any_network_scheme.insert(resource, "*", "");
        if let Origin::Network{scheme: scheme, ..} = origin {
            if !Self::is_network_scheme(scheme) {
                self.host_by_scheme.entry((scheme.to_ascii_lowercase(), Port::Wildcard))
                    .or_insert_with(tree::HostNode::new)
                    .insert(resource, "*", "")
            }
        }
    }
    pub fn insert_host_parts(&mut self, origin: Origin, resource: Resource, scheme: Option<&'a str>, port: Option<Port>, host: &'a str, path: &'a str) {
        if let Some(scheme) = scheme {
            let port = port.unwrap_or_else(|| Self::default_port(scheme));
            self.host_by_scheme.entry((scheme.to_ascii_lowercase(), port))
                .or_insert_with(tree::HostNode::new)
                .insert(resource, host, path);
        } else if let Some(port) = port {
            self.host_by_port.entry(port)
                .or_insert_with(tree::HostNode::new)
                .insert(resource, host, path);
        } else {
            self.host_any_network_scheme.insert(resource, host, path)
        }
    }
    fn is_network_scheme(scheme: &'a str) -> bool {
        scheme.eq_ignore_ascii_case("http")
        || scheme.eq_ignore_ascii_case("ws")
        || scheme.eq_ignore_ascii_case("https")
        || scheme.eq_ignore_ascii_case("wss")
        || scheme.eq_ignore_ascii_case("ftp")
        || scheme.eq_ignore_ascii_case("gopher")
    }
    fn default_port(scheme: &'a str) -> Port {
        if scheme.eq_ignore_ascii_case("http") || scheme.eq_ignore_ascii_case("ws") {
            Port::Number(80)
        } else if scheme.eq_ignore_ascii_case("https") || scheme.eq_ignore_ascii_case("wss") {
            Port::Number(443)
        } else if scheme.eq_ignore_ascii_case("ftp") {
            Port::Number(21)
        } else if scheme.eq_ignore_ascii_case("gopher") {
            Port::Number(70)
        } else {
            Port::Wildcard
        }
    }
}

impl<'a> Default for Csp<'a> {
    fn default() -> Self {
        Csp {
            host_any_network_scheme: tree::HostNode::new(),
            host_by_port: HashMap::new(),
            host_by_scheme: HashMap::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Resource {
    ChildSrc,
    ConnectSrc,
    DefaultSrc,
    FontSrc,
    FrameSrc,
    ImgSrc,
    ManifestSrc,
    MediaSrc,
    ObjectSrc,
    ScriptSrc,
    StyleSrc,
    WorkerSrc,
    BaseUri,
    FormAction,
    FrameAncestors,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Port {
    Number(u16),
    Wildcard,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Origin<'a> {
    Network{
        scheme: &'a str,
        host: &'a str,
        port_number: u16,
    },
    Opaque,
}
