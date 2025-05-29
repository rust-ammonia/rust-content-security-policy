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

pub(crate) mod text_util;
pub mod sandboxing_directive;

pub use url::{Origin, Url};
#[cfg(feature = "serde")] use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::borrow::{Borrow, Cow};
use std::cmp;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
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
use regex::Regex;
use once_cell::sync::Lazy;

fn scheme_is_network(scheme: &str) -> bool {
    scheme == "ftp" || scheme_is_httpx(scheme)
}

fn scheme_is_httpx(scheme: &str) -> bool {
    scheme == "http" || scheme == "https"
}

/**
A single parsed content security policy.

https://www.w3.org/TR/CSP/#content-security-policy-object
*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Policy {
    pub directive_set: Vec<Directive>,
    pub disposition: PolicyDisposition,
    pub source: PolicySource,
}

impl Display for Policy {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for (i, directive) in self.directive_set.iter().enumerate() {
            if i != 0 {
                write!(f, "; ")?;
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
    /// https://www.w3.org/TR/CSP/#parse-serialized-policy
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
    /// https://www.w3.org/TR/CSP/#does-request-violate-policy
    pub fn does_request_violate_policy(&self, request: &Request) -> Violates {
        if request.initiator == Initiator::Prefetch {
            return self.does_resource_hint_violate_policy(request);
        }

        let mut violates = Violates::DoesNotViolate;
        for directive in &self.directive_set {
            let result = directive.pre_request_check(request, self);
            if result == CheckResult::Blocked {
                violates = Violates::Directive(directive.clone());
            }
        }
        violates
    }

    /// https://www.w3.org/TR/CSP/#does-resource-hint-violate-policy
    pub fn does_resource_hint_violate_policy(&self, request: &Request) -> Violates {
        let default_directive = &self.directive_set.iter()
            .find(|x| x.name == "default-src");

        if default_directive.is_none() {
            return Violates::DoesNotViolate;
        }

        for directive in &self.directive_set {
            let result = directive.pre_request_check(request, self);
            if result == CheckResult::Allowed {
                return Violates::DoesNotViolate;
            }
        }

        return Violates::Directive(default_directive.unwrap().clone());
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
/// https://www.w3.org/TR/CSP/#csp-list
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
    /// https://www.w3.org/TR/CSP/#contains-a-header-delivered-content-security-policy
    pub fn contains_a_header_delivered_content_security_policy(&self) -> bool {
        self.0.iter().any(|policy| policy.source == PolicySource::Header)
    }
    /// https://www.w3.org/TR/CSP/#parse-serialized-policy-list
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

    https://www.w3.org/TR/CSP/#report-for-request
    */
    pub fn report_violations_for_request(&self, request: &Request)
        -> Vec<Violation> {
        let mut violations = Vec::new();
        for policy in &self.0 {
            if policy.disposition == PolicyDisposition::Enforce { continue };
            let violates = policy.does_request_violate_policy(request);
            if let Violates::Directive(directive) = violates {
                let resource = ViolationResource::Url(request.url.clone());
                violations.push(Violation {
                    resource,
                    directive: Directive {
                        name: get_the_effective_directive_for_request(request).to_owned(),
                        value: directive.value.clone(),
                    },
                    policy: policy.clone(),
                });
            }
        }
        violations
    }
    /**
    Given a request, this algorithm returns Blocked or Allowed and reports violations based on
    request’s client’s Content Security Policy.

    https://www.w3.org/TR/CSP/#should-block-request
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
                violations.push(Violation {
                    resource,
                    directive: Directive {
                        name: get_the_effective_directive_for_request(request).to_owned(),
                        value: directive.value.clone(),
                    },
                    policy: policy.clone(),
                });
            }
        }
        (result, violations)
    }
    /**
    Given a response and a request, this algorithm returns Blocked or Allowed, and reports
    violations based on request’s client’s Content Security Policy.

    https://www.w3.org/TR/CSP/#should-block-response
    */
    pub fn should_response_to_request_be_blocked(&self, request: &Request, response: &Response)
        -> (CheckResult, Vec<Violation>) {
        // Step 1. Let CSP list be request’s policy container’s CSP list.
        // step 2. Let result be "Allowed".
        let mut result = CheckResult::Allowed;
        let mut violations = Vec::new();
        // Step 3. For each policy of CSP list:
        for policy in &self.0 {
            // Step 3.1. For each directive of policy:
            for directive in &policy.directive_set {
                // Step 3.1.1. If the result of executing directive’s post-request check is "Blocked", then:
                if directive.post_request_check(request, response, policy) == CheckResult::Blocked {
                    // Step 3.1.1.1. Execute §5.5 Report a violation on the result of executing
                    // §2.4.2 Create a violation object for request, and policy. on request, and policy.
                    violations.push(Violation {
                        resource: ViolationResource::Url(request.url.clone()),
                        directive: Directive {
                            name: get_the_effective_directive_for_request(request).to_owned(),
                            value: directive.value.clone(),
                        },
                        policy: policy.clone(),
                    });
                    // Step 3.1.1.2. If policy’s disposition is "enforce", then set result to "Blocked".
                    if policy.disposition == PolicyDisposition::Enforce {
                        result = CheckResult::Blocked;
                    }
                }
            }
        }
        (result, violations)
    }
    /// https://www.w3.org/TR/CSP/#should-block-inline
    pub fn should_elements_inline_type_behavior_be_blocked(&self, element: &Element, type_: InlineCheckType, source: &str) -> (CheckResult, Vec<Violation>) {
        use CheckResult::*;
        let mut result = Allowed;
        let mut violations = Vec::new();
        for policy in &self.0 {
            for directive in &policy.directive_set {
                if directive.inline_check(element, type_, policy, source) == Allowed {
                    continue;
                }
                let sample = if directive.value.iter().any(|t| &t[..] == "'report-sample'") {
                    let max_length = cmp::min(40, source.len());
                    Some(source[0..max_length].to_owned())
                } else {
                    None
                };
                let violation = Violation {
                    resource: ViolationResource::Inline{ sample },
                    directive: Directive {
                        name: get_the_effective_directive_for_inline_checks(type_).to_owned(),
                        value: directive.value.clone(),
                    },
                    policy: policy.clone(),
                };
                violations.push(violation);
                if policy.disposition == PolicyDisposition::Enforce {
                    result = Blocked;
                }
            }
        }
        (result, violations)
    }
    /**
    https://www.w3.org/TR/CSP/#allow-base-for-document

    Note that, while this algoritm is defined as operating on a document, the only property it
    actually uses is the document's CSP List. So this function operates on that.
    */
    pub fn is_base_allowed_for_document(&self, base: &Url, self_origin: &Origin) -> (CheckResult, Vec<Violation>) {
        use CheckResult::*;
        let mut violations = Vec::new();
        for policy in &self.0 {
            let directive = policy.directive_set.iter().find(|directive| directive.name == "base-uri");
            if let Some(directive) = directive {
                if SourceList(&directive.value).does_url_match_source_list_in_origin_with_redirect_count(base, &self_origin, 0) == DoesNotMatch {
                    let violation = Violation {
                        directive: directive.clone(),
                        resource: ViolationResource::Inline { sample: None },
                        policy: policy.clone(),
                    };
                    violations.push(violation);
                    if policy.disposition == PolicyDisposition::Enforce {
                        return (Blocked, violations);
                    }
                }
            }
        }
        return (Allowed, violations);
    }

    /**
    https://w3c.github.io/trusted-types/dist/spec/#should-block-create-policy

    Note that, while this algoritm is defined as operating on a global object, the only property it
    actually uses is the global's CSP List. So this function operates on that.
    */
    pub fn is_trusted_type_policy_creation_allowed(&self, policy_name: String, created_policy_names: Vec<String>) -> (CheckResult, Vec<Violation>) {
        use CheckResult::*;
        // Step 1: Let result be "Allowed".
        let mut result = Allowed;
        let mut violations = Vec::new();
        // Step 2: For each policy in global’s CSP list:
        for policy in &self.0 {
            // Step 2.1: Let createViolation be false.
            let mut create_violation = false;
            // Step 2.2: If policy’s directive set does not contain a directive which name is "trusted-types", skip to the next policy.
            let directive = policy.directive_set.iter().find(|directive| directive.name == "trusted-types");
            // Step 2.3: Let directive be the policy’s directive set’s directive which name is "trusted-types"
            if let Some(directive) = directive {
                // Step 2.4: If directive’s value only contains a tt-keyword which is a match for a value 'none', set createViolation to true.
                if directive.value.len() == 1 && directive.value.contains(&"'none'".to_string()) {
                    create_violation = true;
                }
                // Step 2.5: If createdPolicyNames contains policyName and directive’s value does not contain a tt-keyword
                // which is a match for a value 'allow-duplicates', set createViolation to true.
                if created_policy_names.contains(&policy_name.clone()) && !directive.value.contains(&"'allow-duplicates'".to_string()) {
                    create_violation = true;
                }
                // Step 2.6: If directive’s value does not contain a tt-policy-name, which value is policyName,
                // and directive’s value does not contain a tt-wildcard, set createViolation to true.
                if !directive.value.contains(&policy_name) && !directive.value.contains(&"*".to_string()) {
                    create_violation = true;
                }
                // Step 2.7: If createViolation is false, skip to the next policy.
                if !create_violation {
                    continue;
                }
                let mut sample = policy_name.clone();
                // Step 2.10: Set violation’s sample to the substring of policyName, containing its first 40 characters.
                sample.truncate(40);
                // Step 2.8: Let violation be the result of executing Create a violation object for global, policy,
                // and directive on global, policy and "trusted-types"
                let violation = Violation {
                    directive: directive.clone(),
                    // Step 2.9: Set violation’s resource to "trusted-types-policy".
                    resource: ViolationResource::TrustedTypePolicy {
                        // Step 2.10: Set violation’s sample to the substring of policyName, containing its first 40 characters.
                        sample,
                    },
                    policy: policy.clone(),
                };
                // Step 2.11: Execute Report a violation on violation.
                violations.push(violation);
                // Step 2.12: If policy’s disposition is "enforce", then set result to "Blocked".
                if policy.disposition == PolicyDisposition::Enforce {
                    result = Blocked
                }
            }
        }
        return (result, violations);
    }
    /**
    https://w3c.github.io/trusted-types/dist/spec/#abstract-opdef-does-sink-type-require-trusted-types

    Note that, while this algoritm is defined as operating on a global object, the only property it
    actually uses is the global's CSP List. So this function operates on that.
    */
    pub fn does_sink_type_require_trusted_types(&self, sink_group: &str, include_report_only_policies: bool) -> bool {
        let sink_group = &sink_group.to_owned();
        // Step 1: For each policy in global’s CSP list:
        for policy in &self.0 {
            // Step 1.1: If policy’s directive set does not contain a directive whose name is "require-trusted-types-for", skip to the next policy.
            let directive = policy.directive_set.iter().find(|directive| directive.name == "require-trusted-types-for");
            // Step 1.2: Let directive be the policy’s directive set’s directive whose name is "require-trusted-types-for"
            if let Some(directive) = directive {
                // Step 1.3: If directive’s value does not contain a trusted-types-sink-group which is a match for sinkGroup, skip to the next policy.
                if !directive.value.contains(sink_group) {
                    continue;
                }
                // Step 1.4: Let enforced be true if policy’s disposition is "enforce", and false otherwise.
                let enforced = policy.disposition == PolicyDisposition::Enforce;
                // Step 1.5: If enforced is true, return true.
                if enforced {
                    return true;
                }
                // Step 1.6: If includeReportOnlyPolicies is true, return true.
                if include_report_only_policies {
                    return true;
                }
            }
        }
        // Step 2: Return false.
        false
    }
    /**
    https://w3c.github.io/trusted-types/dist/spec/#should-block-sink-type-mismatch

    Note that, while this algoritm is defined as operating on a global object, the only property it
    actually uses is the global's CSP List. So this function operates on that.
    */
    pub fn should_sink_type_mismatch_violation_be_blocked_by_csp(&self, sink: &str, sink_group: &str, source: &str) -> (CheckResult, Vec<Violation>) {
        use CheckResult::*;
        let sink_group = &sink_group.to_owned();
        // Step 1: Let result be "Allowed".
        let mut result = Allowed;
        let mut violations = Vec::new();
        // Step 2: Let sample be source.
        let sample = source;
        // Step 3: If sink is "Function", then:
        if sink == "Function" {
            // There currently is no sink of function, so skipping these for now
            // Step 3.1: If sample starts with "function anonymous", strip that from sample.
            // Step 3.2: Otherwise if sample starts with "async function anonymous", strip that from sample.
            // Step 3.3: Otherwise if sample starts with "function* anonymous", strip that from sample.
            // Step 3.4: Otherwise if sample starts with "async function* anonymous", strip that from sample.
        }
        // Step 4: For each policy in global’s CSP list:
        for policy in &self.0 {
            // Step 4.1: If policy’s directive set does not contain a directive whose name is "require-trusted-types-for", skip to the next policy.
            let directive = policy.directive_set.iter().find(|directive| directive.name == "require-trusted-types-for");
            // Step 4.2: Let directive be the policy’s directive set’s directive whose name is "require-trusted-types-for"
            let Some(directive) = directive else { continue };
            // Step 4.3: If directive’s value does not contain a trusted-types-sink-group which is a match for sinkGroup, skip to the next policy.
            if !directive.value.contains(sink_group) {
                continue;
            }
            // Step 4.6: Let trimmedSample be the substring of sample, containing its first 40 characters.
            let mut trimmed_sample: String = sample.into();
            trimmed_sample.truncate(40);
            // Step 4.4: Let violation be the result of executing Create a violation object for global, policy,
            // and directive on global, policy and "require-trusted-types-for"
            violations.push(Violation {
                // Step 4.5: Set violation’s resource to "trusted-types-sink".
                resource: ViolationResource::TrustedTypeSink {
                    // Step 4.7: Set violation’s sample to be the result of concatenating the list « sink, trimmedSample « using "|" as a separator.
                    sample: sink.to_owned() + "|" + &trimmed_sample,
                },
                directive: directive.clone(),
                policy: policy.clone(),
            });
            // Step 4.9: If policy’s disposition is "enforce", then set result to "Blocked".
            if policy.disposition == PolicyDisposition::Enforce {
                result = Blocked
            }
        }
        // Step 2: Return false.
        (result, violations)
    }
    pub fn get_sandboxing_flag_set_for_document(&self) -> Option<SandboxingFlagSet> {
        self.0
            .iter()
            .flat_map(|policy| {
                policy.directive_set
                    .iter()
                    .find(|directive| directive.name == "sandbox")
                    .and_then(|directive| directive.get_sandboxing_flag_set_for_document(policy))
            })
            .next()
    }
    /// https://www.w3.org/TR/CSP/#can-compile-strings
    pub fn is_js_evaluation_allowed(&self, source: &str) -> (CheckResult, Vec<Violation>) {
        let mut result = CheckResult::Allowed;
        let mut violations = Vec::new();
        // Step 5: For each policy of global’s CSP list:
        for policy in &self.0 {
            // Step 5.1: Let source-list be null.
            let directive = policy.directive_set
                .iter()
                // Step 5.2: If policy contains a directive whose name is "script-src",
                // then set source-list to that directive’s value.
                .find(|directive| directive.name == "script-src")
                // Step 5.2: Otherwise if policy contains a directive whose name is "default-src",
                // then set source-list to that directive’s value.
                .or_else(|| policy.directive_set.iter().find(|directive| directive.name == "default-src"));
            // Step 5.3: If source-list is not null:
            let Some(directive) = directive else { continue };
            let source_list = SourceList(&directive.value);
            if source_list.does_a_source_list_allow_js_evaluation() == AllowResult::Allows {
                continue;
            }
            // Step 5.3.1: Let trustedTypesRequired be the result of executing
            // Does sink type require trusted types?, with realm, 'script', and false.
            let trusted_types_required = self.does_sink_type_require_trusted_types("'script'", false);
            // Step 5.3.2: If trustedTypesRequired is true and source-list contains a source expression
            // which is an ASCII case-insensitive match for the string "'trusted-types-eval'", then skip the following steps.
            if trusted_types_required && directive.value.iter().any(|t| ascii_case_insensitive_match(&t[..], "'trusted-types-eval'")) {
                continue;
            }
            // Step 5.3.3: If source-list contains a source expression which is
            // an ASCII case-insensitive match for the string "'unsafe-eval'", then skip the following steps.
            if directive.value.iter().any(|t| ascii_case_insensitive_match(&t[..], "'unsafe-eval'")) {
                continue;
            }
            // Step 5.3.6: If source-list contains the expression "'report-sample'",
            // then set violation’s sample to the substring of sourceString containing its first 40 characters.
            let sample = if directive.value.iter().any(|t| &t[..] == "'report-sample'") {
                let max_length = cmp::min(40, source.len());
                Some(source[0..max_length].to_owned())
            } else {
                None
            };
            // Step 5.3.4: Let violation be the result of executing Create a violation object for global, policy,
            // and directive on global, policy and "require-trusted-types-for"
            violations.push(Violation {
                // Step 5.3.5: Set violation’s resource to "eval".
                resource: ViolationResource::Eval { sample },
                directive: directive.clone(),
                policy: policy.clone(),
            });
            // Step 5.3.8: If policy’s disposition is "enforce", then set result to "Blocked".
            if policy.disposition == PolicyDisposition::Enforce {
                result = CheckResult::Blocked
            }
        }
        (result, violations)
    }
    /// https://www.w3.org/TR/CSP/#can-compile-wasm-bytes
    pub fn is_wasm_evaluation_allowed(&self) -> (CheckResult, Vec<Violation>) {
        let mut result = CheckResult::Allowed;
        let mut violations = Vec::new();
        // Step 3: For each policy of global’s CSP list:
        for policy in &self.0 {
            // Step 3.1: Let source-list be null.
            let directive = policy.directive_set
                .iter()
                // Step 3.2: If policy contains a directive whose name is "script-src",
                // then set source-list to that directive’s value.
                .find(|directive| directive.name == "script-src")
                // Step 3.2: Otherwise if policy contains a directive whose name is "default-src",
                // then set source-list to that directive’s value.
                .or_else(|| policy.directive_set.iter().find(|directive| directive.name == "default-src"));
            let Some(directive) = directive else { continue };
            let source_list = SourceList(&directive.value);
            // Step 3.3: If source-list is non-null, and does not contain a source expression
            // which is an ASCII case-insensitive match for the string "'unsafe-eval'",
            // and does not contain a source expression which is an ASCII case-insensitive
            // match for the string "'wasm-unsafe-eval'", then:
            if source_list.does_a_source_list_allow_wasm_evaluation() == AllowResult::Allows {
                continue;
            }
            // Step 3.3.1: Let violation be the result of executing § 2.4.1 Create a violation
            // object for global, policy, and directive on global, policy, and "script-src".
            violations.push(Violation {
                // Step 5.3.5: Set violation’s resource to "wasm-eval".
                resource: ViolationResource::WasmEval,
                directive: directive.clone(),
                policy: policy.clone(),
            });
            // Step 3.3.4: If policy’s disposition is "enforce", then set result to "Blocked".
            if policy.disposition == PolicyDisposition::Enforce {
                result = CheckResult::Blocked
            }
        }
        (result, violations)
    }
    /// <https://w3c.github.io/webappsec-csp/#should-block-navigation-request>
    pub fn should_navigation_request_be_blocked(&self, request: &Request, navigation_check_type: NavigationCheckType) -> (CheckResult, Vec<Violation>) {
        // Step 1: Let result be "Allowed".
        let mut result = CheckResult::Allowed;
        let mut violations = Vec::new();
        // Step 2: For each policy of navigation request’s policy container’s CSP list:
        for policy in &self.0 {
            // Step 2.1: For each directive of policy:
            for directive in &policy.directive_set {
                // Step 2.1.1: If directive’s pre-navigation check returns "Allowed"
                // when executed upon navigation request, type, and policy skip to the next directive.
                if directive.pre_navigation_check(request, navigation_check_type, policy) == CheckResult::Allowed {
                    continue;
                }
                // Step 2.1.2: Otherwise, let violation be the result of executing
                // § 2.4.1 Create a violation object for global, policy, and directive
                // on navigation request’s client’s global object, policy, and directive’s name.
                violations.push(Violation {
                    // Step 2.1.3: Set violation’s resource to navigation request’s URL.
                    resource: ViolationResource::Url(request.url.clone()),
                    directive: Directive {
                        name: get_the_effective_directive_for_request(request).to_owned(),
                        value: directive.value.clone(),
                    },
                    policy: policy.clone(),
                });
                // Step 2.1.5: If policy’s disposition is "enforce", then set result to "Blocked".
                if policy.disposition == PolicyDisposition::Enforce {
                    result = CheckResult::Blocked;
                }
            }
        }
        // Step 3: If result is "Allowed", and if navigation request’s current URL’s scheme is javascript:
        if result == CheckResult::Allowed && request.url.scheme() == "javascript" {
            // Step 3.1: For each policy of navigation request’s policy container’s CSP list:
            for policy in &self.0 {
                // Step 3.1.1: For each directive of policy:
                for directive in &policy.directive_set {
                    // Step 3.1.1.2: If directive’s inline check returns "Allowed" when executed upon null,
                    // "navigation" and navigation request’s current URL, skip to the next directive.
                    if directive.inline_check(&Element { nonce: None }, InlineCheckType::Navigation, policy, request.url.as_str()) == CheckResult::Allowed {
                        continue;
                    }
                    // Step 3.1.1.3: Otherwise, let violation be the result of executing
                    // § 2.4.1 Create a violation object for global, policy, and directive
                    // on navigation request’s client’s global object, policy, and directive’s name.
                    violations.push(Violation {
                        // Step 3.1.1.4: Set violation’s resource to navigation request’s URL.
                        resource: ViolationResource::Inline { sample: None },
                        directive: Directive {
                            // Step 3.1.1.1: Let directive-name be the result of executing
                            // § 6.8.2 Get the effective directive for inline checks on type.
                            name: get_the_effective_directive_for_inline_checks(InlineCheckType::Navigation).to_owned(),
                            value: directive.value.clone(),
                        },
                        policy: policy.clone(),
                    });
                    // Step 3.1.1.6: If policy’s disposition is "enforce", then set result to "Blocked".
                    if policy.disposition == PolicyDisposition::Enforce {
                        result = CheckResult::Blocked;
                    }
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

/**
The valid values for type are "script", "script attribute", "style", and "style attribute".

https://www.w3.org/TR/CSP/#should-block-inline
*/
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InlineCheckType {
    Script,
    ScriptAttribute,
    Style,
    StyleAttribute,
    Navigation,
}

/**
The valid values for type are "form-submission" and "other".

https://w3c.github.io/webappsec-csp/#directive-pre-navigation-check
*/
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NavigationCheckType {
    FormSubmission,
    Other,
}

/**
request to be validated

https://fetch.spec.whatwg.org/#concept-request
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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Initiator {
    Download,
    ImageSet,
    Manifest,
    Prefetch,
    Prerender,
    Fetch,
    Xslt,
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
    Frame,
    IFrame,
    Image,
    Json,
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
    WebIdentity,
    Worker,
    Xslt,
}

pub struct InvalidDestination;

impl FromStr for Destination {
    type Err = InvalidDestination;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let destination = match s {
            "" => Self::None,
            "audio" => Self::Audio,
            "audioworklet" => Self::AudioWorklet,
            "document" => Self::Document,
            "embed" => Self::Embed,
            "font" => Self::Font,
            "frame" => Self::Frame,
            "iframe" => Self::IFrame,
            "image" => Self::Image,
            "json" => Self::Json,
            "manifest" => Self::Manifest,
            "object" => Self::Object,
            "paintworklet" => Self::PaintWorklet,
            "report" => Self::Report,
            "script" => Self::Script,
            "serviceworker" => Self::ServiceWorker,
            "sharedworker" => Self::SharedWorker,
            "style" => Self::Style,
            "track" => Self::Track,
            "video" => Self::Video,
            "webidentity" => Self::WebIdentity,
            "worker" => Self::Worker,
            "xslt" => Self::Xslt,
            _ => return Err(InvalidDestination),
        };

        Ok(destination)
    }
}

impl Destination {
    /// https://fetch.spec.whatwg.org/#request-destination-script-like
    pub fn is_script_like(self) -> bool {
        use Destination::*;
        matches!(self, AudioWorklet | PaintWorklet | Script | ServiceWorker | SharedWorker | Worker | Xslt)
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Audio => "audio",
            Self::AudioWorklet => "audioworklet",
            Self::Document => "document",
            Self::Embed => "embed",
            Self::Font => "font",
            Self::Frame => "frame",
            Self::IFrame => "iframe",
            Self::Image => "image",
            Self::Json => "json",
            Self::Manifest => "manifest",
            Self::Object => "object",
            Self::PaintWorklet => "paintworklet",
            Self::Report => "report",
            Self::Script => "script",
            Self::ServiceWorker => "serviceworker",
            Self::SharedWorker => "sharedworker",
            Self::Style => "style",
            Self::Track => "track",
            Self::Video => "video",
            Self::WebIdentity => "webidentity",
            Self::Worker => "worker",
            Self::Xslt => "xslt",
        }
    }
}

/**
response to be validated
https://fetch.spec.whatwg.org/#concept-response
*/
#[derive(Clone, Debug)]
pub struct Response {
    pub url: Url,
    pub redirect_count: u32,
}

/**
violation information

https://www.w3.org/TR/CSP/#violation
*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Violation {
    pub resource: ViolationResource,
    pub directive: Directive,
    pub policy: Policy,
}

/**
violation information

https://www.w3.org/TR/CSP/#violation
*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ViolationResource {
    Url(Url),
    Inline {
        sample: Option<String>,
    },
    TrustedTypePolicy {
        sample: String,
    },
    TrustedTypeSink {
        sample: String,
    },
    Eval {
        sample: Option<String>,
    },
    WasmEval,
}

/**
Many algorithms are allowed to return either "Allowed" or "Blocked".
The spec describes these as strings.
*/
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum CheckResult {
    Allowed,
    Blocked,
}

/**
https://www.w3.org/TR/CSP/#does-request-violate-policy
*/
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Violates {
    DoesNotViolate,
    Directive(Directive),
}

/// https://www.w3.org/TR/CSP/#policy-disposition
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum PolicyDisposition {
    Enforce,
    Report,
}

/// https://www.w3.org/TR/CSP/#policy-source
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum PolicySource {
    Header,
    Meta,
}

/// https://www.w3.org/TR/CSP/#directives
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Directive {
    pub name: String,
    pub value: Vec<String>,
}

impl Display for Directive {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        <str as Display>::fmt(&self.name[..], f)?;
        write!(f, " ")?;
        for (i, token) in self.value.iter().enumerate() {
            if i != 0 {
                write!(f, " ")?;
            }
            <str as Display>::fmt(&token[..], f)?;
        }
        Ok(())
    }
}

impl Directive {
    /// https://www.w3.org/TR/CSP/#serialized-directive
    pub fn is_valid(&self) -> bool {
        DIRECTIVE_NAME_GRAMMAR.is_match(&self.name) &&
            self.value.iter().all(|t| DIRECTIVE_VALUE_TOKEN_GRAMMAR.is_match(&t[..]))
    }
    /// https://www.w3.org/TR/CSP/#directive-pre-request-check
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
    /// https://www.w3.org/TR/CSP/#directive-post-request-check
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
    /// https://www.w3.org/TR/CSP/#directive-inline-check
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
    /// https://www.w3.org/TR/CSP/#sandbox-init
    pub fn get_sandboxing_flag_set_for_document(&self, policy: &Policy) -> Option<SandboxingFlagSet> {
        use PolicyDisposition::*;
        match &self.name[..] {
            "sandbox" => {
                if policy.disposition != Enforce {
                    None
                } else {
                    Some(parse_a_sandboxing_directive(&self.value[..]))
                }
            },
            _ => None,
        }
    }
    /// <https://w3c.github.io/webappsec-csp/#directive-pre-navigation-check>
    pub fn pre_navigation_check(&self, request: &Request, type_: NavigationCheckType, _policy: &Policy) -> CheckResult {
        use CheckResult::*;
        match &self.name[..] {
            // <https://w3c.github.io/webappsec-csp/#form-action-pre-navigate>
            "form-action" => {
                // Step 2: If navigation type is "form-submission":
                if type_ == NavigationCheckType::FormSubmission {
                    let source_list = SourceList(&self.value);
                    // Step 2.1: If the result of executing § 6.7.2.5 Does request match source list? on request,
                    // this directive’s value, and a policy, is "Does Not Match", return "Blocked".
                    if source_list.does_request_match_source_list(request) == DoesNotMatch {
                        return Blocked;
                    }
                }
                // Step 3: Return "Allowed".
                Allowed
            },
            _ => Allowed,
        }
    }
}

/// https://www.w3.org/TR/CSP/#effective-directive-for-inline-check
fn get_the_effective_directive_for_inline_checks(type_: InlineCheckType) -> &'static str {
    use InlineCheckType::*;
    match type_ {
        Script | Navigation => "script-src-elem",
        ScriptAttribute => "script-src-attr",
        Style => "style-src-elem",
        StyleAttribute => "style-src-attr",
    }
}

/// <https://www.w3.org/TR/CSP/#script-pre-request>
fn script_directives_prerequest_check(request: &Request, directive: &Directive) -> CheckResult {
    use CheckResult::*;
    // Step 1. If request’s destination is script-like:
    if request_is_script_like(request) {
        let source_list = SourceList(&directive.value[..]);
        // Step 1.1. If the result of executing § 6.7.2.3 Does nonce match source list? on
        // request’s cryptographic nonce metadata and this directive’s value is "Matches", return "Allowed".
        if source_list.does_nonce_match_source_list(&request.nonce) == Matches {
            return Allowed;
        }
        // Step 1.2. If the result of executing § 6.7.2.4 Does integrity metadata match source list? on
        // request’s integrity metadata and this directive’s value is "Matches", return "Allowed".
        if source_list.does_integrity_metadata_match_source_list(&request.integrity_metadata) == Matches {
            return Allowed;
        }
        // Step 1.3. If directive’s value contains a source expression that is an
        // ASCII case-insensitive match for the "'strict-dynamic'" keyword-source:
        if directive.value.iter().any(|ex| ascii_case_insensitive_match(ex, "'strict-dynamic'")) {
            // Step 1.3.1. If the request’s parser metadata is "parser-inserted", return "Blocked".
            if request.parser_metadata == ParserMetadata::ParserInserted {
                return Blocked;
            }
            // Otherwise, return "Allowed".
            return Allowed;
        }

        // Step 1.4. If the result of executing § 6.7.2.5 Does request match source list? on
        // request, directive’s value, and policy, is "Does Not Match", return "Blocked".
        if source_list.does_request_match_source_list(request) == DoesNotMatch {
            return Blocked;
        }
    }
    // Step 2. Return "Allowed".
    Allowed
}

/// https://www.w3.org/TR/CSP/#script-post-request
fn script_directives_postrequest_check(request: &Request, response: &Response, directive: &Directive) -> CheckResult {
    use CheckResult::*;
    // Step 1. If request’s destination is script-like:
    if request_is_script_like(request) {
        // Step 1.1. Call potentially report hash with response, request, directive and policy.
        // TODO
        let source_list = SourceList(&directive.value[..]);
        // Step 1.2. If the result of executing § 6.7.2.3 Does nonce match source list? on
        // request’s cryptographic nonce metadata and this directive’s value is "Matches", return "Allowed".
        if source_list.does_nonce_match_source_list(&request.nonce) == Matches {
            return Allowed;
        }
        // Step 1.3. If the result of executing § 6.7.2.4 Does integrity metadata match source list? on
        // request’s integrity metadata and this directive’s value is "Matches", return "Allowed".
        if source_list.does_integrity_metadata_match_source_list(&request.integrity_metadata) == Matches {
            return Allowed;
        }
        // Step 1.4. If directive’s value contains "'strict-dynamic'":
        if directive.value.iter().any(|ex| ascii_case_insensitive_match(ex, "'strict-dynamic'")) {
            // Step 1.4.1. If the request’s parser metadata is "parser-inserted", return "Blocked".
            if request.parser_metadata == ParserMetadata::ParserInserted {
                return Blocked;
            }
            // Otherwise, return "Allowed".
            return Allowed;
        }
        // Step 1.5. If the result of executing § 6.7.2.6 Does response to request match source list? on
        // response, request, directive’s value, and policy, is "Does Not Match", return "Blocked".
        if source_list.does_response_to_request_match_source_list(request, response) == DoesNotMatch {
            return Blocked;
        }
    }
    // Step 2. Return "Allowed".
    Allowed
}

/// https://fetch.spec.whatwg.org/#request-destination-script-like
fn request_is_script_like(request: &Request) -> bool {
    request.destination.is_script_like()
}

/// https://www.w3.org/TR/CSP/#should-directive-execute
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

/// https://www.w3.org/TR/CSP/#directive-fallback-list
fn get_fetch_directive_fallback_list(directive_name: &str) -> &'static [&'static str] {
    match directive_name {
        "script-src-elem" => &["script-src-elem", "script-src", "default-src"],
        "script-src-attr" => &["script-src-attr", "script-src", "default-src"],
        "style-src-elem"  => &["style-src-elem", "style-src", "default-src"],
        "style-src-attr"  => &["style-src-attr", "style-src", "default-src"],
        "worker-src"      => &["worker-src", "child-src", "script-src", "default-src"],
        "connect-src"     => &["connect-src", "default-src"],
        "manifest-src"    => &["manifest-src", "default-src"],
        "object-src"      => &["object-src", "default-src"],
        "frame-src"       => &["frame-src", "child-src", "default-src"],
        "media-src"       => &["media-src", "default-src"],
        "font-src"        => &["font-src", "default-src"],
        "img-src"         => &["img-src", "default-src"],
        _                 => &[],
    }
}

/// https://www.w3.org/TR/CSP/#effective-directive-for-a-request
fn get_the_effective_directive_for_request(request: &Request) -> &'static str {
    use Initiator::*;
    use Destination::*;
    // Step 1: If request’s initiator is "prefetch" or "prerender", return default-src.
    if request.initiator == Prefetch || request.initiator == Prerender {
        return "default-src";
    }
    // Step 2: Switch on request’s destination, and execute the associated steps:
    match request.destination {
        Destination::Manifest => "manifest-src",
        Object | Embed => "object-src",
        Frame | IFrame => "frame-src",
        Audio | Track | Video => "media-src",
        Font => "font-src",
        Image => "img-src",
        Style => "style-src-elem",
        Script | Destination::Xslt | AudioWorklet | PaintWorklet => "script-src-elem",
        ServiceWorker | SharedWorker | Worker => "worker-src",
        Json | WebIdentity => "connect-src",
        Report => "",
        // Step 3: Return connect-src.
        _ => "connect-src",
    }
}

