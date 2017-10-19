#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectiveSet<'a> {
  pub child_src: Option<Vec<Source<'a>>>,
  pub connect_src: Option<Vec<Source<'a>>>,
  pub default_src: Option<Vec<Source<'a>>>,
  pub font_src: Option<Vec<Source<'a>>>,
  pub frame_src: Option<Vec<Source<'a>>>,
  pub img_src: Option<Vec<Source<'a>>>,
  pub manifest_src: Option<Vec<Source<'a>>>,
  pub media_src: Option<Vec<Source<'a>>>,
  pub object_src: Option<Vec<Source<'a>>>,
  pub script_src: Option<Vec<Source<'a>>>,
  pub style_src: Option<Vec<Source<'a>>>,
  pub worker_src: Option<Vec<Source<'a>>>,
  pub base_uri: Option<Vec<Source<'a>>>,
  pub plugin_types: Option<Vec<&'a str>>,
  pub sandbox: bool,
  pub disown_opener: bool,
  pub form_action: Option<Vec<Source<'a>>>,
  pub frame_ancestors: Option<Vec<Ancestor<'a>>>,
  pub report_uri: Option<Vec<&'a str>>,
  pub report_to: Option<&'a str>,
}

impl<'a> DirectiveSet<'a> {
  pub fn new() -> Self {
    DirectiveSet {
      child_src: None,
      connect_src: None,
      default_src: None,
      font_src: None,
      frame_src: None,
      img_src: None,
      manifest_src: None,
      media_src: None,
      object_src: None,
      script_src: None,
      style_src: None,
      worker_src: None,
      base_uri: None,
      plugin_types: None,
      sandbox: false,
      disown_opener: false,
      form_action: None,
      frame_ancestors: None,
      report_uri: None,
      report_to: None,
    }
  }

  pub fn with(directive: Directive<'a>) -> Self {
    let mut s = DirectiveSet::new();
    s.add_directive(directive);
    s
  }

  pub fn add_directive(&mut self, directive: Directive<'a>) {
    match directive {
      Directive::ChildSrc(x) => self.child_src = Some(x),
      Directive::ConnectSrc(x) => self.connect_src = Some(x),
      Directive::DefaultSrc(x) => self.default_src = Some(x),
      Directive::FontSrc(x) => self.font_src = Some(x),
      Directive::FrameSrc(x) => self.frame_src = Some(x),
      Directive::ImgSrc(x) => self.img_src = Some(x),
      Directive::ManifestSrc(x) => self.manifest_src = Some(x),
      Directive::MediaSrc(x) => self.media_src = Some(x),
      Directive::ObjectSrc(x) => self.object_src = Some(x),
      Directive::ScriptSrc(x) => self.script_src = Some(x),
      Directive::StyleSrc(x) => self.style_src = Some(x),
      Directive::WorkerSrc(x) => self.worker_src = Some(x),
      Directive::BaseUri(x) => self.base_uri = Some(x),
      Directive::PluginTypes(x) => self.plugin_types = Some(x),
      Directive::Sandbox => self.sandbox = true,
      Directive::DisownOpener => self.disown_opener = true,
      Directive::FormAction(x) => self.form_action = Some(x),
      Directive::FrameAncestors(x) => self.frame_ancestors = Some(x),
      Directive::ReportUri(x) => self.report_uri = Some(x),
      Directive::ReportTo(x) => self.report_to = Some(x),
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Directive<'a> {
  ChildSrc(Vec<Source<'a>>),
  ConnectSrc(Vec<Source<'a>>),
  DefaultSrc(Vec<Source<'a>>),
  FontSrc(Vec<Source<'a>>),
  FrameSrc(Vec<Source<'a>>),
  ImgSrc(Vec<Source<'a>>),
  ManifestSrc(Vec<Source<'a>>),
  MediaSrc(Vec<Source<'a>>),
  ObjectSrc(Vec<Source<'a>>),
  ScriptSrc(Vec<Source<'a>>),
  StyleSrc(Vec<Source<'a>>),
  WorkerSrc(Vec<Source<'a>>),
  BaseUri(Vec<Source<'a>>),
  PluginTypes(Vec<&'a str>),
  Sandbox,
  DisownOpener,
  FormAction(Vec<Source<'a>>),
  FrameAncestors(Vec<Ancestor<'a>>),
  ReportUri(Vec<&'a str>),
  ReportTo(&'a str),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Source<'a> {
  Scheme(&'a str),
  Host(&'a str),
  Nonce(&'a str),
  Sha256(&'a str),
  Sha384(&'a str),
  Sha512(&'a str),
  Self_,
  UnsafeInline,
  UnsafeEval,
  StrictDynamic,
  UnsafeHashedAttributes,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Ancestor<'a> {
  Scheme(&'a str),
  Host(&'a str),
  Self_,
}
