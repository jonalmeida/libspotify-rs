#[feature(libc)]
extern crate libc;

use std::default::Default;
use super::*;

struct session;

//TODO: implement
struct audio_buffer_stats;
struct audioformat;

pub trait Callbacks_f<T> {
    /// Called when login has been processed and was successful
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [in]    error       One of the following errors, from error ERROR_OK
    ///                         ERROR_CLIENT_TOO_OLD ERROR_UNABLE_TO_CONTACT_SERVER
    ///                         ERROR_BAD_USERNAME_OR_PASSWORD ERROR_USER_BANNED ERROR_USER_NEEDS_PREMIUM
    ///                         ERROR_OTHER_TRANSIENT ERROR_OTHER_PERMANENT
    fn logged_in(s: &session, error: ::error::Error);

    /// Called when logout has been processed. Either called explicitly if you initialize a logout
    /// operation, or implicitly if there is a permanent connection error
    ///
    /// Parameters:
    ///     [in]    session     Session
    fn logged_out(s: &session);

    /// Called whenever metadata has been updated
    ///
    /// If you have metadata cached outside of libspotify, you should purge your caches and fetch
    /// new versions.
    ///
    /// Parameters:
    ///     [in]    session     Session
    fn metadata_updated(s: &session);

    /// Called when there is a connection error, and the library has problems reconnecting to the
    /// Spotify service. Could be called multiple times (as long as the problem is present)
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [in]    error       One of the following errors, from error ERROR_OK
    ///                         ERROR_CLIENT_TOO_OLD ERROR_UNABLE_TO_CONTACT_SERVER
    ///                         ERROR_BAD_USERNAME_OR_PASSWORD ERROR_USER_BANNED ERROR_USER_NEEDS_PREMIUM
    ///                         ERROR_OTHER_TRANSIENT ERROR_OTHER_PERMANENT
    ///
    fn connection_error(s: &session, error: ::error::Error);

    /// Called when the access point wants to display a message to the user
    ///
    /// In the desktop client, these are shown in a blueish toolbar just below the search box.
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [in]    message     String in UTF-8 format.
    fn message_to_user(s: &session, message: String);

    ///
    ///
    /// Called when processing needs to take place on the main thread.
    ///
    /// You need to call session_process_events() in the main thread to get libspotify to do
    /// more work. Failure to do so may cause request timeouts, or a lost connection.
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///
    /// Note:
    ///     This function is called from an internal session thread - you need to have proper
    ///     synchronization!
    ///
    fn notify_main_thread(s: &session);


    /// Called when there is decompressed audio data available.
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [in]    format  Audio format descriptor audioformat
    ///     [in]    frames  Points to raw PCM data as described by format
    ///     [in]    num_frames  Number of available samples in frames. If this is 0, a discontinuity has occurred (such as after a seek). The application should flush its audio fifos, etc.
    ///
    /// Returns:
    ///     Number of frames consumed. This value can be used to rate limit the output from the library if your output buffers are saturated. The library will retry delivery in about 100ms.
    ///
    /// Note:
    ///     This function is called from an internal session thread - you need to have proper synchronization!
    ///     This function must never block. If your output buffers are full you must return 0 to signal that the library should retry delivery in a short while.


    fn music_delivery(s: &session, format: &audioformat, frames: &T, num_frames: isize) -> isize;

    /// Music has been paused because an account only allows music to be played from one location
    /// simultaneously.
    ///
    /// Note:
    ///     When this callback is invoked the application should behave just as if the user pressed
    ///     the pause button. The application should also display a message to the user indicating
    ///     the playback has been paused because another application is playing using the same
    ///     account.
    ///     IT MUST NOT automatically resume playback but must instead wait for the user to
    ///     press play.
    ///
    /// Parameters:
    ///     [in]    session     Session
    fn play_token_lost(s: &session);

    /// Logging callback.
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [in]    data        Log data
    fn log_message(s: &session, data: &str);

    /// End of track. Called when the currently played track has reached its end.
    ///
    /// Note:
    ///     This function is invoked from the main thread
    ///
    /// Parameters:
    ///     [in]    session     Session
    fn end_of_track(s: &session);

    /// Streaming error. Called when streaming cannot start or continue.
    ///
    /// Note:
    ///     This function is invoked from the main thread
    ///
    /// Parameters:
    ///    [in]    session      Session
    ///    [in]    error        One of the following errors, from error
    ///                         ERROR_NO_STREAM_AVAILABLE ERROR_OTHER_TRANSIENT ERROR_OTHER_PERMANENT
    fn streaming_error(s: &session, error: ::error::Error);

