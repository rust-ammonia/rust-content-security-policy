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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Source<'a> {
  Scheme(&'a str),
  Self_,
  UnsafeInline,
  UnsafeEval,
  StrictDynamic,
  UnsafeHashedAttributes,
}