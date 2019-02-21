extern crate content_security_policy;
use content_security_policy::*;
#[test]
fn sandbox_test_block() {
    let csp_list = CspList::parse("sandbox", PolicySource::Header, PolicyDisposition::Enforce);
    let (check_result, _) = csp_list.should_response_to_request_be_blocked(
        &Request {
            url: Url::parse("https://www.notriddle.com").unwrap(),
            origin: Url::parse("https://www.notriddle.com").unwrap().origin(),
            redirect_count: 0,
            destination: Destination::Worker,
            initiator: Initiator::None,
            nonce: String::new(),
            integrity_metadata: String::new(),
            parser_metadata: ParserMetadata::None,
        },
        &Response {
            csp_list: csp_list.clone(),
        }
    );
    assert_eq!(check_result, CheckResult::Blocked);
}
#[test]
fn sandbox_test_block_with_allow_popups() {
    let csp_list = CspList::parse("sandbox allow-popups", PolicySource::Header, PolicyDisposition::Enforce);
    let (check_result, _) = csp_list.should_response_to_request_be_blocked(
        &Request {
            url: Url::parse("https://www.notriddle.com").unwrap(),
            origin: Url::parse("https://www.notriddle.com").unwrap().origin(),
            redirect_count: 0,
            destination: Destination::Worker,
            initiator: Initiator::None,
            nonce: String::new(),
            integrity_metadata: String::new(),
            parser_metadata: ParserMetadata::None,
        },
        &Response {
            csp_list: csp_list.clone(),
        }
    );
    assert_eq!(check_result, CheckResult::Blocked);
}
#[test]
fn sandbox_test_allow_with_no_directive() {
    let csp_list = CspList::parse("", PolicySource::Header, PolicyDisposition::Enforce);
    let (check_result, _) = csp_list.should_response_to_request_be_blocked(
        &Request {
            url: Url::parse("https://www.notriddle.com").unwrap(),
            origin: Url::parse("https://www.notriddle.com").unwrap().origin(),
            redirect_count: 0,
            destination: Destination::Worker,
            initiator: Initiator::None,
            nonce: String::new(),
            integrity_metadata: String::new(),
            parser_metadata: ParserMetadata::None,
        },
        &Response {
            csp_list: csp_list.clone(),
        }
    );
    assert_eq!(check_result, CheckResult::Allowed);
}

#[test]
fn sandbox_test_allow_images() {
    let csp_list = CspList::parse("sandbox", PolicySource::Header, PolicyDisposition::Enforce);
    let (check_result, _) = csp_list.should_response_to_request_be_blocked(
        &Request {
            url: Url::parse("https://www.notriddle.com").unwrap(),
            origin: Url::parse("https://www.notriddle.com").unwrap().origin(),
            redirect_count: 0,
            destination: Destination::Image,
            initiator: Initiator::None,
            nonce: String::new(),
            integrity_metadata: String::new(),
            parser_metadata: ParserMetadata::None,
        },
        &Response {
            csp_list: csp_list.clone(),
        }
    );
    assert_eq!(check_result, CheckResult::Allowed);
}