/// https://www.w3.org/TR/CSP/#match-element-to-source-list
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MatchResult {
    Matches,
    DoesNotMatch,
}

/// https://www.w3.org/TR/CSP/#grammardef-directive-name
static DIRECTIVE_NAME_GRAMMAR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^[0-9a-z\-]+$"#).unwrap());
/// https://www.w3.org/TR/CSP/#grammardef-directive-value
static DIRECTIVE_VALUE_TOKEN_GRAMMAR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^[\u{21}-\u{2B}\u{2D}-\u{3A}\u{3C}-\u{7E}]+$"#).unwrap());
/// https://www.w3.org/TR/CSP/#grammardef-nonce-source
static NONCE_SOURCE_GRAMMAR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^'nonce-(?P<n>[a-zA-Z0-9\+/\-_]+=*)'$"#).unwrap());
static NONE_SOURCE_GRAMMAR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^'none'$"#).unwrap());
/// https://www.w3.org/TR/CSP/#grammardef-scheme-source
static SCHEME_SOURCE_GRAMMAR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^(?P<scheme>[a-zA-Z][a-zA-Z0-9\+\-\.]*):$"#).unwrap());
/// https://www.w3.org/TR/CSP/#grammardef-host-source
static HOST_SOURCE_GRAMMAR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^((?P<scheme>[a-zA-Z][a-zA-Z0-9\+\-\.]*)://)?(?P<host>\*|(\*\.)?[a-zA-Z0-9\-]+(\.[a-zA-Z0-9\-]+)*)(?P<port>:(\*|[0-9]+))?(?P<path>/([:@%!\$&'\(\)\*\+,;=0-9a-zA-Z\-\._~]+)?(/[:@%!\$&'\(\)\*\+,;=0-9a-zA-Z\-\._~]*)*)?$"#).unwrap());
/// https://www.w3.org/TR/CSP/#grammardef-hash-source
static HASH_SOURCE_GRAMMAR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^'(?P<algorithm>[sS][hH][aA](256|384|512))-(?P<value>[a-zA-Z0-9\+/\-_]+=*)'$"#).unwrap());

/// https://www.w3.org/TR/CSP/#framework-directive-source-list
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SourceList<'a, U: 'a + ?Sized + Borrow<str>, I: Clone + IntoIterator<Item=&'a U>>(I);

impl<'a, U: 'a + ?Sized + Borrow<str>, I: Clone + IntoIterator<Item=&'a U>> SourceList<'a, U, I> {
    /// https://www.w3.org/TR/CSP/#match-nonce-to-source-list
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
    /// https://www.w3.org/TR/CSP/#match-integrity-metadata-to-source-list
    fn does_integrity_metadata_match_source_list(&self, integrity_metadata: &str) -> MatchResult {
        // Step 2: Let integrity expressions be the set of source expressions in source list that match the hash-source grammar.
        let integrity_expressions: Vec<HashFunction> = self.0.clone().into_iter()
            .filter_map(|expression| {
                if let Some(captures) = HASH_SOURCE_GRAMMAR.captures(expression.borrow()) {
                    if let (Some(algorithm), Some(value)) = (captures.name("algorithm").and_then(|a| HashAlgorithm::from_name(a.as_str())), captures.name("value")) {
                        return Some(HashFunction{ algorithm, value: String::from(value.as_str()) });
                    }
                }
                None
            })
            .collect();
        // Step 3: If integrity expressions is empty, return "Does Not Match".
        if integrity_expressions.is_empty() {
            return DoesNotMatch;
        }
        // Step 4: Let integrity sources be the result of executing the algorithm defined in SRI § 3.3.3 Parse metadata. on integrity metadata.
        let integrity_sources = parse_subresource_integrity_metadata(integrity_metadata);
        match integrity_sources {
            // Step 5: If integrity sources is "no metadata" or an empty set, return "Does Not Match".
            SubresourceIntegrityMetadata::NoMetadata => DoesNotMatch,
            SubresourceIntegrityMetadata::IntegritySources(integrity_sources) => {
                if integrity_sources.is_empty() {
                    return DoesNotMatch;
                }
                // Step 6: For each source of integrity sources:
                for source in &integrity_sources {
                    // Step 6.1: If integrity expressions does not contain a source expression whose hash-algorithm
                    // is an ASCII case-insensitive match for source’s hash-algorithm,
                    // and whose base64-value is identical to source’s base64-value, return "Does Not Match".
                    //
                    // Note that the case-insensitivy is already handled in HashAlgorithm::from_name and therefore
                    // we can do a simple equals check here for both algorithm and value.
                    if !integrity_expressions.iter().any(|ex| ex == source) {
                        return DoesNotMatch;
                    }
                }
                // Step 7: Return "Matches".
                Matches
            }
        }
    }
    /// https://www.w3.org/TR/CSP/#match-request-to-source-list
    fn does_request_match_source_list(&self, request: &Request) -> MatchResult {
        self.does_url_match_source_list_in_origin_with_redirect_count(
            &request.url,
            &request.origin,
            request.redirect_count,
        )
    }
    /// https://www.w3.org/TR/CSP/#match-url-to-source-list
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
    /// https://www.w3.org/TR/CSP/#match-element-to-source-list
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
                    if let Some(captures) = NONCE_SOURCE_GRAMMAR.captures(expression) {
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
                if let Some(captures) = HASH_SOURCE_GRAMMAR.captures(expression) {
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
    /// https://www.w3.org/TR/CSP/#allow-all-inline
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
    /// https://www.w3.org/TR/CSP/#match-response-to-source-list
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
    /// https://www.w3.org/TR/CSP/#can-compile-strings
    fn does_a_source_list_allow_js_evaluation(&self) -> AllowResult {
        for expression in self.0.clone().into_iter().map(Borrow::borrow) {
            // Step 5.3: If source-list contains a source expression which is an ASCII case-insensitive match
            // for the string "'unsafe-eval'", then skip the following steps.
            if ascii_case_insensitive_match(expression, "'unsafe-eval'") {
                return AllowResult::Allows;
            }
        }
        AllowResult::DoesNotAllow
    }
    /// https://www.w3.org/TR/CSP/#can-compile-wasm-bytes
    fn does_a_source_list_allow_wasm_evaluation(&self) -> AllowResult {
        for expression in self.0.clone().into_iter().map(Borrow::borrow) {
            if ascii_case_insensitive_match(expression, "'unsafe-eval'") ||
                ascii_case_insensitive_match(expression, "'wasm-unsafe-eval'") {
                return AllowResult::Allows;
            }
        }
        AllowResult::DoesNotAllow
    }
}

/// https://www.w3.org/TR/CSP/#allow-all-inline
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AllowResult {
    Allows,
    DoesNotAllow,
}

/// https://www.w3.org/TR/CSP/#match-url-to-source-expression
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
        // Skip the first byte of the port capture to avoid the `:`.
        let port_part = captures.name("port").map(|port| &port.as_str()[1..]).unwrap_or("");
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
        if let Origin::Tuple(scheme, host, port) = origin {
            let hosts_are_the_same = Some(host) == url.host().map(|p| p.to_owned()).as_ref();
            let ports_are_the_same = Some(*port) == url.port();
            let origins_port_is_default_for_scheme = Some(*port) == default_port(scheme);
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

/// https://www.w3.org/TR/CSP/#match-hosts
fn host_part_match(pattern: &str, host: &str) -> MatchResult {
    debug_assert!(!host.is_empty());
    // Step 1. If host is not a domain, return "Does Not Match".
    if host.is_empty() {
        return DoesNotMatch;
    }
    if pattern.as_bytes()[0] == b'*' {
        // Step 2. If pattern is "*", return "Matches".
        if pattern.len() == 1 {
            return Matches;
        }
        // Step 3. If pattern starts with "*.":
        if pattern.as_bytes()[1] == b'.' {
            // Step 3.1 Let remaining be pattern with the leading U+002A (*) removed and ASCII lowercased.
            let remaining_pattern = &pattern[1..];
            if remaining_pattern.len() > host.len() {
                return DoesNotMatch;
            }
            let remaining_host = &host[(host.len()-remaining_pattern.len())..];
            debug_assert_eq!(remaining_host.len(), remaining_pattern.len());
            // Step 3.2. If host to ASCII lowercase ends with remaining, then return "Matches".
            if ascii_case_insensitive_match(remaining_pattern, remaining_host) {
                return Matches;
            }
            // Step 3.3 Return "Does Not Match".
            return DoesNotMatch;
        }
    }
    // Step 4. If pattern is not an ASCII case-insensitive match for host, return "Does Not Match".
    if !ascii_case_insensitive_match(pattern, host) {
        return DoesNotMatch;
    }
    static IPV4_ADDRESS_RULE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r#"([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\.([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\.([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\.([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])"#).unwrap());
    if IPV4_ADDRESS_RULE.is_match(pattern) && pattern != "127.0.0.1" {
        return DoesNotMatch;
    }
    // The spec uses the phrase "if A is an IPv6 address", without giving specific instructions on
    // how to tell if this is the case. In URLs, IPv6 addresses start with `[`, so let's go with that.
    // See https://url.spec.whatwg.org/#host-parsing
    if pattern.as_bytes()[0] == b'[' {
        return DoesNotMatch;
    }
    // Step 5. Return "Matches".
    Matches
}

/// https://www.w3.org/TR/CSP/#match-ports
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
    if port_b.is_empty() {
        if port_a == default_port_str(scheme_b) {
            return Matches;
        } else {
            return DoesNotMatch;
        }
    }
    DoesNotMatch
}

/// https://www.w3.org/TR/CSP/#match-paths
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
        debug_assert_eq!(path_list_a[path_list_a.len()-1], "");
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
    if let Origin::Tuple(scheme, _host, _port) = a {
        scheme_part_match(&scheme[..], b)
    } else {
        DoesNotMatch
    }
}

/// https://www.w3.org/TR/CSP/#match-schemes
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
            "sha256" | "Sha256" | "sHa256" | "shA256" | "SHa256" | "ShA256" | "sHA256" | "SHA256" => Some(Sha256),
            "sha384" | "Sha384" | "sHa384" | "shA384" | "SHa384" | "ShA384" | "sHA384" | "SHA384" => Some(Sha384),
            "sha512" | "Sha512" | "sHa512" | "shA512" | "SHa512" | "ShA512" | "sHA512" | "SHA512" => Some(Sha512),
            _ => None,
        }
    }
    pub fn apply(self, value: &str) -> String {
        use base64::Engine as _;
        let bytes = value.as_bytes();
        let standard = base64::engine::general_purpose::STANDARD;
        match self {
            HashAlgorithm::Sha256 => standard.encode(sha2::Sha256::digest(bytes)),
            HashAlgorithm::Sha384 => standard.encode(sha2::Sha384::digest(bytes)),
            HashAlgorithm::Sha512 => standard.encode(sha2::Sha512::digest(bytes)),
        }
    }
}

