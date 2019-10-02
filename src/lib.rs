/*!
Parse and validate Web [Content-Security-Policy level 3](https://www.w3.org/TR/CSP/)

# Example

```rust
extern crate content_security_policy;
use content_security_policy::*;
fn main() {
    let csp_list = CspList::parse("script-src *.notriddle.com", PolicySource::Header, PolicyDisposition::Enforce);
    let (check_result, _) = csp_list.should_request_be_blocked(&Request {
        url: Url::parse("https://www.notriddle.com/script.js").unwrap(),
        origin: Origin::Tuple("https".to_string(), url::Host::Domain("notriddle.com".to_owned()), 443),
        redirect_count: 0,
        destination: Destination::Script,
        initiator: Initiator::None,
        nonce: String::new(),
        integrity_metadata: String::new(),
        parser_metadata: ParserMetadata::None,
    });
    assert_eq!(check_result, CheckResult::Allowed);
    let (check_result, _) = csp_list.should_request_be_blocked(&Request {
        url: Url::parse("https://www.evil.example/script.js").unwrap(),
        origin: Origin::Tuple("https".to_string(), url::Host::Domain("notriddle.com".to_owned()), 443),
        redirect_count: 0,
        destination: Destination::Script,
        initiator: Initiator::None,
        nonce: String::new(),
        integrity_metadata: String::new(),
        parser_metadata: ParserMetadata::None,
    });
    assert_eq!(check_result, CheckResult::Blocked);
}
```
*/

#![forbid(unsafe_code)]

pub extern crate url;
pub extern crate percent_encoding;
extern crate regex;
use regex::Regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;
#[cfg(feature = "serde")]
extern crate serde;

pub mod text_util;
pub mod sandboxing_directive;

pub use url::{Origin, Url};
#[cfg(feature = "serde")] use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, Cow};
use std::fmt::{self, Display, Formatter};
use text_util::{
    strip_leading_and_trailing_ascii_whitespace,
    split_ascii_whitespace,
    split_commas,
    ascii_case_insensitive_match,
    collect_a_sequence_of_non_ascii_white_space_code_points,
};
use sandboxing_directive::{SandboxingFlagSet, parse_a_sandboxing_directive};
use MatchResult::Matches;
use MatchResult::DoesNotMatch;
use std::collections::HashSet;

fn scheme_is_network(scheme: &str) -> bool {
    scheme == "ftp" || scheme_is_httpx(scheme)
}

fn scheme_is_httpx(scheme: &str) -> bool {
    scheme == "http" || scheme == "https"
}

/**
A single parsed content security policy
*/
#[derive(Clone, Debug)]
pub struct Policy {
    pub directive_set: Vec<Directive>,
    pub disposition: PolicyDisposition,
    pub source: PolicySource,
}

impl Display for Policy {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for (i, directive) in self.directive_set.iter().enumerate() {
            if i != 0 {
                write!(f, ";")?;
            }
            <Directive as Display>::fmt(directive, f)?;
        }
        Ok(())
    }
}

impl Policy {
    pub fn is_valid(&self) -> bool {
        self.directive_set.iter().all(Directive::is_valid) &&
            self.directive_set.iter().map(|d| d.name.clone()).collect::<HashSet<_>>().len() == self.directive_set.len() &&
            !self.directive_set.is_empty()
    }
    pub fn parse(serialized: &str, source: PolicySource, disposition: PolicyDisposition) -> Policy {
        let mut policy = Policy {
            directive_set: Vec::new(),
            source, disposition,
        };
        // Rust's str::split corresponds to a WHATWG "strict split"
        for token in serialized.split(';') {
            let token = strip_leading_and_trailing_ascii_whitespace(token);
            if token.is_empty() { continue };
            let (directive_name, token) =
                collect_a_sequence_of_non_ascii_white_space_code_points(token);
            let mut directive_name = directive_name.to_owned();
            directive_name.make_ascii_lowercase();
            if policy.contains_a_directive_whose_name_is(&directive_name) {
                continue;
            }
            let directive_value = split_ascii_whitespace(token).map(String::from).collect();
            policy.directive_set.push(Directive {
                name: directive_name,
                value: directive_value,
            });
        }
        policy
    }
    pub fn contains_a_directive_whose_name_is(&self, directive_name: &str) -> bool {
        self.directive_set.iter().any(|d| d.name == directive_name)
    }
    pub fn does_request_violate_policy(&self, request: &Request) -> Violates {
        let mut violates = Violates::DoesNotViolate;
        for directive in &self.directive_set {
            let result = directive.pre_request_check(request, self);
            if result == CheckResult::Blocked {
                violates = Violates::Directive(directive.clone());
            }
        }
        violates
    }
}

#[derive(Clone, Debug)]
pub struct CspList(pub Vec<Policy>);

impl Display for CspList {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for (i, directive) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ",")?;
            }
            <Policy as Display>::fmt(directive, f)?;
        }
        Ok(())
    }
}

