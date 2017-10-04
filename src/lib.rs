#![doc(html_root_url = "https://docs.rs/content-security-policy/0.0.0")]

extern crate lalrpop_util;
extern crate regex;

mod syntax;
mod check;

// TODO: Once the checker works, re-export its functionality and/or wrappers here.
