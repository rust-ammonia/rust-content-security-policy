#![doc(html_root_url = "https://docs.rs/content-security-policy/0.0.0")]

extern crate lalrpop_util;
extern crate regex;

pub mod parse;
pub mod types;

#[cfg(test)]
mod test{
    use super::parse::*;
    use super::types::*;
    #[test]
    fn parse_scheme_source() {
        assert_eq!(parse_SourceExpression("http:").unwrap(), Source::Scheme("http"));
        assert_eq!(parse_SourceExpression("magnet:").unwrap(), Source::Scheme("magnet"));
        assert_eq!(parse_SourceExpression("rust.doc:").unwrap(), Source::Scheme("rust.doc"));
        assert_eq!(parse_SourceExpression("h2:").unwrap(), Source::Scheme("h2"));
        assert_eq!(parse_SourceExpression("http+spdy:").unwrap(), Source::Scheme("http+spdy"));
        assert!(parse_SourceExpression(":").is_err());
        assert!(parse_SourceExpression("1:").is_err());
        assert!(parse_SourceExpression("::").is_err());
        assert!(parse_SourceExpression("http").is_err());
        // TODO: figure out how to make lalrpop throw an error on unexpected trailing characters.
        //assert!(parse_SourceExpression("htt:p").is_err());
    }
    #[test]
    fn parse_keyword_source() {
        assert_eq!(parse_SourceExpression("'self'").unwrap(), Source::Self_);
        assert_eq!(parse_SourceExpression("'unsafe-inline'").unwrap(), Source::UnsafeInline);
        assert_eq!(parse_SourceExpression("'unsafe-eval'").unwrap(), Source::UnsafeEval);
        assert_eq!(parse_SourceExpression("'strict-dynamic'").unwrap(), Source::StrictDynamic);
        assert_eq!(parse_SourceExpression("'unsafe-hashed-attributes'").unwrap(), Source::UnsafeHashedAttributes);
        assert!(parse_SourceExpression("'arbitrary'").is_err());
        assert!(parse_SourceExpression("arbitrary").is_err());
        assert!(parse_SourceExpression("self").is_err());
        // TODO: figure out how to make lalrpop throw an error on unexpected trailing characters.
        //assert!(parse_SourceExpression("'self'arbitrary").is_err());
    }
    #[test]
    fn parse_source_list() {
        assert_eq!(parse_SourceExpressionList("http: 'self'").unwrap(), vec![Source::Scheme("http"), Source::Self_]);
        assert_eq!(parse_SourceExpressionList("http:  'self'").unwrap(), vec![Source::Scheme("http"), Source::Self_]);
        assert_eq!(parse_SourceExpressionList("http: 'self' ").unwrap(), vec![Source::Scheme("http"), Source::Self_]);
        assert_eq!(parse_SourceExpressionList(" http: 'self'  ").unwrap(), vec![Source::Scheme("http"), Source::Self_]);
        assert_eq!(parse_SourceExpressionList("'none'").unwrap(), vec![]);
        assert!(parse_SourceExpressionList("self").is_err());
        assert!(parse_SourceExpressionList(" ").is_err());
        assert!(parse_SourceExpressionList("").is_err());
    }
    #[test]
    fn parse_directive_fetch() {
        assert_eq!(parse_Directive("script-src https: http: 'self'").unwrap(),
            Directive::ScriptSrc(vec![Source::Scheme("https"), Source::Scheme("http"), Source::Self_]));
        assert_eq!(parse_Directive("child-src https: http: 'self'").unwrap(),
            Directive::ChildSrc(vec![Source::Scheme("https"), Source::Scheme("http"), Source::Self_]));
        assert_eq!(parse_Directive("base-uri 'self'").unwrap(),
            Directive::BaseUri(vec![Source::Self_]));
        assert_eq!(parse_Directive("img-src 'none'").unwrap(),
            Directive::ImgSrc(vec![]));
        assert!(parse_Directive("worker-src").is_err());
    }
    #[test]
    fn parse_directive_plugin_types() {
        assert_eq!(parse_Directive("plugin-types image/png application/x-shockwave-flash application/vnd.ms-excel").unwrap(),
            Directive::PluginTypes(vec!["image/png", "application/x-shockwave-flash", "application/vnd.ms-excel"]));
        assert_eq!(parse_Directive("plugin-types image/png").unwrap(),
            Directive::PluginTypes(vec!["image/png"]));
        assert!(parse_Directive("plugin-types").is_err());
    }
    #[test]
    fn parse_directive_sandbox() {
        assert_eq!(parse_Directive("sandbox a b c").unwrap(),
            Directive::Sandbox);
        assert_eq!(parse_Directive("sandbox").unwrap(),
            Directive::Sandbox);
    }
}