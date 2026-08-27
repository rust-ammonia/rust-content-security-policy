/*!
An opt-in registry of embedder-registered URL schemes, modelled on Chromium's
[`url::AddStandardScheme`][add-standard-scheme] scheme registry.

# Why this exists

The URL Standard gives every URL whose scheme is not [special] an [opaque
origin], and two opaque origins are never [same origin] — not even when the two
URLs are byte-for-byte identical. A user agent that serves its own content over
a scheme it invented (`tauri://`, `chrome-extension://`, `moz-extension://`,
`webui://`, …) therefore cannot express "the document's own origin" with
[`Origin`][url::Origin] alone, and `'self'` in a policy delivered with such a
document matches nothing at all.

Browsers resolve this by keeping a list of schemes that, although not special,
the embedder has declared to behave like network schemes for security purposes:
they have an authority, and two URLs sharing scheme, host and port are treated
as same origin. Chromium calls these *standard* schemes, registers them at
startup from [`ContentClient::AddAdditionalSchemes`][add-additional-schemes],
and gates its CSP host matching on [`GURL::IsStandard`][is-standard] — a URL
whose scheme is not standard reports an empty host and so matches no
host-bearing source expression, `'self'` included.

This module is that list. It is empty by default, so nothing changes for a
caller that does not opt in: a policy on a `custom://` document keeps matching
nothing, exactly as the specification says it should. An embedder that
registers a scheme is asserting that it — not the URL Standard — defines the
origin of URLs with that scheme, and that it supplies a matching
[`Origin::Tuple`][url::Origin] in [`Request::origin`](crate::Request::origin).

# Usage

Register each scheme once, during start-up, before any policy is checked:

```rust
use content_security_policy::scheme_registry;

scheme_registry::add_standard_scheme("tauri");
assert!(scheme_registry::is_standard_scheme("tauri"));
```

A `tauri://example.com` document may then use `'self'` to reach
`tauri://example.com/app.js`, and still nothing else: registration grants
same-scheme, same-host, same-port matching and no upgrade allowance, so
`'self'` on such a document never reaches `https://example.com/`.

# Differences from Chromium

* Chromium's registry is a plain global guarded by
  [`url::LockSchemeRegistries`][lock-scheme-registries], which exists because
  the underlying vectors are unsynchronised. This one is behind an
  [`RwLock`], so late registration is a logic error rather than a data race and
  no locking step is required.
* Chromium's built-in standard schemes (`http`, `https`, `ws`, `wss`, `ftp`,
  `file`) live in the same table. Here they do not: their origins are already
  defined by the URL Standard, and `'self'` matching for them is unaffected by
  this registry. Registering one is a no-op.
* Chromium records a [`SchemeType`][scheme-type] per scheme describing which
  authority components the scheme admits. Every scheme registered here is
  treated as Chromium's `SCHEME_WITH_HOST_AND_PORT`: host required, port
  optional, no default port.

[special]: https://url.spec.whatwg.org/#special-scheme
[opaque origin]: https://html.spec.whatwg.org/multipage/browsers.html#concept-origin-opaque
[same origin]: https://html.spec.whatwg.org/multipage/browsers.html#same-origin
[add-standard-scheme]: https://source.chromium.org/chromium/chromium/src/+/main:url/url_util.h;l=63
[add-additional-schemes]: https://source.chromium.org/chromium/chromium/src/+/main:content/public/common/content_client.h
[is-standard]: https://source.chromium.org/chromium/chromium/src/+/main:url/gurl.h
[lock-scheme-registries]: https://source.chromium.org/chromium/chromium/src/+/main:url/url_util.h
[scheme-type]: https://source.chromium.org/chromium/chromium/src/+/main:url/url_canon.h;l=257
*/

use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{PoisonError, RwLock};

/// A scheme this registry cannot say anything useful about: either the URL
/// Standard already defines its origin (it is
/// [special](https://url.spec.whatwg.org/#special-scheme)), or it has a default
/// port, which a tuple origin cannot distinguish from an absent one. Either way
/// registering it would store a value that could never be read, so
/// `add_standard_scheme` drops it rather than let it look effective.
fn registration_would_be_inert(scheme: &str) -> bool {
    matches!(scheme, "ftp" | "file" | "http" | "https" | "ws" | "wss")
        || crate::default_port(scheme).is_some()
}

/// Registration order is preserved, as Chromium's `GetStandardSchemes` does.
/// A browser registers a handful of schemes at most, so a linear scan costs
/// less than the hashing it would replace.
static STANDARD_SCHEMES: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(Vec::new()));

/// Whether `STANDARD_SCHEMES` has ever been written to. A caller that never
/// opts in is the common case and is on the matching path for every `'self'`
/// source expression, so let it answer `is_standard_scheme` with one atomic
/// load rather than taking the lock to look at an empty list.
static ANY_SCHEME_REGISTERED: AtomicBool = AtomicBool::new(false);

/// Declare that `scheme` names URLs whose origin the embedder defines by their
/// scheme, host and port, the way Chromium's
/// [`url::AddStandardScheme`](self#differences-from-chromium) does.
///
/// The scheme is lowercased, since [scheme comparison is ASCII
/// case-insensitive](https://www.w3.org/TR/CSP/#match-schemes). Registering the
/// same scheme twice does nothing, and so does registering one this registry
/// cannot speak for: a scheme the URL Standard already gives an origin, or one
/// with a default port that a tuple origin could not tell from an absent one.
///
/// Call this during start-up, before any policy is checked. Registering a
/// scheme once requests are already being matched against a policy will change
/// the outcome of later checks without changing earlier ones.
pub fn add_standard_scheme(scheme: &str) {
    let scheme = scheme.to_ascii_lowercase();
    if scheme.is_empty() || registration_would_be_inert(&scheme) {
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

/// Whether `scheme` was registered with [`add_standard_scheme`].
///
/// Unlike Chromium's `url::IsStandard`, this reports only what the embedder
/// registered: it is `false` for `http`, `https` and the other
/// [special](https://url.spec.whatwg.org/#special-scheme) schemes, whose
/// origins the URL Standard defines without any help from this registry.
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
    fn unregistered_and_special_schemes_are_not_standard() {
        add_standard_scheme("https");
        assert!(!is_standard_scheme("https"));
        assert!(!is_standard_scheme("registry-unit-test-never-registered"));
    }

    /// A scheme with a default port cannot be told apart from one without a
    /// port by a tuple origin, so registering it is refused rather than
    /// accepted and then quietly ignored at matching time.
    #[test]
    fn schemes_with_a_default_port_are_refused() {
        add_standard_scheme("gopher");
        assert!(!is_standard_scheme("gopher"));
    }
}
