// License: see LICENSE file at root directory of `master` branch

//! # Usage
//!
//! This module requires you to define a constant named `::TAG` at your root crate.
//!
//! It's recommended to include crate code name and version name in `::TAG`.

/// # Unique universally identifier of this kit
#[allow(dead_code)]
pub const UUID: &'static str = "3ec2df31-b5fb-49a9-b91f-de63ec69a058";

/// # Version
#[allow(dead_code)]
pub const VERSION: &'static str = "0.0.1";

/// # Release date (year/month/day)
#[allow(dead_code)]
pub const RELEASE_DATE: (u16, u8, u8) = (2018, 7, 27);

/// # Wrapper for format!(), which prefixes your message with: ::TAG, file!(), line!()
macro_rules! __ { ($($arg: tt)+) => {
    format!("[{}][{}-{}] {}", ::TAG, file!(), line!(), format!($($arg)+))
};}

/// # Wrapper for println!() + __!()
macro_rules! __p { ($($arg: tt)+) => {
    println!("{}", __!($($arg)+));
};}

/// # Wrapper for eprintln!() + __!()
macro_rules! __e { ($($arg: tt)+) => {
    eprintln!("{}", __!($($arg)+));
};}
