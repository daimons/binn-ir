// License: see LICENSE file at root directory of `master` branch

//! # Error

use alloc::{
    fmt::{self, Display, Formatter},
    string::String,
};

#[cfg(feature="std")]
use std::io;

/// # Error
#[derive(Debug)]
pub struct Error {

    /// # Message
    msg: String,

}

impl Error {

    /// # Error message
    pub fn msg(&self) -> &str {
        &self.msg
    }

}

impl From<String> for Error {

    fn from(msg: String) -> Self {
        Self { msg }
    }

}

impl Display for Error {

    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.msg)
    }

}

#[cfg(feature="std")]
impl From<Error> for io::Error {

    fn from(err: Error) -> Self {
        io::Error::new(io::ErrorKind::Other, err.msg)
    }

}
