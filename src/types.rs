#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Source<'a> {
  Scheme(&'a str),
  Self_,
  UnsafeInline,
  UnsafeEval,
  StrictDynamic,
  UnsafeHashedAttributes,
}