    /// Called after user info (anything related to user objects) have been updated.
    ///
    /// Parameters:
    ///     [in]    session     Session
    fn userinfo_updated(s: &session);

    /// Called when audio playback should start
    ///
    /// Note:
    ///     For this to work correctly the application must also implement get_audio_buffer_stats()
    ///     This function is called from an internal session thread - you need to have proper
    ///     synchronization!
    ///     This function must never block.
    ///
    /// Parameters:
    ///     [in]    session     Session
    fn start_playback(s: &session);

    /// Called when audio playback should stop
    ///
    /// Note:
    ///     For this to work correctly the application must also implement get_audio_buffer_stats()
    ///     This function is called from an internal session thread - you need to have proper
    ///     synchronization!
    ///     This function must never block.
    ///
    /// Parameters:
    ///     [in]    session     Session
    fn stop_playback(s: &session);

    /// Called to query application about its audio buffer
    ///
    /// Note:
    ///     This function is called from an internal session thread - you need to have proper
    ///     synchronization!
    ///         This function must never block.
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [out]   stats       Stats struct to be filled by application
    fn get_audio_buffer_stats(s: &session, stats: &audio_buffer_stats);

    /// Called when offline synchronization status is updated
    ///
    /// Parameters:
    ///     [in]    session     Session
    fn offline_status_updated(s: &session);

    /// Called when offline synchronization status is updated
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [in]    error       Offline error. Will be ERROR_OK if the offline synchronization
    ///                         error state has cleared
    fn offline_error(s: &session, error: ::error::Error);

    /// Called when storable credentials have been updated, usually called when we have connected
    /// to the AP.
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [in]    blob        Blob is a null-terminated string which contains an encrypted token
    ///                         that can be stored safely on disk instead of storing plaintext passwords.
    fn credentials_blob_updated(s: &session, blob: &str);

    /// Called when the connection state has updated - such as when logging in, going offline, etc.
    ///
    /// Parameters:
    ///     [in]    session     Session
    fn connectionstate_updated(s: &session);

    /// Called when there is a scrobble error event
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [in]    error       Scrobble error. Currently ERROR_LASTFM_AUTH_ERROR.
    fn scrobble_error(s: &session, error: ::error::Error);

    /// Called when there is a change in the private session mode
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [in]    isPrivate   True if in private session, false otherwhise
    fn private_session_mode_changed(session: &session, is_private: bool);
}

#[repr(C)]
pub struct Callbacks {
    /// Called when login has been processed and was successful
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [in]    error       One of the following errors, from error ERROR_OK
    ///                         ERROR_CLIENT_TOO_OLD ERROR_UNABLE_TO_CONTACT_SERVER
    ///                         ERROR_BAD_USERNAME_OR_PASSWORD ERROR_USER_BANNED ERROR_USER_NEEDS_PREMIUM
    ///                         ERROR_OTHER_TRANSIENT ERROR_OTHER_PERMANENT
    logged_in: extern fn(&session, ::error::Error),

    /// Called when logout has been processed. Either called explicitly if you initialize a logout
    /// operation, or implicitly if there is a permanent connection error
    ///
    /// Parameters:
    ///     [in]    session     Session
    logged_out: extern fn(s: &session),

    /// Called whenever metadata has been updated
    ///
    /// If you have metadata cached outside of libspotify, you should purge your caches and fetch
    /// new versions.
    ///
    /// Parameters:
    ///     [in]    session     Session
    metadata_updated: extern fn(s: &session),

    /// Called when there is a connection error, and the library has problems reconnecting to the
    /// Spotify service. Could be called multiple times (as long as the problem is present)
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [in]    error       One of the following errors, from error ERROR_OK
    ///                         ERROR_CLIENT_TOO_OLD ERROR_UNABLE_TO_CONTACT_SERVER
    ///                         ERROR_BAD_USERNAME_OR_PASSWORD ERROR_USER_BANNED ERROR_USER_NEEDS_PREMIUM
    ///                         ERROR_OTHER_TRANSIENT ERROR_OTHER_PERMANENT
    ///
    connection_error: extern fn(s: &session, error: ::error::Error),

    /// Called when the access point wants to display a message to the user
    ///
    /// In the desktop client, these are shown in a blueish toolbar just below the search box.
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [in]    message     String in UTF-8 format.
    message_to_user: extern fn(s: &session, message: String),

    ///
    ///
    /// Called when processing needs to take place on the main thread.
    ///
    /// You need to call session_process_events() in the main thread to get libspotify to do
    /// more work. Failure to do so may cause request timeouts, or a lost connection.
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///
    /// Note:
    ///     This function is called from an internal session thread - you need to have proper
    ///     synchronization!
    ///
    notify_main_thread: extern fn(s: &session),

