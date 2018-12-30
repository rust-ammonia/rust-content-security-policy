Parse and validate Web [Content-Security-Policy level 3](https://www.w3.org/TR/CSP/)

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

# Installation

To use `content-security-policy`, add it to your project's `Cargo.toml` file:

```toml
[dependencies]
content-security-policy = "0.0.1"
```