impl CspList {
    pub fn is_valid(&self) -> bool {
        self.0.iter().all(Policy::is_valid)
    }
    pub fn contains_a_header_delivered_content_security_policy(&self) -> bool {
        self.0.iter().any(|policy| policy.source == PolicySource::Header)
    }
    pub fn parse(list: &str, source: PolicySource, disposition: PolicyDisposition) -> CspList {
        let mut policies = Vec::new();
        for token in split_commas(list) {
            let policy = Policy::parse(token, source, disposition);
            if policy.directive_set.is_empty() { continue };
            policies.push(policy)
        }
        CspList(policies)
    }
    pub fn append(&mut self, mut other: CspList) {
        self.0.append(&mut other.0)
    }
    pub fn push(&mut self, policy: Policy) {
        self.0.push(policy)
    }
    /**
    Given a request, this algorithm reports violations based on client’s "report only" policies.
    */
    pub fn report_violations_for_request(&self, request: &Request)
        -> Vec<Violation> {
        let mut violations = Vec::new();
        for policy in &self.0 {
            if policy.disposition == PolicyDisposition::Enforce { continue };
            let violates = policy.does_request_violate_policy(request);
            if let Violates::Directive(directive) = violates {
                let resource = ViolationResource::Url(request.url.clone());
                violations.push(Violation { resource, directive });
            }
        }
        violations
    }
    /**
    Given a request, this algorithm returns Blocked or Allowed and reports violations based on
    request’s client’s Content Security Policy.
    */
    pub fn should_request_be_blocked(&self, request: &Request) -> (CheckResult, Vec<Violation>) {
        let mut result = CheckResult::Allowed;
        let mut violations = Vec::new();
        for policy in &self.0 {
            if policy.disposition == PolicyDisposition::Report { continue };
            let violates = policy.does_request_violate_policy(request);
            if let Violates::Directive(directive) = violates {
                result = CheckResult::Blocked;
                let resource = ViolationResource::Url(request.url.clone());
                violations.push(Violation { resource, directive });
            }
        }
        (result, violations)
    }
    /**
    Given a response and a request, this algorithm returns Blocked or Allowed, and reports
    violations based on request’s client’s Content Security Policy.
    */
    pub fn should_response_to_request_be_blocked(&self, request: &Request, response: &Response)
        -> (CheckResult, Vec<Violation>) {
        let mut result = CheckResult::Allowed;
        let mut violations = Vec::new();
        for policy in &self.0 {
            for directive in &policy.directive_set {
                if directive.post_request_check(request, response, policy) == CheckResult::Blocked {
                    violations.push(Violation {
                        resource: ViolationResource::Url(request.url.clone()),
                        directive: directive.clone(),
                    });
                    if policy.disposition == PolicyDisposition::Enforce {
                        result = CheckResult::Blocked;
                    }
                }
            }
        }
        for policy in &response.csp_list.0 {
            for directive in &policy.directive_set {
                if directive.response_check(request, response, policy) == CheckResult::Blocked {
                    violations.push(Violation {
                        resource: ViolationResource::Url(request.url.clone()),
                        directive: directive.clone(),
                    });
                    if policy.disposition == PolicyDisposition::Enforce {
                        result = CheckResult::Blocked;
                    }
                }
            }
        }
        (result, violations)
    }
    pub fn should_elements_inline_type_behavior_be_blocked(&self, element: &Element, type_: InlineCheckType, source: &str) -> (CheckResult, Vec<Violation>) {
        use CheckResult::*;
        let mut result = Allowed;
        let mut violations = Vec::new();
        for policy in &self.0 {
            for directive in &policy.directive_set {
                if directive.inline_check(element, type_, policy, source) == Allowed {
                    continue;
                }
                let report_sample = directive.value.iter().filter(|t| &t[..] == "'report-sample'").next().is_some();
                let violation = Violation {
                    resource: ViolationResource::Inline{ report_sample },
                    directive: directive.clone(),
                };
                violations.push(violation);
                if policy.disposition == PolicyDisposition::Enforce {
                    result = Blocked;
                }
            }
        }
        (result, violations)
    }
}

#[derive(Clone, Debug)]
pub struct Element<'a> {
    /// When there is no nonce, populate this member with `None`.
    ///
    /// When the element is not [nonceable], also populate it with `None`.
    ///
    /// [nonceable]: https://www.w3.org/TR/CSP/#is-element-nonceable
    pub nonce: Option<Cow<'a, str>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InlineCheckType {
    Script,
    ScriptAttribute,
    Style,
    StyleAttribute,
    Navigation,
}