    /// Called when there is decompressed audio data available.
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [in]    format  Audio format descriptor audioformat
    ///     [in]    frames  Points to raw PCM data as described by format
    ///     [in]    num_frames  Number of available samples in frames. If this is 0, a discontinuity has occurred (such as after a seek). The application should flush its audio fifos, etc.
    ///
    /// Returns:
    ///     Number of frames consumed. This value can be used to rate limit the output from the library if your output buffers are saturated. The library will retry delivery in about 100ms.
    ///
    /// Note:
    ///     This function is called from an internal session thread - you need to have proper synchronization!
    ///     This function must never block. If your output buffers are full you must return 0 to signal that the library should retry delivery in a short while.
    music_delivery: extern fn(s: &session, format: &audioformat, frames: &libc::c_int, num_frames: isize)
                                -> isize,

    /// Music has been paused because an account only allows music to be played from one location
    /// simultaneously.
    ///
    /// Note:
    ///     When this callback is invoked the application should behave just as if the user pressed
    ///     the pause button. The application should also display a message to the user indicating
    ///     the playback has been paused because another application is playing using the same
    ///     account.
    ///     IT MUST NOT automatically resume playback but must instead wait for the user to
    ///     press play.
    ///
    /// Parameters:
    ///     [in]    session     Session
    play_token_lost: extern fn(s: &session),

    /// Logging callback.
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [in]    data        Log data
    log_message: extern fn(s: &session, data: &str),

    /// End of track. Called when the currently played track has reached its end.
    ///
    /// Note:
    ///     This function is invoked from the main thread
    ///
    /// Parameters:
    ///     [in]    session     Session
    end_of_track: extern fn(s: &session),

    /// Streaming error. Called when streaming cannot start or continue.
    ///
    /// Note:
    ///     This function is invoked from the main thread
    ///
    /// Parameters:
    ///    [in]    session      Session
    ///    [in]    error        One of the following errors, from error
    ///                         ERROR_NO_STREAM_AVAILABLE ERROR_OTHER_TRANSIENT ERROR_OTHER_PERMANENT
    streaming_error: extern fn(s: &session, error: ::error::Error),

    /// Called after user info (anything related to user objects) have been updated.
    ///
    /// Parameters:
    ///     [in]    session     Session
    userinfo_updated: extern fn(s: &session),

    /// Called when audio playback should start
    ///
    /// Note:
    ///     For this to work correctly the application must also implement get_audio_buffer_stats()
    ///     This function is called from an internal session thread - you need to have proper
    ///     synchronization!
    ///     This function must never block.
    ///
    /// Parameters:
    ///     [in]    session     Session
    start_playback: extern fn(s: &session),

    /// Called when audio playback should stop
    ///
    /// Note:
    ///     For this to work correctly the application must also implement get_audio_buffer_stats()
    ///     This function is called from an internal session thread - you need to have proper
    ///     synchronization!
    ///     This function must never block.
    ///
    /// Parameters:
    ///     [in]    session     Session
    stop_playback: extern fn(s: &session),

    /// Called to query application about its audio buffer
    ///
    /// Note:
    ///     This function is called from an internal session thread - you need to have proper
    ///     synchronization!
    ///         This function must never block.
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [out]   stats       Stats struct to be filled by application
    get_audio_buffer_stats: extern fn(s: &session, stats: &audio_buffer_stats),

    /// Called when offline synchronization status is updated
    ///
    /// Parameters:
    ///     [in]    session     Session
    offline_status_updated: extern fn(s: &session),

    /// Called when offline synchronization status is updated
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [in]    error       Offline error. Will be ERROR_OK if the offline synchronization
    ///                         error state has cleared
    offline_error: extern fn(s: &session, error: ::error::Error),

    /// Called when storable credentials have been updated, usually called when we have connected
    /// to the AP.
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [in]    blob        Blob is a null-terminated string which contains an encrypted token
    ///                         that can be stored safely on disk instead of storing plaintext passwords.
    credentials_blob_updated: extern fn(s: &session, blob: &str),

    /// Called when the connection state has updated - such as when logging in, going offline, etc.
    ///
    /// Parameters:
    ///     [in]    session     Session
    connectionstate_updated: extern fn(s: &session),

    /// Called when there is a scrobble error event
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [in]    error       Scrobble error. Currently ERROR_LASTFM_AUTH_ERROR.
    scrobble_error: extern fn(s: &session, error: ::error::Error),

    /// Called when there is a change in the private session mode
    ///
    /// Parameters:
    ///     [in]    session     Session
    ///     [in]    isPrivate   True if in private session, false otherwhise
    private_session_mode_changed: extern fn(session: &session, is_private: bool),
}
