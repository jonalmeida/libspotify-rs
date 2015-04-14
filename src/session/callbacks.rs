use super::*;
use session::error;

struct session;

//TODO: implement
struct audio_buffer_stats;

pub trait Callbacks {
    fn logged_in(s: &session, error: error::Error);
    fn logged_out(s: &session);
    fn metadata_updated(s: &session);
    fn connection_error(s: &session, error: error::Error);
    fn message_to_user(s: &session, message: String);
    fn notify_main_threaD(s: &session);
    //fn music_delivery(sp_session *session, const sp_audioformat *format,
    //                  const void *frames, int num_frames) -> isize;
    fn play_token_lost(s: &session);
    fn log_message(s: &session, data: &str);
    fn end_of_track(s: &session);
    fn streaming_error(s: &session, error: error::Error);
    fn userinfo_updated(s: &session);
    fn start_playback(s: &session);
    fn stop_playback(s: &session);
    fn get_audio_buffer_stats(s: &session, stats: &audio_buffer_stats);
    fn offline_status_updated(s: &session);
    fn offline_error(s: &session, error: error::Error);
    //fn credentials_blob_updated(s: &session, const char *blob);
    fn connectionstate_updated(s: &session);
    fn scrobble_error(s: &session, error: error::Error);
    fn private_session_mode_changed(session: &session, is_private: bool);
}
