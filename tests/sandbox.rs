extern crate content_security_policy;
use content_security_policy::*;
use content_security_policy::sandboxing_directive::SandboxingFlagSet;

#[test]
fn sandbox_document_flags() {
    let policy = CspList::parse("sandbox", PolicySource::Header, PolicyDisposition::Enforce);
    assert_eq!(Some(SandboxingFlagSet::all()), policy.get_sandboxing_flag_set_for_document());
    let policy = CspList::parse("sandbox allow-popups", PolicySource::Header, PolicyDisposition::Enforce);
    assert_eq!(Some(SandboxingFlagSet::all() ^ SandboxingFlagSet::SANDBOXED_AUXILIARY_NAVIGATION_BROWSING_CONTEXT_FLAG), policy.get_sandboxing_flag_set_for_document());
    let policy = CspList::parse("sandbox allow-forms", PolicySource::Header, PolicyDisposition::Enforce);
    assert_eq!(Some(SandboxingFlagSet::all() ^ SandboxingFlagSet::SANDBOXED_FORMS_BROWSING_CONTEXT_FLAG), policy.get_sandboxing_flag_set_for_document());
    let policy = CspList::parse("sandbox allow-downloads", PolicySource::Header, PolicyDisposition::Enforce);
    assert_eq!(Some(SandboxingFlagSet::all() ^ SandboxingFlagSet::SANDBOXED_DOWNLOADS_BROWSING_CONTEXT_FLAG), policy.get_sandboxing_flag_set_for_document());
    let policy = CspList::parse("sandbox; connect-src https://*.notriddle.com:443", PolicySource::Header, PolicyDisposition::Enforce);
    assert_eq!(Some(SandboxingFlagSet::all()), policy.get_sandboxing_flag_set_for_document());
    let policy = CspList::parse("sandbox allow-popups; connect-src https://*.notriddle.com:443", PolicySource::Header, PolicyDisposition::Enforce);
    assert_eq!(Some(SandboxingFlagSet::all() ^ SandboxingFlagSet::SANDBOXED_AUXILIARY_NAVIGATION_BROWSING_CONTEXT_FLAG), policy.get_sandboxing_flag_set_for_document());
    let policy = CspList::parse("connect-src https://*.notriddle.com:443", PolicySource::Header, PolicyDisposition::Enforce);
    assert_eq!(None, policy.get_sandboxing_flag_set_for_document());
}