#![doc(html_root_url = "https://docs.rs/content-security-policy/0.0.0")]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate if_chain;
extern crate lalrpop_util;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate url;

pub(crate) mod syntax;
pub(crate) mod check;

// TODO: Once the checker works, re-export its functionality and/or wrappers here.
