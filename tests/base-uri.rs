extern crate content_security_policy;
use content_security_policy::*;
#[test]
fn base_uri_test_allow() {
    let csp_list = CspList::parse("base-uri https://www.notriddle.com", PolicySource::Header, PolicyDisposition::Enforce);
    let (check_result, _) = csp_list.is_base_allowed_for_document(&Url::parse("https://www.notriddle.com").unwrap(), &Origin::new_opaque());
    assert_eq!(check_result, CheckResult::Allowed);
}
#[test]
fn base_uri_test_blocked() {
    let csp_list = CspList::parse("base-uri https://www.example.com", PolicySource::Header, PolicyDisposition::Enforce);
    let (check_result, _) = csp_list.is_base_allowed_for_document(&Url::parse("https://www.notriddle.com").unwrap(), &Origin::new_opaque());
    assert_eq!(check_result, CheckResult::Blocked);
}