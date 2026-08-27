/*!
An opt-in allowlist of schemes whose origin the embedder, rather than the URL
Standard, defines. Modelled on Chromium's standard-scheme registry
([`url::AddStandardScheme`](https://source.chromium.org/chromium/chromium/src/+/main:url/url_util.h;l=63)).

A URL whose scheme is not [special](https://url.spec.whatwg.org/#special-scheme)
has an opaque origin, and no two opaque origins are same origin, so `'self'` in
a policy delivered over a scheme the user agent invented (`tauri://`,
`moz-extension://`, ...) matches nothing at all. Registering the scheme makes
`'self'` compare scheme, host and port instead:

```rust
use content_security_policy::scheme_registry;

scheme_registry::add_standard_scheme("tauri");
assert!(scheme_registry::is_standard_scheme("tauri"));
```

The registry is empty by default, so a caller that does not opt in keeps the
behaviour the specification prescribes. Registering a scheme grants same-scheme,
same-host, same-port matching and no upgrade allowance: `'self'` on a
`tauri://example.com` document still cannot reach `https://example.com/`. The
embedder must also supply the matching [`Origin::Tuple`](crate::Origin) in
[`Request::origin`](crate::Request::origin).
*/

use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{PoisonError, RwLock};

static STANDARD_SCHEMES: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(Vec::new()));

/// Lets the common case - a caller that never opts in - answer
/// `is_standard_scheme` without taking the lock.
static ANY_SCHEME_REGISTERED: AtomicBool = AtomicBool::new(false);

/// Declare that `scheme` names URLs whose origin is their scheme, host and port.
///
/// Call this during start-up, before any policy is checked. Scheme comparison is
/// ASCII case-insensitive. Registering the same scheme twice does nothing, and
/// neither does registering one with a default port: a tuple origin cannot tell
/// that from an absent port, so `'self'` matching skips it either way.
pub fn add_standard_scheme(scheme: &str) {
    let scheme = scheme.to_ascii_lowercase();
    if scheme.is_empty() || crate::default_port(&scheme).is_some() {
        return;
    }
    let mut schemes = STANDARD_SCHEMES
        .write()
        .unwrap_or_else(PoisonError::into_inner);
    if !schemes.contains(&scheme) {
        schemes.push(scheme);
        ANY_SCHEME_REGISTERED.store(true, Ordering::Release);
    }
}

/// Whether `scheme` was registered with [`add_standard_scheme`]. Unlike
/// Chromium's `url::IsStandard`, this is `false` for special schemes.
pub fn is_standard_scheme(scheme: &str) -> bool {
    if !ANY_SCHEME_REGISTERED.load(Ordering::Acquire) {
        return false;
    }
    STANDARD_SCHEMES
        .read()
        .unwrap_or_else(PoisonError::into_inner)
        .iter()
        .any(|known| known.eq_ignore_ascii_case(scheme))
}

/// Every scheme registered with [`add_standard_scheme`], in registration order.
pub fn standard_schemes() -> Vec<String> {
    STANDARD_SCHEMES
        .read()
        .unwrap_or_else(PoisonError::into_inner)
        .clone()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn registration_is_case_insensitive_and_idempotent() {
        add_standard_scheme("Registry-Unit-Test");
        add_standard_scheme("registry-unit-test");
        assert!(is_standard_scheme("REGISTRY-UNIT-TEST"));
        assert_eq!(
            standard_schemes()
                .iter()
                .filter(|scheme| *scheme == "registry-unit-test")
                .count(),
            1
        );
    }

    #[test]
    fn unregistered_schemes_are_not_standard() {
        assert!(!is_standard_scheme("registry-unit-test-never-registered"));
    }

    #[test]
    fn schemes_with_a_default_port_are_refused() {
        for scheme in ["ftp", "http", "https", "ws", "wss", "gopher"] {
            add_standard_scheme(scheme);
            assert!(!is_standard_scheme(scheme), "{}", scheme);
        }
    }
}
