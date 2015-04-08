#![feature(libc)]
extern crate libc;
use libc::*;
use std::ffi;
use std::str;


#[link(name = "spotify")]
extern {
    fn sp_build_id() -> *const c_char;
}

pub fn build_id() -> String {
    unsafe {
        let slice = ffi::CStr::from_ptr(sp_build_id());
        str::from_utf8(slice.to_bytes()).unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_build_id() {
        assert!(!build_id().is_empty());
    }
}
