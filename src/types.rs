use std::borrow::Cow;
use std::path::Path;

pub type SStr = Cow<'static, str>;
pub type SPath = Cow<'static, Path>;