// License: see LICENSE file at root directory of `master` branch

//! # Error

use {
    alloc::{
        borrow::Cow,
        fmt::{self, Display, Formatter},
    },
};

#[cfg(feature="std")]
use {
    alloc::string::ToString,
    std::io,
};

/// # Error
#[derive(Debug)]
pub struct Error {
    line: u32,
    module_path: &'static str,
    msg: Option<Cow<'static, str>>,
}

impl Error {

    /// # Makes new instance
    pub (crate) const fn new(line: u32, module_path: &'static str, msg: Option<Cow<'static, str>>) -> Self {
        Self {
            line,
            module_path,
            msg,
        }
    }

    /// # Line
    pub const fn line(&self) -> u32 {
        self.line
    }

    /// # Module path
    pub const fn module_path(&self) -> &str {
        self.module_path
    }

    /// # Error message
    pub fn msg(&self) -> Option<&str> {
        self.msg.as_deref()
    }

}

impl Display for Error {

    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self.msg.as_ref() {
            Some(msg) => write!(
                f, "[{tag}][{module_path}-{line}] {msg}", tag=crate::TAG, line=self.line, module_path=self.module_path, msg=msg,
            ),
            None => write!(f, "[{tag}][{module_path}-{line}]", tag=crate::TAG, line=self.line, module_path=self.module_path),
        }
    }

}

#[cfg(feature="std")]
impl From<Error> for io::Error {

    fn from(err: Error) -> Self {
        io::Error::new(io::ErrorKind::Other, err.to_string())
    }

}
