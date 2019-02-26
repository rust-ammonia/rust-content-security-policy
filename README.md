# Parse and validate Web [Content-Security-Policy level 3](https://www.w3.org/TR/CSP/)

[![Crates.IO](https://img.shields.io/crates/v/content-security-policy.svg)](https://crates.rs/crates/content-security-policy)
![Requires rustc 1.24.0](https://img.shields.io/badge/rustc-1.24.0+-green.svg)

This function parses a CSP string into a data structure, and provides a bunch of functions you can call on it (basically all of the "hooks" defined in the CSP standard). It directly uses the `url` crate, but it's intentionally agnostic to your HTML parser and your networking stack, so there are a few things it doesn't do:

* While this library does directly use `rust-url`, it is intentionally not entangled with any particular networking stack, HTML parser, or DOM implementation.
* Rather than directly adding events to the event loop, like the spec says, it returns a `Vec<>` of objects that the library's users should push to the event loop. Just iterate over the vec, convert to your internal event representation, and push them to the event loop after calling the rust-content-security-policy function. Since the CSP specification never spins the event loop in the middle of any of its algorithms, that will be spec compliant anyway.
* Simple algorithms that don't operate on any part of the parsed CSP data structures, like [is element nonceable](https://www.w3.org/TR/CSP/#is-element-nonceable), probably won't ever be implemented by this library. It would take as much effort for a user to convert from the HTML parser's data structure into whatever rust-content-security-policy would accept as it would take for them to just implement the algorithm themselves.

# Installation

To use `content-security-policy`, add it to your project's `Cargo.toml` file:

```toml
[dependencies]
content-security-policy = "0.1.0"
```

# Example

```rust
extern crate content_security_policy;
use content_security_policy::*;
fn main() {
    let csp_list = CspList::parse("script-src *.notriddle.com", PolicySource::Header, PolicyDisposition::Enforce);
    let (check_result, _) = csp_list.should_request_be_blocked(&Request {
        url: Url::parse("https://www.notriddle.com/script.js").unwrap(),
        origin: Origin::Tuple("https".to_string(), url::Host::Domain("notriddle.com".to_owned()), 443),
        redirect_count: 0,
        destination: Destination::Script,
        initiator: Initiator::None,
        nonce: String::new(),
        integrity_metadata: String::new(),
        parser_metadata: ParserMetadata::None,
    });
    assert_eq!(check_result, CheckResult::Allowed);
    let (check_result, _) = csp_list.should_request_be_blocked(&Request {
        url: Url::parse("https://www.evil.example/script.js").unwrap(),
        origin: Origin::Tuple("https".to_string(), url::Host::Domain("notriddle.com".to_owned()), 443),
        redirect_count: 0,
        destination: Destination::Script,
        initiator: Initiator::None,
        nonce: String::new(),
        integrity_metadata: String::new(),
        parser_metadata: ParserMetadata::None,
    });
    assert_eq!(check_result, CheckResult::Blocked);
}
```
