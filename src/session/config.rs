#[feature(libc)]
extern crate libc;

// use super::*;
use session::callbacks;
use libc::types::common::c95::c_void;
use std::ffi::CString;
use std::ptr;

pub struct sp_session_config;
pub struct sp_session;

#[repr(C)]
pub struct session_config {
    api_version: libc::c_int,
    cache_location: *const libc::c_char,
    settings_location: *const libc::c_char,
    application_key: *const c_void,
    application_key_size: libc::c_int,
    user_agent: *const libc::c_char,
    //callbacks: extern fn(),
    userdata: *mut c_void,
    compress_playlists: bool,
    dont_save_metadata_for_playlists: bool,
    initially_unload_playlists: bool,
    device_id: *const libc::c_char,
    proxy: *const libc::c_char,
    proxy_username: *const libc::c_char,
    proxy_password: *const libc::c_char,
    ca_certs_filename: *const libc::c_char,
    tracefile: *const libc::c_char,
}

#[link(name = "spotify")]
extern {
    fn sp_session_create(config: session_config,
                         session: *mut sp_session)
                         -> ::error::Error;
}

pub struct Config<'a> {
    api_version: isize,
    cache_location: &'a[u8],
    settings_location: &'a[u8],
    application_key: &'a str,
    user_agent: &'a str,
    //callbacks: Option<callbacks::Callbacks>,
    userdata: &'a str,
    compress_playlists: bool,
    dont_save_metadata_for_playlists: bool,
    initially_unload_playlists: bool,
    device_id: &'a str,
    proxy: &'a str,
    proxy_username: &'a str,
    proxy_password: &'a str,
    ca_certs_filename: &'a str,
    tracefile: &'a str,
}

pub fn session_create<'a> (config: Config<'a>, session: *mut sp_session)
                          -> Result<::error::Error, String> {
    let c_config_ref = session_config {
        api_version:        config.api_version as libc::c_int,
        cache_location:     CString::new(config.cache_location).unwrap().as_ptr(),
        settings_location:  CString::new(config.settings_location).unwrap().as_ptr(),
        application_key:    CString::new(config.application_key).unwrap().as_ptr() as *mut c_void,
        application_key_size: config.application_key.len() as libc::c_int,
        user_agent:         CString::new(config.user_agent).unwrap().as_ptr(),
        userdata:           0isize as *mut c_void,
        compress_playlists: config.compress_playlists,
        dont_save_metadata_for_playlists: config.dont_save_metadata_for_playlists,
        initially_unload_playlists: config.initially_unload_playlists,
        device_id:          CString::new(config.device_id).unwrap().as_ptr(),
        proxy:              CString::new(config.proxy).unwrap().as_ptr(),
        proxy_username:     CString::new(config.proxy_username).unwrap().as_ptr(),
        proxy_password:     CString::new(config.proxy_password).unwrap().as_ptr(),
        ca_certs_filename:  CString::new(config.ca_certs_filename).unwrap().as_ptr(),
        tracefile:          CString::new(config.tracefile).unwrap().as_ptr(),
    };

    let c_error_ref = unsafe {
        sp_session_create(c_config_ref, session)
    };

    match c_error_ref {
        _ => Ok(c_error_ref),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_session_create() {
        /*
        let spconfig: sp_session_config = {
            api_version = 12,
            cache_location = "tmp",
            settings_location = "tmp",
            application_key = "",
            application_key_size = 0, // Set in main()
            user_agent = "spotify-jukebox-example",
            callbacks = 0,
            0,
        };
        */
    }

}
