use std::borrow::Cow;
use std::path::Path;

pub type SStr = Cow<'static, str>;
pub type SPath = Cow<'static, Path>;

pub enum SS {
    SStr(SStr),
    SPath(SPath),
}

impl AsRef<Path> for SS {
    fn as_ref(&self) -> &Path {
        match self {
            SS::SStr(s) => s.as_ref().as_ref(),
            SS::SPath(p) => p.as_ref(),
        }
    }
}
