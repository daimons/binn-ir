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
pub const VERSION: &'static str = "0.4.0";

/// # Release date (year/month/day)
#[allow(dead_code)]
pub const RELEASE_DATE: (u16, u8, u8) = (2019, 6, 1);

/// # Wrapper for format!(), which prefixes your message with: ::TAG, module_path!(), line!()
macro_rules! __ { ($($arg: tt)+) => {
    format!("[{}][{}-{}] {}", crate::TAG, module_path!(), line!(), format!($($arg)+))
};}

/// # Wrapper for format!(), which wraps your message inside 'bold' tag
macro_rules! __b { ($($arg: tt)+) => {
    format!("\x1b[1m{}\x1b[m", format!($($arg)+))
};}

/// # Wrapper for format!(), which wraps your message inside a warning 'red color' tag
macro_rules! __w { ($($arg: tt)+) => {
    format!("\x1b[38;5;1m{}\x1b[39m", format!($($arg)+))
};}

/// # Wrapper for println!() + __!()
macro_rules! __p { ($($arg: tt)+) => {
    println!("{}", __!($($arg)+));
};}

/// # Checks to see if given file descriptor is a char-device
#[cfg(unix)]
macro_rules! __is_char_device { ($fd: expr) => {{
    use ::std::os::unix::fs::FileTypeExt;
    use ::std::os::unix::io::RawFd;
    use ::std::path::PathBuf;

    let fd: RawFd = $fd;
    match PathBuf::from(format!("/proc/{}/fd/{}", std::process::id(), fd)).metadata().map(|m| m.file_type()) {
        Ok(ft) => ft.is_char_device(),
        Err(_) => false,
    }
}};}

/// # Wrapper for __p!() + __b!() + __!()
macro_rules! __pp { ($($arg: tt)+) => {
    match match cfg!(unix) {
        true => {
            use ::std::os::unix::io::AsRawFd;
            __is_char_device!(::std::io::stdout().as_raw_fd())
        },
        false => true,
    } {
        true => __p!("{}", __b!("{}", __!($($arg)+))),
        false => __p!($($arg)+),
    };
};}

/// # Wrapper for eprintln!() + __!()
macro_rules! __e { ($($arg: tt)+) => {
    eprintln!("{}", __!($($arg)+));
};}

/// # Wrapper for __e!() + __w!() + __!()
macro_rules! __ee { ($($arg: tt)+) => {
    match match cfg!(unix) {
        true => {
            use ::std::os::unix::io::AsRawFd;
            __is_char_device!(::std::io::stderr().as_raw_fd())
        },
        false => true,
    } {
        true => __e!("{}", __w!("{}", __!($($arg)+))),
        false => __e!($($arg)+),
    };
};}
