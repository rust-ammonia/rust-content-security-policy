extern crate content_security_policy;
use content_security_policy::*;

macro_rules! test_should_request_be_blocked {
    ($((name: $name:ident, url: $url:expr, origin: $origin:expr, policy: $policy:expr, dest: $destination:tt, result: $result:tt)),*$(,)*) => {
        $(
            #[test]
            fn $name() {
                let csp_list = CspList::parse($policy, PolicySource::Header, PolicyDisposition::Enforce);
                let (check_result, _) = csp_list.should_request_be_blocked(&Request {
                    url: Url::parse($url).unwrap(),
                    origin: Url::parse($origin).unwrap().origin(),
                    redirect_count: 0,
                    destination: Destination::$destination,
                    initiator: Initiator::None,
                    nonce: String::new(),
                    integrity_metadata: String::new(),
                    parser_metadata: ParserMetadata::None,
                });
                assert_eq!(check_result, CheckResult::$result);
            }
        )*
    }
}

// all tests should have a name starting with pre_request_
test_should_request_be_blocked!{
    (   name: pre_request_wild_script_allow,
        url: "https://www.notriddle.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "script-src *.notriddle.com",
        dest: Script,
        result: Allowed),
    (   name: pre_request_wild_script_block,
        url: "https://www.evil.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "script-src *.notriddle.com",
        dest: Script,
        result: Blocked),
    (   name: pre_request_upgrade_script_allow,
        url: "https://www.notriddle.com/script.js",
        origin: "http://www.notriddle.com",
        policy: "script-src *.notriddle.com",
        dest: Script,
        result: Allowed),
    (   name: pre_request_upgrade_script_block,
        url: "https://www.evil.com/script.js",
        origin: "http://www.notriddle.com",
        policy: "script-src *.notriddle.com",
        dest: Script,
        result: Blocked),
    (   name: pre_request_exact_upgrade_script_allow,
        url: "https://www.notriddle.com/script.js",
        origin: "http://www.notriddle.com",
        policy: "script-src www.notriddle.com",
        dest: Script,
        result: Allowed),
    (   name: pre_request_exact_upgrade_script_block,
        url: "https://www.evil.com/script.js",
        origin: "http://www.notriddle.com",
        policy: "script-src www.notriddle.com",
        dest: Script,
        result: Blocked),
    (   name: pre_request_default_script_allow,
        url: "https://www.notriddle.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle.com",
        dest: Script,
        result: Allowed),
    (   name: pre_request_default_script_block,
        url: "https://www.evil.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Script,
        result: Blocked),
    (   name: pre_request_wild_style_allow,
        url: "https://www.notriddle.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "style-src *.notriddle.com",
        dest: Style,
        result: Allowed),
    (   name: pre_request_wild_style_block,
        url: "https://www.evil.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "style-src *.notriddle.com",
        dest: Style,
        result: Blocked),
}

macro_rules! test_should_elements_inline_type_behavior_be_blocked {
    ($((name: $name:ident, policy: $policy:expr, nonce: $nonce:expr, inline_check_type: $type_:tt, source: $source:expr, result: $result:tt)),*$(,)*) => {
        $(
            #[test]
            fn $name() {
                let csp_list = CspList::parse($policy, PolicySource::Header, PolicyDisposition::Enforce);
                let (check_result, _) = csp_list.should_elements_inline_type_behavior_be_blocked(
                    &Element { nonce: $nonce },
                    InlineCheckType::$type_,
                    $source,
                );
                assert_eq!(check_result, CheckResult::$result);
            }
        )*
    }
}

// all tests should have a name starting with inline_
test_should_elements_inline_type_behavior_be_blocked!{
    (   name: inline_blocked_script,
        policy: "script-src 'none'",
        nonce: None,
        inline_check_type: Script,
        source: "",
        result: Blocked),
    (   name: inline_allowed_script_check_style,
        policy: "script-src 'none'",
        nonce: None,
        inline_check_type: Style,
        source: "",
        result: Allowed),
}