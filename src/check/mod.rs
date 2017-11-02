pub mod tree;
pub mod search;

use std::ascii::AsciiExt;
use std::collections::HashMap;
use url::Url;

/**

A single Content-Security-Policy is a set of directives plus the resource to report failures to.

This data structure does not store the origin; it needs to be provided to every method that uses it.
There is a wrapper, at the top level of the crate, that stores it along with a vector of CSPs.
The vector of CSPs is why we don't want to store a copy of the origin for each one.

*/
pub struct Policy<'a> {
    host_any_network_scheme: tree::HostNode<'a>,
    host_by_port: HashMap<Port, tree::HostNode<'a>>,
    host_by_scheme: HashMap<(String, Port), tree::HostNode<'a>>,
    specified_resources: ResourceFlags,
}

impl<'a> Policy<'a> {
    pub fn new() -> Self {
        Default::default()
    }
    // https://www.w3.org/TR/CSP/#match-url-to-source-expression
    pub fn insert_wildcard(&mut self, origin: Origin<'a>, resource: Resource) {
        self.specified_resources |= resource.flag();
        self.host_any_network_scheme.insert(resource, b"*", b"");
        if let Origin::Network{scheme: scheme, ..} = origin {
            if !Self::is_network_scheme(scheme) {
                self.host_by_scheme.entry((scheme.to_ascii_lowercase(), Port::Wildcard))
                    .or_insert_with(tree::HostNode::new)
                    .insert(resource, b"*", b"")
            }
        }
    }
    pub fn insert_self(&mut self, origin: Origin<'a>, resource: Resource) {
        self.specified_resources |= resource.flag();
        if let Origin::Network{scheme: scheme, host: host, port_number: port_number} = origin {
            self.host_by_scheme.entry((scheme.to_ascii_lowercase(), Port::Number(port_number)))
                .or_insert_with(tree::HostNode::new)
                .insert(resource, host.as_bytes(), b"");
            if scheme.eq_ignore_ascii_case("http") {
                let port = if port_number == 80 { Port::Wildcard } else { Port::Number(443) };
                self.host_by_scheme.entry(("https".to_owned(), port))
                    .or_insert_with(tree::HostNode::new)
                    .insert(resource, host.as_bytes(), b"");
            }
        }
    }
    pub fn insert_host(&mut self, origin: Origin<'a>, resource: Resource, scheme: Option<&'a str>, port: Option<Port>, host: &'a str, path: &'a str) {
        self.specified_resources |= resource.flag();
        if let Some(scheme) = scheme {
            let port = port.unwrap_or_else(|| Self::default_port(scheme));
            self.host_by_scheme.entry((scheme.to_ascii_lowercase(), port))
                .or_insert_with(tree::HostNode::new)
                .insert(resource, host.as_bytes(), path.as_bytes());
            if scheme.eq_ignore_ascii_case("http") {
                let port = if port == Port::Number(80) { Port::Number(443) } else { port };
                self.host_by_scheme.entry(("https".to_owned(), port))
                    .or_insert_with(tree::HostNode::new)
                    .insert(resource, host.as_bytes(), b"");
            }
        } else if let Some(port) = port {
            self.host_by_port.entry(port)
                .or_insert_with(tree::HostNode::new)
                .insert(resource, host.as_bytes(), path.as_bytes());
        } else {
            self.host_any_network_scheme.insert(resource, host.as_bytes(), path.as_bytes())
        }
    }
    pub fn check_url(&self, resource: Resource, url: &Url, is_redirect: bool) -> Option<bool> {
        if is_redirect { unimplemented!() };
        if !self.specified_resources.contains(resource.flag()) { return None };
        let scheme = url.scheme();
        let host = match url.host() {
            Some(::url::Host::Domain(host)) => host.as_bytes(),
            _ => return Some(false),
        };
        let port = url.port().map(Port::Number).unwrap_or_else(|| Self::default_port(scheme));
        let path = url.path().as_bytes();
        Some(
            (Self::is_network_scheme(scheme) && self.host_any_network_scheme.check(resource, host, path))
            || self.host_by_scheme.get(&(scheme.to_ascii_lowercase(), port))
                .map(|h| h.check(resource, host, path))
                .unwrap_or(false)
            || self.host_by_port.get(&port)
                .map(|h| h.check(resource, host, path))
                .unwrap_or(false)
        )
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

impl<'a> Default for Policy<'a> {
    fn default() -> Self {
        Policy {
            host_any_network_scheme: tree::HostNode::new(),
            host_by_port: HashMap::new(),
            host_by_scheme: HashMap::new(),
            specified_resources: ResourceFlags::empty(),
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

impl Resource {
    fn flag(self) -> ResourceFlags {
        match self {
            Resource::ChildSrc => ResourceFlags::CHILD_SRC,
            Resource::ConnectSrc => ResourceFlags::CONNECT_SRC,
            Resource::DefaultSrc => ResourceFlags::DEFAULT_SRC,
            Resource::FontSrc => ResourceFlags::FONT_SRC,
            Resource::FrameSrc => ResourceFlags::FRAME_SRC,
            Resource::ImgSrc => ResourceFlags::IMG_SRC,
            Resource::ManifestSrc => ResourceFlags::MANIFEST_SRC,
            Resource::MediaSrc => ResourceFlags::MEDIA_SRC,
            Resource::ObjectSrc => ResourceFlags::OBJECT_SRC,
            Resource::ScriptSrc => ResourceFlags::SCRIPT_SRC,
            Resource::StyleSrc => ResourceFlags::STYLE_SRC,
            Resource::WorkerSrc => ResourceFlags::WORKER_SRC,
            Resource::BaseUri => ResourceFlags::BASE_URI,
            Resource::FormAction => ResourceFlags::FORM_ACTION,
            Resource::FrameAncestors => ResourceFlags::FRAME_ANCESTORS,
        }
    }
}

// If ResourceFlags is all-zero, then no permissions are granted
// This policy can be effectively dropped with no behavioral changes.
bitflags!{
    struct ResourceFlags: u16 {
        const CHILD_SRC       = 0b00000000_00000001;
        const CONNECT_SRC     = 0b00000000_00000010;
        const DEFAULT_SRC     = 0b00000000_00000100;
        const FONT_SRC        = 0b00000000_00001000;
        const FRAME_SRC       = 0b00000000_00010000;
        const IMG_SRC         = 0b00000000_00100000;
        const MANIFEST_SRC    = 0b00000000_01000000;
        const MEDIA_SRC       = 0b00000000_10000000;
        const OBJECT_SRC      = 0b00000001_00000000;
        const SCRIPT_SRC      = 0b00000010_00000000;
        const STYLE_SRC       = 0b00000100_00000000;
        const WORKER_SRC      = 0b00001000_00000000;
        const BASE_URI        = 0b00010000_00000000;
        const FORM_ACTION     = 0b00100000_00000000;
        const FRAME_ANCESTORS = 0b01000000_00000000;
    }
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
