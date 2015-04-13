#![feature(libc)]
extern crate libc;
use libc::*;
use std::ffi;
use std::str;

use std::env;
use std::fs::File;
use std::path::PathBuf;

mod session;

pub enum Error {
    /// No errors encountered.
    ErrorOk,
    /// The library version targeted does not match the one you claim you support.
    ErrorBadApiVersion,
    /// Initialization of library failed - are cache locations etc. valid?
    ErrorApiInitializationFailed,
    /// The track specified for playing cannot be played.
    ErrorTrackNotPlayable,
    /// The application key is invalid.
    ErrorBadApplicationKey,
    /// Login failed because of bad username and/or password.
    ErrorBadUsernameOrPassword,
    /// The specified username is banned.
    ErrorUserBanned,
    /// Cannot connect to the Spotify backend system.
    ErrorUnableToContactServer,
    /// Client is too old, library will need to be updated.
    ErrorClientTooOld,
    /// Some other error occurred, and it is permanent (e.g. trying to relogin will not help).
    ErrorOtherPermanent,
    /// The user agent string is invalid or too long.
    ErrorBadUserAgent,
    /// No valid callback registered to handle events.
    ErrorMissingCallback,
    /// Input data was either missing or invalid.
    ErrorInvalidIndata,
    /// Index out of range.
    ErrorIndexOutOfRange,
    /// The specified user needs a premium account.
    ErrorUserNeedsPremium,
    /// A transient error occurred.
    ErrorOtherTransient,
    /// The resource is currently loading.
    ErrorIsLoading,
    /// Could not find any suitable stream to play.
    ErrorNoStreamAvailable,
    ///Requested operation is not allowed.
    ErrorPermissionDenied,
    ///Target inbox is full.
    ErrorInboxIsFull,
    ///Cache is not enabled.
    ErrorNoCache,
    ///Requested user does not exist.
    ErrorNoSuchUser,
    ///No credentials are stored.
    ErrorNoCredentials,
    ///Network disabled.
    ErrorNetworkDisabled,
    ///Invalid device ID.
    ErrorInvalidDeviceId,
    ///Unable to open trace file.
    ErrorCantOpenTraceFile,
    ///This application is no longer allowed to use the Spotify service.
    ErrorApplicationBanned,
    ///Reached the device limit for number of tracks to download.
    ErrorOfflineTooManyTracks,
    ///Disk cache is full so no more tracks can be downloaded to offline mode.
    ErrorOfflineDiskCache,
    /// Offline key has expired, the user needs to go online again.
    ErrorOfflineExpired,
    /// This user is not allowed to use offline mode.
    ErrorOfflineNotAllowed,
    /// The license for this device has been lost. Most likely because the user used offline on three other device.
    ErrorOfflineLicenseLost,
    /// The Spotify license server does not respond correctly.
    ErrorOfflineLicenseError,
    /// A LastFM scrobble authentication error has occurred.
    ErrorLastfmAuthError,
    /// An invalid argument was specified.
    ErrorInvalidArgument,
    /// An operating system error.
    ErrorSystemFailure,
}

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