/**
request to be validated
*/
#[derive(Clone, Debug)]
pub struct Request {
    pub url: Url,
    pub origin: Origin,
    pub redirect_count: u32,
    pub destination: Destination,
    pub initiator: Initiator,
    pub nonce: String,
    pub integrity_metadata: String,
    pub parser_metadata: ParserMetadata,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParserMetadata {
    ParserInserted,
    NotParserInserted,
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Initiator {
    Prefetch,
    Prerender,
    Fetch,
    None,
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Destination {
    None,
    Audio,
    AudioWorklet,
    Document,
    Embed,
    Font,
    Image,
    Manifest,
    Object,
    PaintWorklet,
    Report,
    Script,
    ServiceWorker,
    SharedWorker,
    Style,
    Track,
    Video,
    Worker,
    Xslt,
}

impl Destination {
    pub fn is_script_like(self) -> bool {
        use Destination::*;
        match self {
            AudioWorklet | PaintWorklet | Script | ServiceWorker | SharedWorker | Worker | Xslt => true,
            _ => false
        }
    }
}

/**
response to be validated
*/
#[derive(Clone, Debug)]
pub struct Response {
    pub csp_list: CspList,
    pub url: Url,
    pub redirect_count: u32,
}

/**
violation information
*/
#[derive(Clone, Debug)]
pub struct Violation {
    pub resource: ViolationResource,
    pub directive: Directive,
}

#[derive(Clone, Debug)]
pub enum ViolationResource {
    Url(Url),
    Inline {
        report_sample: bool,
    },
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CheckResult {
    Allowed,
    Blocked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Violates {
    DoesNotViolate,
    Directive(Directive),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicyDisposition {
    Enforce,
    Report,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicySource {
    Header,
    Meta,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Directive {
    name: String,
    value: Vec<String>,
}

impl Display for Directive {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        <str as Display>::fmt(&self.name[..], f)?;
        write!(f, " ")?;
        for token in &self.value {
            <str as Display>::fmt(&token[..], f)?;
            write!(f, " ")?;
        }
        Ok(())
    }
}

impl Directive {
    pub fn is_valid(&self) -> bool {
        DIRECTIVE_NAME_GRAMMAR.is_match(&self.name) &&
            self.value.iter().all(|t| DIRECTIVE_VALUE_TOKEN_GRAMMAR.is_match(&t[..]))
    }
    pub fn pre_request_check(&self, request: &Request, policy: &Policy) -> CheckResult {
        use CheckResult::*;
        match &self.name[..] {
            "child-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "child-src", policy) {
                    return Allowed;
                }
                (Directive {
                    name: String::from(name),
                    value: self.value.clone(),
                }).pre_request_check(request, policy)
            },
            "connect-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "connect-src", policy) {
                    return Allowed;
                }
                if SourceList(&self.value[..]).does_request_match_source_list(request) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "default-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "default-src", policy) {
                    return Allowed;
                }
                (Directive {
                    name: String::from(name),
                    value: self.value.clone(),
                }).pre_request_check(request, policy)
            }
            "font-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "font-src", policy) {
                    return Allowed;
                }
                if SourceList(&self.value[..]).does_request_match_source_list(request) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "frame-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "frame-src", policy) {
                    return Allowed;
                }
                if SourceList(&self.value[..]).does_request_match_source_list(request) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "img-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "img-src", policy) {
                    return Allowed;
                }
                if SourceList(&self.value[..]).does_request_match_source_list(request) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "manifest-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "manifest-src", policy) {
                    return Allowed;
                }
                if SourceList(&self.value[..]).does_request_match_source_list(request) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "media-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "media-src", policy) {
                    return Allowed;
                }
                if SourceList(&self.value[..]).does_request_match_source_list(request) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "prefetch-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "prefetch-src", policy) {
                    return Allowed;
                }
                if SourceList(&self.value[..]).does_request_match_source_list(request) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "object-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "object-src", policy) {
                    return Allowed;
                }
                if SourceList(&self.value[..]).does_request_match_source_list(request) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "script-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "script-src", policy) {
                    return Allowed;
                }
                script_directives_prerequest_check(request, self)
            }
            "script-src-elem" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "script-src-elem", policy) {
                    return Allowed;
                }
                script_directives_prerequest_check(request, self)
            }
            "style-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "style-src", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_nonce_match_source_list(&request.nonce) == Matches {
                    return Allowed;
                }
                if source_list.does_request_match_source_list(request) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "style-src-elem" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "style-src-elem", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_nonce_match_source_list(&request.nonce) == Matches {
                    return Allowed;
                }
                if source_list.does_request_match_source_list(request) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "worker-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "worker-src", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_request_match_source_list(request) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            _ => Allowed,
        }
    }
    pub fn post_request_check(&self, request: &Request, response: &Response, policy: &Policy) -> CheckResult {
        use CheckResult::*;
        match &self.name[..] {
            "child-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "child-src", policy) {
                    return Allowed;
                }
                Directive {
                    name: name.to_owned(),
                    value: self.value.clone()
                }.post_request_check(request, response, policy)
            }
            "connect-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "connect-src", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_response_to_request_match_source_list(request, response) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "default-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "default-src", policy) {
                    return Allowed;
                }
                Directive {
                    name: name.to_owned(),
                    value: self.value.clone(),
                }.post_request_check(request, response, policy)
            }
            "font-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "font-src", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_response_to_request_match_source_list(request, response) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "frame-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "frame-src", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_response_to_request_match_source_list(request, response) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "img-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "img-src", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_response_to_request_match_source_list(request, response) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "manifest-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "manifest-src", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_response_to_request_match_source_list(request, response) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "media-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "media-src", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_response_to_request_match_source_list(request, response) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "prefetch-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "prefetch-src", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_response_to_request_match_source_list(request, response) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "object-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "object-src", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_response_to_request_match_source_list(request, response) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "script-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "script-src", policy) {
                    return Allowed;
                }
                script_directives_postrequest_check(request, response, self)
            }
            "script-src-elem" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "script-src-elem", policy) {
                    return Allowed;
                }
                script_directives_postrequest_check(request, response, self)
            }
            "style-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "style-src", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_nonce_match_source_list(&request.nonce) == Matches {
                    return Allowed;
                }
                if source_list.does_response_to_request_match_source_list(request, response) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "style-src-elem" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "style-src-elem", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_nonce_match_source_list(&request.nonce) == Matches {
                    return Allowed;
                }
                if source_list.does_response_to_request_match_source_list(request, response) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "worker-src" => {
                let name = get_the_effective_directive_for_request(request);
                if !should_fetch_directive_execute(name, "worker-src", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_response_to_request_match_source_list(request, response) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            _ => Allowed,
        }
    }
    pub fn response_check(&self, request: &Request, _response: &Response, policy: &Policy) -> CheckResult {
        use CheckResult::*;
        use Destination::*;
        use PolicyDisposition::*;
        match &self.name[..] {
            "sandbox" => {
                if policy.disposition != Enforce {
                    return Allowed;
                }
                match request.destination {
                    ServiceWorker | SharedWorker | Worker => {
                        let sandboxing = parse_a_sandboxing_directive(&self.value[..]);
                        if sandboxing.contains(SandboxingFlagSet::SANDBOXED_SCRIPTS_BROWSING_CONTEXT_FLAG) || sandboxing.contains(SandboxingFlagSet::SANDBOXED_ORIGIN_BROWSING_CONTEXT_FLAG) {
                            Blocked
                        } else {
                            Allowed
                        }
                    },
                    _ => Allowed,
                }
            },
            _ => Allowed,
        }
    }
    pub fn inline_check(&self, element: &Element, type_: InlineCheckType, policy: &Policy, source: &str) -> CheckResult {
        use CheckResult::*;
        match &self.name[..] {
            "default-src" => {
                let name = get_the_effective_directive_for_inline_checks(type_);
                if !should_fetch_directive_execute(name, "default-src", policy) {
                    return Allowed;
                }
                Directive {
                    name: name.to_owned(),
                    value: self.value.clone()
                }.inline_check(element, type_, policy, source)
            }
            "script-src" => {
                let name = get_the_effective_directive_for_inline_checks(type_);
                if !should_fetch_directive_execute(name, "script-src", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_element_match_source_list_for_type_and_source(element, type_, source) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "script-src-elem" => {
                let name = get_the_effective_directive_for_inline_checks(type_);
                if !should_fetch_directive_execute(name, "script-src-elem", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_element_match_source_list_for_type_and_source(element, type_, source) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "script-src-attr" => {
                let name = get_the_effective_directive_for_inline_checks(type_);
                if !should_fetch_directive_execute(name, "script-src-attr", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_element_match_source_list_for_type_and_source(element, type_, source) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "style-src" => {
                let name = get_the_effective_directive_for_inline_checks(type_);
                if !should_fetch_directive_execute(name, "style-src", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_element_match_source_list_for_type_and_source(element, type_, source) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "style-src-elem" => {
                let name = get_the_effective_directive_for_inline_checks(type_);
                if !should_fetch_directive_execute(name, "style-src-elem", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_element_match_source_list_for_type_and_source(element, type_, source) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            "style-src-attr" => {
                let name = get_the_effective_directive_for_inline_checks(type_);
                if !should_fetch_directive_execute(name, "style-src-attr", policy) {
                    return Allowed;
                }
                let source_list = SourceList(&self.value);
                if source_list.does_element_match_source_list_for_type_and_source(element, type_, source) == DoesNotMatch {
                    return Blocked;
                }
                Allowed
            }
            _ => Allowed
        }
    }
}

fn get_the_effective_directive_for_inline_checks(type_: InlineCheckType) -> &'static str {
    use InlineCheckType::*;
    match type_ {
        Script | Navigation => "script-src-elem",
        ScriptAttribute => "script-src-attr",
        Style => "style-src-elem",
        StyleAttribute => "style-src-attr",
    }
}

fn script_directives_prerequest_check(request: &Request, directive: &Directive) -> CheckResult {
    use CheckResult::*;
    if request_is_script_like(request) {
        let source_list = SourceList(&directive.value[..]);
        if source_list.does_nonce_match_source_list(&request.nonce) == Matches {
            return Allowed;
        }
        let integrity_expressions: Vec<HashFunction> = directive.value.iter()
            .filter_map(|expression| {
                if let Some(captures) = HASH_SOURCE_GRAMMAR.captures(expression) {
                    if let (Some(algorithm), Some(value)) = (captures.name("algorithm").and_then(|a| HashAlgorithm::from_name(a.as_str())), captures.name("value")) {
                        return Some(HashFunction{ algorithm, value: String::from(value.as_str()) });
                    }
                }
                None
            })
            .collect();
        if !integrity_expressions.is_empty() {
            let integrity_sources = parse_subresource_integrity_metadata(&request.integrity_metadata);
            if let SubresourceIntegrityMetadata::IntegritySources(integrity_sources) = integrity_sources {
                let mut bypass_due_to_integrity_match = true;
                for source in &integrity_sources {
                    if integrity_expressions.iter().filter(|&ex| ex == source).next().is_none() {
                        bypass_due_to_integrity_match = false;
                    }
                }
                if bypass_due_to_integrity_match {
                    return Allowed;
                }
            }
            if directive.value.iter().filter(|ex| ascii_case_insensitive_match(ex, "'strict-dynamic'")).next().is_some() {
                if request.parser_metadata == ParserMetadata::ParserInserted {
                    return Blocked;
                } else {
                    return Allowed;
                }
            }
        }
        if source_list.does_request_match_source_list(request) == DoesNotMatch {
            return Blocked;
        }
    }
    Allowed
}

fn script_directives_postrequest_check(request: &Request, response: &Response, directive: &Directive) -> CheckResult {
    use CheckResult::*;
    if request_is_script_like(request) {
        let source_list = SourceList(&directive.value[..]);
        if source_list.does_nonce_match_source_list(&request.nonce) == Matches {
            return Allowed;
        }
        if directive.value.iter().filter(|ex| ascii_case_insensitive_match(ex, "'strict-dynamic'")).next().is_some() && request.parser_metadata != ParserMetadata::ParserInserted {
            return Allowed;
        }
        if source_list.does_response_to_request_match_source_list(request, response) == DoesNotMatch {
            return Blocked;
        }
    }
    Allowed
}

fn request_is_script_like(request: &Request) -> bool {
    request.destination.is_script_like()
}

fn should_fetch_directive_execute(effective_directive_name: &str, directive_name: &str, policy: &Policy) -> bool {
    let directive_fallback_list = get_fetch_directive_fallback_list(effective_directive_name);
    for fallback_directive in directive_fallback_list {
        if directive_name == *fallback_directive {
            return true;
        }
        if policy.contains_a_directive_whose_name_is(fallback_directive) {
            return false;
        }
    }
    false
}

fn get_fetch_directive_fallback_list(directive_name: &str) -> &'static [&'static str] {
    match directive_name {
        "script-src-elem" => &["script-src-elem", "script-src", "default-src"],
        "script-src-attr" => &["script-src-attr", "script-src", "default-src"],
        "style-src-elem"  => &["style-src-elem", "style-src", "default-src"],
        "style-src-attr"  => &["style-src-attr", "style-src", "default-src"],
        "worker-src"      => &["worker-src", "child-src", "script-src", "default-src"],
        "connect-src"     => &["connect-src", "default-src"],
        "manifest-src"    => &["manifest-src", "default-src"],
        "prefetch-src"    => &["prefetch-src", "default-src"],
        "object-src"      => &["object-src", "default-src"],
        "frame-src"       => &["frame-src", "child-src", "default-src"],
        "media-src"       => &["media-src", "default-src"],
        "font-src"        => &["font-src", "default-src"],
        "img-src"         => &["img-src", "default-src"],
        _                 => &[],
    }
}

fn get_the_effective_directive_for_request(request: &Request) -> &'static str {
    use Initiator::*;
    use Destination::*;
    if request.initiator == Fetch || request.destination == Destination::None {
        return "connect-src";
    }
    if request.initiator == Prefetch || request.initiator == Prerender {
        return "prefetch-src";
    }
    match request.destination {
        Manifest => "manifest-src",
        Object | Embed => "object-src",
        Document => "frame-src",
        Audio | Track | Video => "media-src",
        Font => "font-src",
        Image => "img-src",
        Style => "style-src-elem",
        Script | Xslt => "script-src-elem",
        ServiceWorker | SharedWorker | Worker => "worker-src",
        _ => "",
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MatchResult {
    Matches,
    DoesNotMatch,
}

lazy_static!{
    static ref DIRECTIVE_NAME_GRAMMAR: Regex =
        Regex::new(r#"^[0-9a-z\-]+$"#).unwrap();
    static ref DIRECTIVE_VALUE_TOKEN_GRAMMAR: Regex =
        Regex::new(r#"^[\u{21}-\u{2B}\u{2D}-\u{3A}\u{3C}-\u{7E}]+$"#).unwrap();
    static ref NONCE_SOURCE_GRAMMAR: Regex =
        Regex::new(r#"^'nonce-(?P<n>[a-zA-Z0-9\+/\-_]+=*)'$"#).unwrap();
    static ref NONE_SOURCE_GRAMMAR: Regex =
        Regex::new(r#"^'none'$"#).unwrap();
    static ref SCHEME_SOURCE_GRAMMAR: Regex =
        Regex::new(r#"^(?P<scheme>[a-zA-Z][a-zA-Z0-9\+\-\.]*):$"#).unwrap();
    static ref HOST_SOURCE_GRAMMAR: Regex =
        Regex::new(r#"^((?P<scheme>[a-zA-Z][a-zA-Z0-9\+\-\.]*)://)?(?P<host>\*|(\*\.)?[a-zA-Z0-9\-]+(\.[a-zA-Z0-9\-]+)*)(?P<port>:(\*|[0-9]+))?(?P<path>/([:@%!\$&'\(\)\*\+,;=0-9a-zA-Z\-\._~]+)?(/[:@%!\$&'\(\)\*\+,;=0-9a-zA-Z\-\._~]*)*)?$"#).unwrap();
    static ref HASH_SOURCE_GRAMMAR: Regex =
        Regex::new(r#"^'(?P<algorithm>sha256|sha384|sha512)-(?P<value>[a-zA-Z0-9\+/\-_]+=*)'$"#).unwrap();
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SourceList<'a, U: 'a + ?Sized + Borrow<str>, I: Clone + IntoIterator<Item=&'a U>>(I);

impl<'a, U: 'a + ?Sized + Borrow<str>, I: Clone + IntoIterator<Item=&'a U>> SourceList<'a, U, I> {
    fn does_nonce_match_source_list(&self, nonce: &str) -> MatchResult {
        if nonce.is_empty() { return DoesNotMatch };
        for expression in self.0.clone().into_iter() {
            if let Some(captures) = NONCE_SOURCE_GRAMMAR.captures(expression.borrow()) {
                if let Some(captured_nonce) = captures.name("n") {
                    if nonce == captured_nonce.as_str() {
                        return Matches;
                    }
                }
            }
        }
        DoesNotMatch
    }
    fn does_request_match_source_list(&self, request: &Request) -> MatchResult {
        self.does_url_match_source_list_in_origin_with_redirect_count(
            &request.url,
            &request.origin,
            request.redirect_count,
        )
    }
    fn does_url_match_source_list_in_origin_with_redirect_count(
        &self,
        url: &Url,
        origin: &Origin,
        redirect_count: u32,
    ) -> MatchResult {
        for expression in self.0.clone().into_iter().map(Borrow::borrow) {
            if NONE_SOURCE_GRAMMAR.is_match(expression) { continue };
            let result = does_url_match_expression_in_origin_with_redirect_count(
                url,
                expression,
                origin,
                redirect_count
            );
            if result == Matches {
                return Matches;
            }
        }
        DoesNotMatch
    }
    fn does_element_match_source_list_for_type_and_source(
        &self,
        element: &Element,
        type_: InlineCheckType,
        source: &str,
    ) -> MatchResult {
        if self.does_a_source_list_allow_all_inline_behavior_for_type(type_) == AllowResult::Allows {
            return Matches;
        }
        if type_ == InlineCheckType::Script || type_ == InlineCheckType::Style {
            if let Some(nonce) = element.nonce.as_ref() {
                for expression in self.0.clone().into_iter().map(Borrow::borrow) {
                    if let Some(captures) = NONCE_SOURCE_GRAMMAR.captures(expression.borrow()) {
                        if let Some(captured_nonce) = captures.name("n") {
                            if nonce == captured_nonce.as_str() {
                                return Matches;
                            }
                        }
                    }
                }
            }
        }
        let mut unsafe_hashes = false;
        for expression in self.0.clone().into_iter().map(Borrow::borrow) {
            if ascii_case_insensitive_match(expression, "'unsafe-hashes'") {
                unsafe_hashes = true;
                break;
            }
        }
        if type_ == InlineCheckType::Script || type_ == InlineCheckType::Style || unsafe_hashes {
            for expression in self.0.clone().into_iter().map(Borrow::borrow) {
                if let Some(captures) = HASH_SOURCE_GRAMMAR.captures(expression.borrow()) {
                    if let (Some(algorithm), Some(value)) = (captures.name("algorithm").and_then(|a| HashAlgorithm::from_name(a.as_str())), captures.name("value")) {
                        let actual = algorithm.apply(source);
                        let expected = value.as_str().replace('-', "+").replace('_', "/");
                        if actual == expected {
                            return Matches;
                        }
                    }
                }
            }
        }
        DoesNotMatch
    }
    fn does_a_source_list_allow_all_inline_behavior_for_type(&self, type_: InlineCheckType) -> AllowResult {
        use InlineCheckType::*;
        let mut allow_all_inline = false;
        for expression in self.0.clone().into_iter().map(Borrow::borrow) {
            if HASH_SOURCE_GRAMMAR.is_match(expression) || NONCE_SOURCE_GRAMMAR.is_match(expression) {
                return AllowResult::DoesNotAllow;
            }
            if (type_ == Script || type_ == ScriptAttribute || type_ == Navigation) && expression == "'strict-dynamic'" {
                return AllowResult::DoesNotAllow;
            }
            if ascii_case_insensitive_match(expression, "'unsafe-inline'") {
                allow_all_inline = true;
            }
        }
        if allow_all_inline {
            AllowResult::Allows
        } else {
            AllowResult::DoesNotAllow
        }
    }
    fn does_response_to_request_match_source_list(
        &self,
        request: &Request,
        response: &Response) -> MatchResult {
        self.does_url_match_source_list_in_origin_with_redirect_count(
            &response.url,
            &request.origin,
            response.redirect_count,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AllowResult {
    Allows,
    DoesNotAllow,
}

fn does_url_match_expression_in_origin_with_redirect_count(
    url: &Url,
    expression: &str,
    origin: &Origin,
    redirect_count: u32,
) -> MatchResult {
    let url_scheme = url.scheme();
    if expression == "*" {
        if scheme_is_network(url_scheme) {
            return Matches;
        }
        return origin_scheme_part_match(origin, url_scheme);
    }
    if let Some(captures) = SCHEME_SOURCE_GRAMMAR.captures(expression) {
        if let Some(expression_scheme) = captures.name("scheme") {
            return scheme_part_match(expression_scheme.as_str(), url_scheme);
        }
        // It should not be possible to match HOST_SOURCE_GRAMMAR without having a scheme part
        return DoesNotMatch;
    }
    if let Some(captures) = HOST_SOURCE_GRAMMAR.captures(expression) {
        let expr_has_scheme_part = if let Some(expression_scheme) = captures.name("scheme") {
            if scheme_part_match(expression_scheme.as_str(), url_scheme) != Matches {
                return DoesNotMatch;
            }
            true
        } else {
            false
        };
        let url_host = if let Some(url_host) = url.host() {
            url_host
        } else {
            return DoesNotMatch;
        };
        if !expr_has_scheme_part &&
            origin_scheme_part_match(origin, url.scheme()) != Matches {
            return DoesNotMatch;
        }
        if let Some(expression_host) = captures.name("host") {
            if host_part_match(expression_host.as_str(), &url_host.to_string()) != Matches {
                return DoesNotMatch;
            }
        } else {
            // It should not be possible to match HOST_SOURCE_GRAMMAR without having a host part
            return DoesNotMatch;
        }
        let port_part = captures.name("port").map(|port| port.as_str()).unwrap_or("");
        let url_port = url_port(url);
        if port_part_match(port_part, &url_port[..], url.scheme()) != Matches {
            return DoesNotMatch;
        }
        let path_part = captures.name("path").map(|path_part| path_part.as_str()).unwrap_or("");
        if path_part != "/" && redirect_count == 0 {
            let path = url.path();
            if path_part_match(path_part, path) != Matches {
                return DoesNotMatch;
            }
        }
        return Matches;
    }
    if ascii_case_insensitive_match(expression, "'self'") {
        if *origin == url.origin() {
            return Matches;
        }
        if let &Origin::Tuple(ref scheme, ref host, port) = origin {
            let hosts_are_the_same = Some(host) == url.host().map(|p| p.to_owned()).as_ref();
            let ports_are_the_same = Some(port) == url.port();
            let origins_port_is_default_for_scheme = Some(port) == default_port(scheme);
            let url_port_is_default_port_for_scheme = url.port() == default_port(scheme)
                && default_port(scheme).is_some();
            let ports_are_default = url_port_is_default_port_for_scheme && origins_port_is_default_for_scheme;
            if hosts_are_the_same
                && (ports_are_the_same || ports_are_default)
                && ((url_scheme == "https" || url_scheme == "wss")
                        || (scheme == "http" && (url_scheme == "http" || url_scheme == "ws"))) {
                return Matches;
            }
        }
    }
    DoesNotMatch
}

fn host_part_match(a: &str, b: &str) -> MatchResult {
    debug_assert!(!a.is_empty());
    if a.is_empty() {
        return DoesNotMatch;
    }
    if a.as_bytes()[0] == b'*' {
        let remaining = &a[1..];
        debug_assert_eq!(&remaining[..1], ".");
        if remaining.len() > b.len() {
            return DoesNotMatch;
        }
        let remaining_b = &b[(b.len()-remaining.len())..];
        debug_assert_eq!(remaining_b.len(), remaining.len());
        if ascii_case_insensitive_match(remaining, remaining_b) {
            return Matches;
        } else {
            return DoesNotMatch;
        }
    }
    if !ascii_case_insensitive_match(a, b) {
        return DoesNotMatch;
    }
    lazy_static!{
        static ref IPV4_ADDRESS_RULE: Regex =
            Regex::new(r#"([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\.([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\.([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\.([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])"#).unwrap();
    }
    if IPV4_ADDRESS_RULE.is_match(a) && a != "127.0.0.1" {
        return DoesNotMatch;
    }
    // The spec uses the phrase "if A is an IPv6 address", without giving specific instructions on
    // how to tell if this is the case. In URLs, IPv6 addresses start with `[`, so let's go with that.
    // See https://url.spec.whatwg.org/#host-parsing
    if a.as_bytes()[0] == b'[' {
        return DoesNotMatch;
    }
    Matches
}

fn port_part_match(port_a: &str, port_b: &str, scheme_b: &str) -> MatchResult {
    if port_a.is_empty() {
        if port_b == default_port_str(scheme_b) {
            return Matches;
        } else {
            return DoesNotMatch;
        }
    }
    if port_a == "*" {
        return Matches;
    }
    if port_a == port_b {
        return Matches;
    }
    if port_b == "" {
        if port_a == default_port_str(scheme_b) {
            return Matches;
        } else {
            return DoesNotMatch;
        }
    }
    DoesNotMatch
}

fn path_part_match(path_a: &str, path_b: &str) -> MatchResult {
    if path_a.is_empty() {
        return Matches;
    }
    if path_a == "/" && path_b.is_empty() {
        return Matches;
    }
    let exact_match = path_a.as_bytes()[path_a.len()-1] != b'/';
    let (mut path_list_a, path_list_b): (Vec<&str>, Vec<&str>) =
        (path_a.split('/').collect(), path_b.split('/').collect());
    if path_list_a.len() > path_list_b.len() {
        return DoesNotMatch;
    }
    if exact_match && path_list_a.len() != path_list_b.len() {
        return DoesNotMatch;
    }
    if !exact_match {
        debug_assert_eq!(&path_list_a[path_list_a.len()-1][..], "");
        path_list_a.pop();
    }
    let mut piece_b_iter = path_list_b.iter();
    for piece_a in &path_list_a {
        let piece_b = piece_b_iter.next().unwrap();
        let piece_a: Vec<u8> = percent_encoding::percent_decode(piece_a.as_bytes()).collect();
        let piece_b: Vec<u8> = percent_encoding::percent_decode(piece_b.as_bytes()).collect();
        if piece_a != piece_b {
            return DoesNotMatch;
        }
    }
    Matches
}

fn url_port(url: &Url) -> String {
    match url.port() {
        Some(num) => num.to_string(),
        None => default_port_str(url.scheme()).to_string(),
    }
}

fn default_port_str(scheme: &str) -> &'static str {
    match scheme {
        "ftp" => "21",
        "gopher" => "70",
        "http" => "80",
        "https" => "443",
        "ws" => "80",
        "wss" => "443",
        _ => "",
    }
}

fn default_port(scheme: &str) -> Option<u16> {
    Some(match scheme {
        "ftp" => 21,
        "gopher" => 70,
        "http" => 80,
        "https" => 443,
        "ws" => 80,
        "wss" => 443,
        _ => return None,
    })
}

fn origin_scheme_part_match(a: &Origin, b: &str) -> MatchResult {
    if let &Origin::Tuple(ref scheme, ref _host, ref _port) = a {
        scheme_part_match(&scheme[..], b)
    } else {
        DoesNotMatch
    }
}

fn scheme_part_match(a: &str, b: &str) -> MatchResult {
    let a = a.to_ascii_lowercase();
    let b = b.to_ascii_lowercase();
    match (&a[..], &b[..]) {
        _ if a == b => Matches,
        ("http", "https") |
        ("ws", "wss") |
        ("wss", "https") => Matches,
        _ => DoesNotMatch,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HashAlgorithm {
    Sha256,
    Sha384,
    Sha512,
}

impl HashAlgorithm {
    pub fn from_name(name: &str) -> Option<HashAlgorithm> {
        use HashAlgorithm::*;
        match name {
            "sha256" | "Sha256" | "sHa256" | "shA256" | "SHa256" | "sHA256" | "SHA256" => Some(Sha256),
            "sha384" | "Sha384" | "sHa384" | "shA384" | "SHa384" | "sHA384" | "SHA384" => Some(Sha384),
            "sha512" | "Sha512" | "sHa512" | "shA512" | "SHa512" | "sHA512" | "SHA512" => Some(Sha512),
            _ => None,
        }
    }
    pub fn apply(self, _value: &str) -> String {
        unimplemented!();
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HashFunction {
    algorithm: HashAlgorithm,
    value: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubresourceIntegrityMetadata {
    NoMetadata,
    IntegritySources(Vec<HashFunction>)
}

lazy_static!{
    static ref SUBRESOURCE_METADATA_GRAMMAR: Regex =
        Regex::new(r#"(?P<algorithm>sha256|sha384|sha512)-(?P<value>[a-zA-Z0-9\+/\-_]+=*)"#).unwrap();
}

pub fn parse_subresource_integrity_metadata(string: &str) -> SubresourceIntegrityMetadata {
    let mut result = Vec::new();
    let mut empty = true;
    for token in split_ascii_whitespace(string) {
        empty = false;
        if let Some(captures) = SUBRESOURCE_METADATA_GRAMMAR.captures(token) {
            if let (Some(algorithm), Some(value)) = (captures.name("algorithm").and_then(|a| HashAlgorithm::from_name(a.as_str())), captures.name("value")) {
                result.push(HashFunction{ algorithm, value: String::from(value.as_str()) });
            }
        }
    }
    if empty {
        SubresourceIntegrityMetadata::NoMetadata
    } else {
        SubresourceIntegrityMetadata::IntegritySources(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn empty_directive_is_not_valid() {
        let d = Directive {
            name: String::new(),
            value: Vec::new(),
        };
        assert!(!d.is_valid());
    }
    #[test]
    pub fn duplicate_policy_is_not_valid() {
        let d = Directive {
            name: "test".to_owned(),
            value: vec!["test".to_owned()],
        };
        let p = Policy {
            directive_set: vec![d.clone(), d.clone()],
            disposition: PolicyDisposition::Enforce,
            source: PolicySource::Header,
        };
        assert!(!p.is_valid());
    }
    #[test]
    pub fn basic_policy_is_valid() {
        let p = Policy::parse("script-src notriddle.com", PolicySource::Header, PolicyDisposition::Enforce);
        assert!(p.is_valid());
    }
    #[test]
    pub fn policy_with_empty_directive_set_is_not_valid() {
        let p = Policy {
            directive_set: vec![],
            disposition: PolicyDisposition::Enforce,
            source: PolicySource::Header,
        };
        assert!(!p.is_valid());
    }
}
