extern crate content_security_policy;
use content_security_policy::*;

macro_rules! examples {
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

examples!{
    (   name: wild_script_allow,
        url: "https://www.notriddle.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "script-src *.notriddle.com",
        dest: Script,
        result: Allowed),
    (   name: wild_script_block,
        url: "https://www.evil.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "script-src *.notriddle.com",
        dest: Script,
        result: Blocked),
    (   name: upgrade_script_allow,
        url: "https://www.notriddle.com/script.js",
        origin: "http://www.notriddle.com",
        policy: "script-src *.notriddle.com",
        dest: Script,
        result: Allowed),
    (   name: upgrade_script_block,
        url: "https://www.evil.com/script.js",
        origin: "http://www.notriddle.com",
        policy: "script-src *.notriddle.com",
        dest: Script,
        result: Blocked),
    (   name: exact_upgrade_script_allow,
        url: "https://www.notriddle.com/script.js",
        origin: "http://www.notriddle.com",
        policy: "script-src www.notriddle.com",
        dest: Script,
        result: Allowed),
    (   name: exact_upgrade_script_block,
        url: "https://www.evil.com/script.js",
        origin: "http://www.notriddle.com",
        policy: "script-src www.notriddle.com",
        dest: Script,
        result: Blocked),
    (   name: default_script_allow,
        url: "https://www.notriddle.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle.com",
        dest: Script,
        result: Allowed),
    (   name: default_script_block,
        url: "https://www.evil.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Script,
        result: Blocked),
    (   name: wild_style_allow,
        url: "https://www.notriddle.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "style-src *.notriddle.com",
        dest: Style,
        result: Allowed),
    (   name: wild_style_block,
        url: "https://www.evil.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "style-src *.notriddle.com",
        dest: Style,
        result: Blocked),
}