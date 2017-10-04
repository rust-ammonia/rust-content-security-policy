mod directive;
mod media_type;
mod source_expression;
mod types;

#[cfg(test)]
mod test{
    use super::directive::*;
    use super::source_expression::*;
    use super::types::*;
    #[test]
    fn parse_scheme_source() {
        assert_eq!(parse_SourceExpression("http:").unwrap(), Source::Scheme("http"));
        assert_eq!(parse_SourceExpression("magnet:").unwrap(), Source::Scheme("magnet"));
        assert_eq!(parse_SourceExpression("rust.doc:").unwrap(), Source::Scheme("rust.doc"));
        assert_eq!(parse_SourceExpression("h2:").unwrap(), Source::Scheme("h2"));
        assert_eq!(parse_SourceExpression("http+spdy:").unwrap(), Source::Scheme("http+spdy"));
        assert!(parse_SourceExpression(":").is_err());
        assert!(parse_SourceExpression("::").is_err());
    }
    #[test]
    fn parse_keyword_source() {
        assert_eq!(parse_SourceExpression("'self'").unwrap(), Source::Self_);
        assert_eq!(parse_SourceExpression("'unsafe-inline'").unwrap(), Source::UnsafeInline);
        assert_eq!(parse_SourceExpression("'unsafe-eval'").unwrap(), Source::UnsafeEval);
        assert_eq!(parse_SourceExpression("'strict-dynamic'").unwrap(), Source::StrictDynamic);
        assert_eq!(parse_SourceExpression("'unsafe-hashed-attributes'").unwrap(), Source::UnsafeHashedAttributes);
        assert!(parse_SourceExpression("'arbitrary'").is_err());
    }
    #[test]
    fn parse_source_list() {
        assert_eq!(parse_SourceExpressionList("http: 'self'").unwrap(), vec![Source::Scheme("http"), Source::Self_]);
        assert_eq!(parse_SourceExpressionList("http:  'self'").unwrap(), vec![Source::Scheme("http"), Source::Self_]);
        assert_eq!(parse_SourceExpressionList("http: 'self' ").unwrap(), vec![Source::Scheme("http"), Source::Self_]);
        assert_eq!(parse_SourceExpressionList(" http: 'self'  ").unwrap(), vec![Source::Scheme("http"), Source::Self_]);
        assert_eq!(parse_SourceExpressionList("'none'").unwrap(), vec![]);
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
    #[test]
    fn parse_directive_set() {
        let d = parse_DirectiveSet("script-src https: 'self'; child-src 'none'; sandbox").unwrap();
        assert_eq!(d.script_src, Some(vec![Source::Scheme("https"), Source::Self_]));
        assert_eq!(d.child_src, Some(vec![]));
        assert_eq!(d.img_src, None);
        assert!(d.sandbox);
    }
    #[test]
    fn parse_directive_set_precedence() {
        let d = parse_DirectiveSet("script-src https: 'self'; script-src 'none'").unwrap();
        assert_eq!(d.script_src, Some(vec![Source::Scheme("https"), Source::Self_]));
        assert!(!d.sandbox);
        let d = parse_DirectiveSet("script-src 'none'; script-src https: 'self'").unwrap();
        assert_eq!(d.script_src, Some(vec![]));
        assert!(!d.sandbox);
    }
    #[test]
    fn parse_url_directive() {
        let d = parse_DirectiveSet("script-src vnd.google.spdy://google.com/secure https://google.com https://google.com/ google.com google.com:8080 google.com:8080/secure https://google.com:8080/secure https://google.com:8080 google.com:* *.google.com:* 'self'").unwrap();
        let h1 = Source::Host("vnd.google.spdy://google.com/secure");
        let h2 = Source::Host("https://google.com");
        let h3 = Source::Host("https://google.com/");
        let h4 = Source::Host("google.com");
        let h5 = Source::Host("google.com:8080");
        let h6 = Source::Host("google.com:8080/secure");
        let h7 = Source::Host("https://google.com:8080/secure");
        let h8 = Source::Host("https://google.com:8080");
        let h9 = Source::Host("google.com:*");
        let h10 = Source::Host("*.google.com:*");
        assert_eq!(d.script_src, Some(vec![h1, h2, h3, h4, h5, h6, h7, h8, h9, h10, Source::Self_]))
    }
    #[test]
    fn parse_url_directive_percent_encoded() {
        let d = parse_DirectiveSet("script-src notriddle.com/%20/").unwrap();
        assert_eq!(d.script_src, Some(vec![Source::Host("notriddle.com/%20/")]))
    }
    #[test]
    fn parse_form_action() {
        let d = parse_DirectiveSet("form-action notriddle.com/%20/").unwrap();
        assert_eq!(d.form_action, Some(vec![Source::Host("notriddle.com/%20/")]))
    }
    #[test]
    fn parse_no_disown_opener() {
        let d = parse_DirectiveSet("form-action notriddle.com/%20/").unwrap();
        assert!(!d.disown_opener);
    }
    #[test]
    fn parse_disown_opener() {
        let d = parse_DirectiveSet("disown-opener;form-action notriddle.com/%20/").unwrap();
        assert!(d.disown_opener);
    }
    #[test]
    fn parse_ancestor_source() {
        assert_eq!(parse_AncestorSource("'self'").unwrap(), Ancestor::Self_);
        assert!(parse_AncestorSource("'unsafe-inline'").is_err());
    }
    #[test]
    fn parse_frame_ancestors() {
        let d = parse_DirectiveSet("frame-ancestors notriddle.com/%20/").unwrap();
        assert_eq!(d.frame_ancestors, Some(vec![Ancestor::Host("notriddle.com/%20/")]))
    }
}