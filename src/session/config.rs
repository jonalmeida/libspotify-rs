//#[feature(libc)]
//extern crate libc;

// use super::*;
use session::callbacks;

pub struct sp_session_config;
pub struct sp_session;

#[link(name = "spotify")]
extern {
    fn sp_session_create(config: sp_session_config , session: *mut sp_session) -> ::error::Error;
}

pub struct Config<T: callbacks::Callbacks> {
    api_version: isize,
    cache_location: String,
    settings_location: String,
    application_key: String,
    application_key_size: usize,
    user_agent: String,
    callbacks: Box<T>,
    //userdata: void*?
    compress_playlists: bool,
    dont_save_metadata_for_playlists: bool,
    initially_unload_playlists: bool,
    device_id: String,
    proxy: String,
    proxy_username: String,
    proxy_password: String,
    ca_certs_filename: String,
    tracefile: String,
}

pub fn session_create<'a>(config: sp_session_config, session: *mut sp_session) -> Result<::error::Error, &'a str> {
    unsafe {
        Ok(sp_session_create(config, session))
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
