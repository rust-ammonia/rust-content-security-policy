//! `'self'` matching across the http -> https/wss upgrade allowance, on the
//! default ports for the schemes involved.
extern crate content_security_policy;
use content_security_policy::url::Host;
use content_security_policy::*;

fn check(policy: &str, origin: Origin, url: &str) -> CheckResult {
    let csp_list = CspList::parse(policy, PolicySource::Header, PolicyDisposition::Enforce);
    let (check_result, _) = csp_list.should_request_be_blocked(&Request {
        url: Url::parse(url).unwrap(),
        current_url: Url::parse(url).unwrap(),
        origin,
        redirect_count: 0,
        destination: Destination::Script,
        initiator: Initiator::None,
        nonce: String::new(),
        integrity_metadata: String::new(),
        parser_metadata: ParserMetadata::None,
    });
    check_result
}

fn tuple(scheme: &str, host: &str, port: u16) -> Origin {
    Origin::Tuple(scheme.to_owned(), Host::Domain(host.to_owned()), port)
}

#[test]
fn self_upgrades_on_default_ports() {
    for (origin, url) in [
        (tuple("http", "example.com", 80), "https://example.com/x.js"),
        (
            tuple("http", "example.com", 80),
            "https://example.com:443/x.js",
        ),
        (tuple("http", "example.com", 80), "http://example.com/x.js"),
        (tuple("http", "example.com", 80), "ws://example.com/socket"),
        (tuple("http", "example.com", 80), "wss://example.com/socket"),
        (
            tuple("https", "example.com", 443),
            "https://example.com/x.js",
        ),
        (
            tuple("https", "example.com", 443),
            "wss://example.com/socket",
        ),
    ] {
        assert_eq!(
            check("default-src 'self'", origin, url),
            CheckResult::Allowed,
            "{url}"
        );
    }
}

#[test]
fn self_upgrades_on_explicit_ports() {
    assert_eq!(
        check(
            "default-src 'self'",
            tuple("http", "example.com", 8080),
            "https://example.com:8080/x.js"
        ),
        CheckResult::Allowed
    );
}

#[test]
fn self_does_not_downgrade() {
    for (origin, url) in [
        (
            tuple("https", "example.com", 443),
            "http://example.com/x.js",
        ),
        (
            tuple("https", "example.com", 443),
            "ws://example.com/socket",
        ),
    ] {
        assert_eq!(
            check("default-src 'self'", origin, url),
            CheckResult::Blocked,
            "{url}"
        );
    }
}

#[test]
fn self_does_not_cross_ports() {
    for (origin, url) in [
        (
            tuple("http", "example.com", 80),
            "https://example.com:8443/x.js",
        ),
        (
            tuple("http", "example.com", 8080),
            "https://example.com/x.js",
        ),
        (
            tuple("http", "example.com", 8080),
            "https://example.com:9090/x.js",
        ),
    ] {
        assert_eq!(
            check("default-src 'self'", origin, url),
            CheckResult::Blocked,
            "{url}"
        );
    }
}

#[test]
fn self_does_not_cross_hosts() {
    assert_eq!(
        check(
            "default-src 'self'",
            tuple("http", "example.com", 80),
            "https://evil.example/x.js"
        ),
        CheckResult::Blocked
    );
}
