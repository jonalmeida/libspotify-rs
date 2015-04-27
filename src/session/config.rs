#[feature(libc)]
extern crate libc;

// use super::*;
use session::callbacks;
use libc::types::common::c95::c_void;

pub struct sp_session_config;
pub struct sp_session;

#[repr(C)]
pub struct session_config {
    api_version: libc::c_int,
    cache_location: *mut libc::c_char,
    settings_location: *mut libc::c_char,
    application_key: *mut c_void,
    application_key_size: usize,
    user_agent: *mut libc::c_char,
    callbacks: *mut libc::c_char,
    userdata: *mut c_void,
    compress_playlists: bool,
    dont_save_metadata_for_playlists: bool,
    initially_unload_playlists: bool,
    device_id: *mut libc::c_char,
    proxy: *mut libc::c_char,
    proxy_username: *mut libc::c_char,
    proxy_password: *mut libc::c_char,
    ca_certs_filename: *mut libc::c_char,
    tracefile: *mut libc::c_char,
}

#[link(name = "spotify")]
extern {
    fn sp_session_create(config: session_config , session: *mut sp_session)
                            -> ::error::Error;
}

//pub fn session_create<'a>(config: session::Config, session: *mut session)
//                            -> Result<::error::Error, &'a str> {
//    unsafe {
//        //Ok(sp_session_create(config, session))
//    }
//    Err(())
//}

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
