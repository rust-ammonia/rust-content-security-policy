//! `'self'` matching when the user agent supplies a tuple origin for a URL
//! that the URL standard would give an opaque origin (a non-special scheme).
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
fn self_matches_same_custom_scheme_origin() {
    assert_eq!(
        check(
            "default-src 'self'",
            tuple("custom", "example.com", 0),
            "custom://example.com/app.js"
        ),
        CheckResult::Allowed
    );
}

#[test]
fn self_matches_same_custom_scheme_origin_with_port() {
    assert_eq!(
        check(
            "default-src 'self'",
            tuple("custom", "example.com", 1234),
            "custom://example.com:1234/app.js"
        ),
        CheckResult::Allowed
    );
}

#[test]
fn self_does_not_match_other_custom_scheme_host() {
    assert_eq!(
        check(
            "default-src 'self'",
            tuple("custom", "example.com", 0),
            "custom://evil.example/app.js"
        ),
        CheckResult::Blocked
    );
}

#[test]
fn self_does_not_match_other_custom_scheme_port() {
    assert_eq!(
        check(
            "default-src 'self'",
            tuple("custom", "example.com", 1234),
            "custom://example.com:9999/app.js"
        ),
        CheckResult::Blocked
    );
}

/// The http -> https/wss upgrade allowance must not extend to a custom scheme.
#[test]
fn self_on_custom_scheme_does_not_match_network_schemes() {
    for url in [
        "https://example.com/x.js",
        "wss://example.com/sock",
        "http://example.com/x.js",
        "ws://example.com/sock",
        "ftp://example.com/x.js",
    ] {
        assert_eq!(
            check("default-src 'self'", tuple("custom", "example.com", 0), url),
            CheckResult::Blocked,
            "{url}"
        );
    }
}

/// ... and a network-scheme document must not reach a custom scheme through
/// `'self'` either.
#[test]
fn self_on_https_does_not_match_custom_scheme() {
    assert_eq!(
        check(
            "default-src 'self'",
            tuple("https", "example.com", 443),
            "custom://example.com/x.js"
        ),
        CheckResult::Blocked
    );
}

#[test]
fn self_on_https_does_not_downgrade() {
    assert_eq!(
        check(
            "default-src 'self'",
            tuple("https", "example.com", 443),
            "http://example.com/x.js"
        ),
        CheckResult::Blocked
    );
}
