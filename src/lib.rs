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
        assert!(parse::parse_SourceExpression("http").is_err());
        // TODO: figure out how to make lalrpop throw an error on unexpected trailing characters.
        //assert!(parse::parse_SourceExpression("htt:p").is_err());
    }
    #[test]
    fn parse_keyword_source() {
        assert_eq!(parse::parse_SourceExpression("'self'").unwrap(), types::Source::Self_);
        assert_eq!(parse::parse_SourceExpression("'unsafe-inline'").unwrap(), types::Source::UnsafeInline);
        assert_eq!(parse::parse_SourceExpression("'unsafe-eval'").unwrap(), types::Source::UnsafeEval);
        assert_eq!(parse::parse_SourceExpression("'strict-dynamic'").unwrap(), types::Source::StrictDynamic);
        assert_eq!(parse::parse_SourceExpression("'unsafe-hashed-attributes'").unwrap(), types::Source::UnsafeHashedAttributes);
        assert!(parse::parse_SourceExpression("'arbitrary'").is_err());
        assert!(parse::parse_SourceExpression("arbitrary").is_err());
        assert!(parse::parse_SourceExpression("self").is_err());
        // TODO: figure out how to make lalrpop throw an error on unexpected trailing characters.
        //assert!(parse::parse_SourceExpression("'self'arbitrary").is_err());
    }
}