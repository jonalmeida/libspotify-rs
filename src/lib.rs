#![feature(libc)]
extern crate libc;
use libc::*;
use std::ffi;
use std::str;

use std::env;
use std::fs::File;
use std::path::PathBuf;

mod session;
mod playlist;
mod error;

#[link(name = "spotify")]
extern {
    fn sp_build_id() -> *const c_char;
}

/// Return the libspotify build ID
///
/// This might be useful to have available for display somewhere in your user interface.
///
/// # Examples
/// ```
/// use spotify;
///
/// println!("{}", spotify::build_id());
///
/// // 12.1.51.g86c92b43 Release Linux-x86_64
/// ```
pub fn build_id() -> String {
    unsafe {
        let slice = ffi::CStr::from_ptr(sp_build_id());
        str::from_utf8(slice.to_bytes()).unwrap().to_string()
    }
}

/// Gets the cache directory used for storing the cache handled by libspotify.
fn get_cache_dir() -> PathBuf {
    let mut dir = env::temp_dir();
    dir.push("rusty-spot");
    dir
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_build_id() {
        println!("{}", build_id());
        assert!(!build_id().is_empty());
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn cache_dir() {
        let dir = super::get_cache_dir();
        assert_eq!("/tmp/rusty-spot", dir.into_os_string().into_string().unwrap());
    }
}
