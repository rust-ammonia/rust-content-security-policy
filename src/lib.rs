#![doc(html_root_url = "https://docs.rs/content-security-policy/0.0.0")]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate if_chain;
extern crate lalrpop_util;
#[macro_use] extern crate lazy_static;
extern crate regex;
#[macro_use] extern crate rental;
extern crate url;

pub(crate) mod syntax;
pub(crate) mod check;

use std::error::Error as StdError;
use std::fmt::{self, Debug, Display};

pub use check::Resource;
pub use url::Url;

/// A Content-Security-Policy Directive Set
pub struct Csp<'a> {
    policies: Vec<check::Policy<'a>>,
}

impl<'a> Csp<'a> {
    pub fn parse(s: &'a str) -> Result<Csp<'a>, Error> {
        let syntax = match syntax::parse(s) {
            Ok(syntax) => syntax,
            Err(_) => return Err(Error{_p: ()}),
        };
        let policies = syntax.into_iter().map(|directive_set| {
            let mut policy = check::Policy::new();
            // TODO: implement
            policy
        }).collect();
        Ok(Csp {
            policies: policies,
        })
    }
    pub fn check_url(&self, resource: Resource, url: &Url, is_redirect: bool) -> bool {
        // TODO: loop
        self.policies[0].check_url(resource, url, is_redirect).unwrap()
    }
}

rental!{
    mod rent {
        #[rental]
        pub struct CspOwner {
            buf: String,
            csp: super::Csp<'buf>,
        }
    }
}

/// A Content-Security-Policy Directive Set
pub struct CspOwned {
    i: rent::CspOwner,
}

impl CspOwned {
    pub fn parse(s: String) -> Result<CspOwned, Error> {
        let owner = match rent::CspOwner::try_new(s, |s| Csp::parse(s)) {
            Ok(owner) => owner,
            Err(rental::TryNewError(e, _)) => return Err(e),
        };
        Ok(CspOwned {
            i: owner
        })
    }
    pub fn into_string(self) -> String {
        self.i.into_head()
    }
    pub fn check_url(&self, resource: Resource, url: &Url, is_redirect: bool) -> bool {
        self.i.rent(|csp| csp.check_url(resource, url, is_redirect))
    }
}

#[derive(Clone, Debug)]
pub struct Error {
    _p: (),
}

impl Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Failed to parse content-security-policy")
    }
}

impl StdError for Error {
    fn description(&self) -> &'static str {
        "Failed to parse content-security-policy"
    }
}
