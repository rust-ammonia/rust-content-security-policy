//! `'self'` matching when the user agent supplies a tuple origin for a URL
//! that the URL standard would give an opaque origin (a non-special scheme).
//!
//! Component comparison is opt-in: it applies only to schemes the embedder
//! registered with [`scheme_registry::add_standard_scheme`], mirroring the
//! standard-scheme registry Chromium gates its own CSP host matching on.
extern crate content_security_policy;
use content_security_policy::url::Host;
use content_security_policy::*;

/// Registered by every test that expects component comparison to apply.
/// Registration is idempotent, so tests may run in any order, in parallel.
const REGISTERED: &str = "custom";

/// Never passed to `add_standard_scheme` anywhere in this binary.
const UNREGISTERED: &str = "unregistered";

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

/// A scheme the embedder never registered keeps the behaviour the URL standard
/// prescribes: its origin is opaque, so `'self'` matches nothing, however
/// exactly the components line up.
#[test]
fn self_does_not_match_unregistered_scheme() {
    assert!(!scheme_registry::is_standard_scheme(UNREGISTERED));
    assert_eq!(
        check(
            "default-src 'self'",
            tuple(UNREGISTERED, "example.com", 0),
            &format!("{UNREGISTERED}://example.com/app.js")
        ),
        CheckResult::Blocked
    );
}

/// Registering one scheme says nothing about any other.
#[test]
fn registering_one_scheme_does_not_admit_another() {
    scheme_registry::add_standard_scheme(REGISTERED);
    assert_eq!(
        check(
            "default-src 'self'",
            tuple("also-unregistered", "example.com", 0),
            "also-unregistered://example.com/app.js"
        ),
        CheckResult::Blocked
    );
}

#[test]
fn registration_is_visible_through_the_registry() {
    scheme_registry::add_standard_scheme("Custom");
    assert!(scheme_registry::is_standard_scheme(REGISTERED));
    assert!(scheme_registry::standard_schemes().contains(&REGISTERED.to_owned()));
}

#[test]
fn self_matches_same_custom_scheme_origin() {
    scheme_registry::add_standard_scheme(REGISTERED);
    assert_eq!(
        check(
            "default-src 'self'",
            tuple(REGISTERED, "example.com", 0),
            "custom://example.com/app.js"
        ),
        CheckResult::Allowed
    );
}

#[test]
fn self_matches_same_custom_scheme_origin_with_port() {
    scheme_registry::add_standard_scheme(REGISTERED);
    assert_eq!(
        check(
            "default-src 'self'",
            tuple(REGISTERED, "example.com", 1234),
            "custom://example.com:1234/app.js"
        ),
        CheckResult::Allowed
    );
}

#[test]
fn self_does_not_match_other_custom_scheme_host() {
    scheme_registry::add_standard_scheme(REGISTERED);
    assert_eq!(
        check(
            "default-src 'self'",
            tuple(REGISTERED, "example.com", 0),
            "custom://evil.example/app.js"
        ),
        CheckResult::Blocked
    );
}

#[test]
fn self_does_not_match_other_custom_scheme_port() {
    scheme_registry::add_standard_scheme(REGISTERED);
    assert_eq!(
        check(
            "default-src 'self'",
            tuple(REGISTERED, "example.com", 1234),
            "custom://example.com:9999/app.js"
        ),
        CheckResult::Blocked
    );
}

/// The http -> https/wss upgrade allowance must not extend to a custom scheme.
#[test]
fn self_on_custom_scheme_does_not_match_network_schemes() {
    scheme_registry::add_standard_scheme(REGISTERED);
    for url in [
        "https://example.com/x.js",
        "wss://example.com/sock",
        "http://example.com/x.js",
        "ws://example.com/sock",
        "ftp://example.com/x.js",
    ] {
        assert_eq!(
            check(
                "default-src 'self'",
                tuple(REGISTERED, "example.com", 0),
                url
            ),
            CheckResult::Blocked,
            "{url}"
        );
    }
}

/// ... and a network-scheme document must not reach a custom scheme through
/// `'self'` either.
#[test]
fn self_on_https_does_not_match_custom_scheme() {
    scheme_registry::add_standard_scheme(REGISTERED);
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

/// Registering a scheme changes `'self'` only. A host-source expression names
/// its own scheme, so it never needed the registry, and an unregistered scheme
/// still matches one.
#[test]
fn host_source_expressions_are_unaffected() {
    assert!(!scheme_registry::is_standard_scheme(UNREGISTERED));
    assert_eq!(
        check(
            &format!("default-src {UNREGISTERED}://example.com"),
            tuple(UNREGISTERED, "example.com", 0),
            &format!("{UNREGISTERED}://example.com/app.js")
        ),
        CheckResult::Allowed
    );
}
