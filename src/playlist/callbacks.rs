use super::*;

// TODO: Define these.
struct user;
struct tracks;
struct playlist;

// Copy comments from spotify docs here.
pub trait Callbacks<T> {
    fn tracks_added(pl: &playlist, track: &tracks, num_tracks: isize, position: isize, userdata: &T);
    fn tracks_removed(pl: &playlist, tracks: &isize, num_tracks: isize, userdata: &T);
    fn tracks_moved(pl: &playlist, tracks: [isize], num_tracks: isize, new_position: isize, userdata: &T);
    fn playlist_renamed(pl: &playlist, userdata: &T);
    fn playlist_state_changed(pl: &playlist, userdata: &T);
    fn playlist_update_in_progress(pl: &playlist, done: bool, userdata: &T);
    fn playlist_metadata_updated(pl: &playlist, userdata: &T);
    fn track_created_changed(pl: &playlist, position: isize, user: &user, when: isize, userdata: &T);
    fn track_seen_changed(pl: &playlist, position: isize, seen: bool, userdata: &T);
    fn description_changed(pl: &playlist, desc: &str, userdata: &T);
    fn image_changed(pl: &playlist, image: [u8], userdata: &T);
    fn track_message_changed(pl: &playlist, position: isize,  message: &str, userdata: &T);
    fn subscribers_changed(pl: &playlist, userdata: &T);
}
