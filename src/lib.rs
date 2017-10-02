#![doc(html_root_url = "https://docs.rs/content-security-policy/0.0.0")]

extern crate lalrpop_util;
extern crate regex;

pub mod parse;
pub mod types;

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn parse_scheme_source() {
        assert_eq!(parse::parse_SourceExpression("http:").unwrap(), types::Source::Scheme("http"));
        assert_eq!(parse::parse_SourceExpression("magnet:").unwrap(), types::Source::Scheme("magnet"));
        assert_eq!(parse::parse_SourceExpression("rust.doc:").unwrap(), types::Source::Scheme("rust.doc"));
        assert_eq!(parse::parse_SourceExpression("h2:").unwrap(), types::Source::Scheme("h2"));
        assert_eq!(parse::parse_SourceExpression("http+spdy:").unwrap(), types::Source::Scheme("http+spdy"));
        assert!(parse::parse_SourceExpression(":").is_err());
        assert!(parse::parse_SourceExpression("1:").is_err());
        assert!(parse::parse_SourceExpression("::").is_err());
        // TODO: figure out how to make lalrpop throw an error on unexpected trailing characters.
        //assert!(parse::parse_SourceExpression("htt:p").is_err());
    }
}