/// https://www.w3.org/TR/SRI/#integrity-metadata
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HashFunction {
    algorithm: HashAlgorithm,
    value: String,
    // The spec defines a third member, options, but defines no values.
}

/// https://www.w3.org/TR/SRI/#parse-metadata
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubresourceIntegrityMetadata {
    NoMetadata,
    IntegritySources(Vec<HashFunction>)
}

/// https://www.w3.org/TR/SRI/#the-integrity-attribute
/// This corresponds to the "hash-expression" grammar.
static SUBRESOURCE_METADATA_GRAMMAR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?P<algorithm>[sS][hH][aA](256|384|512))-(?P<value>[a-zA-Z0-9\+/\-_]+=*)"#).unwrap());

/// https://www.w3.org/TR/SRI/#parse-metadata
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

    #[test]
    pub fn prefetch_request_does_not_violate_policy() {
        let request = Request {
            url: Url::parse("https://www.notriddle.com/script.js").unwrap(),
            origin: Origin::Tuple("https".to_string(), url::Host::Domain("notriddle.com".to_owned()), 443),
            redirect_count: 0,
            destination: Destination::Script,
            initiator: Initiator::Prefetch,
            nonce: String::new(),
            integrity_metadata: String::new(),
            parser_metadata: ParserMetadata::None,
        };

        let p = Policy::parse("child-src 'self'", PolicySource::Header, PolicyDisposition::Enforce);

        let violation_result = p.does_request_violate_policy(&request);

        assert!(violation_result == Violates::DoesNotViolate);
    }

    #[test]
    pub fn prefetch_request_violates_policy() {
        let request = Request {
            url: Url::parse("https://www.notriddle.com/script.js").unwrap(),
            origin: Origin::Tuple("https".to_string(), url::Host::Domain("notriddle.com".to_owned()), 443),
            redirect_count: 0,
            destination: Destination::ServiceWorker,
            initiator: Initiator::None,
            nonce: String::new(),
            integrity_metadata: String::new(),
            parser_metadata: ParserMetadata::None,
        };

        let p = Policy::parse("default-src 'none'; script-src 'self' ", PolicySource::Header, PolicyDisposition::Enforce);

        let violation_result = p.does_request_violate_policy(&request);

        let expected_result = Violates::Directive(Directive { name: String::from("script-src"), value: vec![String::from("'self'")] });

        assert!(violation_result == expected_result);
    }

    #[test]
    pub fn prefetch_request_is_allowed_by_directive() {
        let request = Request {
            url: Url::parse("https://www.notriddle.com/script.js").unwrap(),
            origin: Origin::Tuple("https".to_string(), url::Host::Domain("notriddle.com".to_owned()), 443),
            redirect_count: 0,
            destination: Destination::Script,
            initiator: Initiator::Prefetch,
            nonce: String::new(),
            integrity_metadata: String::new(),
            parser_metadata: ParserMetadata::None,
        };

        let p = Policy::parse("default-src 'none'; child-src 'self'", PolicySource::Header, PolicyDisposition::Enforce);

        let violation_result = p.does_request_violate_policy(&request);

        assert!(violation_result == Violates::DoesNotViolate);
    }

    #[test]
    pub fn trusted_type_policy_is_valid() {
        let p = Policy::parse("trusted-types 'none'", PolicySource::Meta, PolicyDisposition::Enforce);
        assert!(p.is_valid());
        assert_eq!(p.directive_set[0].value, vec!["'none'".to_owned()]);
    }

    #[test]
    pub fn csp_list_is_valid() {
        let csp_list = CspList::parse("default-src 'none'; child-src 'self', trusted-types 'none'", PolicySource::Meta, PolicyDisposition::Enforce);
        assert!(csp_list.is_valid());
        assert_eq!(csp_list.0[1].directive_set[0].value, vec!["'none'".to_owned()]);
    }

    #[test]
    pub fn no_trusted_types_specified_allows_all_policies() {
        let csp_list = CspList::parse("default-src 'none'; child-src 'self'", PolicySource::Meta, PolicyDisposition::Enforce);
        assert!(csp_list.is_valid());
        let (check_result, violations) = csp_list.is_trusted_type_policy_creation_allowed("MyPolicy".to_owned(), vec![]);
        assert_eq!(check_result, CheckResult::Allowed);
        assert!(violations.is_empty());
    }

    #[test]
    pub fn none_does_not_allow_for_any_policy() {
        let csp_list = CspList::parse("trusted-types 'none'", PolicySource::Meta, PolicyDisposition::Enforce);
        assert!(csp_list.is_valid());
        let (check_result, violations) = csp_list.is_trusted_type_policy_creation_allowed("some-policy".to_owned(), vec![]);
        assert!(check_result == CheckResult::Blocked);
        assert_eq!(violations.len(), 1);
    }

    #[test]
    pub fn extra_none_allows_all_policies() {
        let csp_list = CspList::parse("trusted-types some-policy 'none'", PolicySource::Meta, PolicyDisposition::Enforce);
        assert!(csp_list.is_valid());
        let (check_result, violations) = csp_list.is_trusted_type_policy_creation_allowed("some-policy".to_owned(), vec![]);
        assert!(check_result == CheckResult::Allowed);
        assert!(violations.is_empty());
    }

    #[test]
    pub fn explicit_policy_named_is_allowed() {
        let csp_list = CspList::parse("trusted-types MyPolicy", PolicySource::Meta, PolicyDisposition::Enforce);
        assert!(csp_list.is_valid());
        let (check_result, violations) = csp_list.is_trusted_type_policy_creation_allowed("MyPolicy".to_owned(), vec![]);
        assert_eq!(check_result, CheckResult::Allowed);
        assert!(violations.is_empty());
    }

    #[test]
    pub fn other_policy_name_is_blocked() {
        let csp_list = CspList::parse("trusted-types MyPolicy", PolicySource::Meta, PolicyDisposition::Enforce);
        assert!(csp_list.is_valid());
        let (check_result, violations) = csp_list.is_trusted_type_policy_creation_allowed("MyOtherPolicy".to_owned(), vec![]);
        assert!(check_result == CheckResult::Blocked);
        assert_eq!(violations.len(), 1);
    }

    #[test]
    pub fn already_created_policy_is_blocked() {
        let csp_list = CspList::parse("trusted-types MyPolicy", PolicySource::Meta, PolicyDisposition::Enforce);
        assert!(csp_list.is_valid());
        let (check_result, violations) = csp_list.is_trusted_type_policy_creation_allowed("MyPolicy".to_owned(), vec!["MyPolicy".to_owned()]);
        assert!(check_result == CheckResult::Blocked);
        assert_eq!(violations.len(), 1);
    }

    #[test]
    pub fn already_created_policy_is_allowed_with_allow_duplicates() {
        let csp_list = CspList::parse("trusted-types MyPolicy 'allow-duplicates'", PolicySource::Meta, PolicyDisposition::Enforce);
        assert!(csp_list.is_valid());
        let (check_result, violations) = csp_list.is_trusted_type_policy_creation_allowed("MyPolicy".to_owned(), vec!["MyPolicy".to_owned()]);
        assert!(check_result == CheckResult::Allowed);
        assert!(violations.is_empty());
    }

    #[test]
    pub fn only_report_policy_issues_for_disposition_report() {
        let csp_list = CspList::parse("trusted-types MyPolicy", PolicySource::Meta, PolicyDisposition::Report);
        assert!(csp_list.is_valid());
        let (check_result, violations) = csp_list.is_trusted_type_policy_creation_allowed("MyPolicy".to_owned(), vec!["MyPolicy".to_owned()]);
        assert!(check_result == CheckResult::Allowed);
        assert_eq!(violations.len(), 1);
    }

    #[test]
    pub fn wildcard_allows_all_policies() {
        let csp_list = CspList::parse("trusted-types *", PolicySource::Meta, PolicyDisposition::Report);
        assert!(csp_list.is_valid());
        let (check_result, violations) = csp_list.is_trusted_type_policy_creation_allowed("MyPolicy".to_owned(), vec![]);
        assert!(check_result == CheckResult::Allowed);
        assert!(violations.is_empty());
    }

    #[test]
    pub fn violation_has_correct_directive() {
        let csp_list = CspList::parse("trusted-types MyPolicy", PolicySource::Meta, PolicyDisposition::Enforce);
        assert!(csp_list.is_valid());
        let (check_result, violations) = csp_list.is_trusted_type_policy_creation_allowed("MyOtherPolicy".to_owned(), vec![]);
        assert!(check_result == CheckResult::Blocked);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].directive, csp_list.0[0].directive_set[0]);
    }

    #[test]
    pub fn long_policy_name_is_truncated() {
        let csp_list = CspList::parse("trusted-types MyPolicy", PolicySource::Meta, PolicyDisposition::Enforce);
        assert!(csp_list.is_valid());
        let (check_result, violations) = csp_list.is_trusted_type_policy_creation_allowed("SuperLongPolicyNameThatExceeds40Characters".to_owned(), vec![]);
        assert!(check_result == CheckResult::Blocked);
        assert_eq!(violations.len(), 1);
        assert!(matches!(&violations[0].resource, ViolationResource::TrustedTypePolicy { sample } if sample == "SuperLongPolicyNameThatExceeds40Characte"));
    }
}
