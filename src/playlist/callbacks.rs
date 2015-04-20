use super::*;

// TODO: Define these.
struct user;
struct tracks;
struct playlist;

/// Playlist callbacks
///
/// Used to get notifications when playlists are updated. If some callbacks should not be of
/// interest, set them to NULL.
pub trait Callbacks<T> {

    /// Called when one or more tracks have been added to a playlist
    ///
    /// Parameters:
    ///     [in]    pl          Playlist object
    ///     [in]    tracks      Array of pointers to track objects
    ///     [in]    num_tracks  Number of entries in tracks
    ///     [in]    position    Position in the playlist for the first track.
    ///     [in]    userdata    Userdata passed to playlist_add_callbacks()
    fn tracks_added(pl: &playlist, track: &tracks, num_tracks: isize, position: isize, userdata: &T);

    /// Called when one or more tracks have been removed from a playlist
    ///
    /// Parameters:
    ///     [in]    pl          Playlist object
    ///     [in]    tracks      Array of positions representing the tracks that were removed
    ///     [in]    num_tracks  Number of entries in tracks
    ///     [in]    userdata    Userdata passed to playlist_add_callbacks()
    fn tracks_removed(pl: &playlist, tracks: &isize, num_tracks: isize, userdata: &T);

    /// Called when one or more tracks have been moved within a playlist
    ///
    /// Parameters:
    ///     [in]    pl          Playlist object
    ///     [in]    tracks      Array of positions representing the tracks that were moved
    ///     [in]    num_tracks  Number of entries in tracks
    ///     [in]    position    New position in the playlist for the first track.
    ///     [in]    userdata    Userdata passed to playlist_add_callbacks()
    fn tracks_moved(pl: &playlist, tracks: [isize], num_tracks: isize, new_position: isize, userdata: &T);

    /// Called when a playlist has been renamed. sp_playlist_name() can be used to find out the new name
    ///
    /// Parameters:
    ///     [in]    pl          Playlist object
    ///     [in]    userdata    Userdata passed to laylist_add_callbacks()
    fn playlist_renamed(pl: &playlist, userdata: &T);

    /// Called when state changed for a playlist.
    ///
    /// There are three states that trigger this callback:
    ///
    ///  - Collaboration for this playlist has been turned on or off
    ///  - The playlist started having pending changes, or all pending changes have now been committed
    ///  - The playlist started loading, or finished loading
    ///
    /// Parameters:
    ///     [in]    pl          Playlist object
    ///     [in]    userdata    Userdata passed to playlist_add_callbacks()
    fn playlist_state_changed(pl: &playlist, userdata: &T);


    /// Called when a playlist is updating or is done updating
    ///
    /// This is called before and after a series of changes are applied to the playlist. It allows
    /// e.g. the user interface to defer updating until the entire operation is complete.
    ///
    /// Parameters:
    ///     [in]    pl          Playlist object
    ///     [in]    done        True iff the update is completed
    ///     [in]    userdata    Userdata passed to playlist_add_callbacks()
    fn playlist_update_in_progress(pl: &playlist, done: bool, userdata: &T);

    /// Called when metadata for one or more tracks in a playlist has been updated.
    ///
    /// Parameters:
    ///     [in]    pl          Playlist object
    ///     [in]    userdata    Userdata passed to playlist_add_callbacks()
    fn playlist_metadata_updated(pl: &playlist, userdata: &T);

    /// Called when create time and/or creator for a playlist entry changes
    ///
    /// Parameters:
    ///     [in]    pl          Playlist object
    ///     [in]    position    Position in playlist
    ///     [in]    user        User object
    ///     [in]    time        When entry was created, seconds since the unix epoch.
    ///     [in]    userdata    Userdata passed to playlist_add_callbacks()
    fn track_created_changed(pl: &playlist, position: isize, user: &user, when: isize, userdata: &T);

    /// Called when seen attribute for a playlist entry changes.
    ///
    /// Parameters:
    ///     [in]    pl          Playlist object
    ///     [in]    position    Position in playlist
    ///     [in]    seen        Set if entry it marked as seen
    ///     [in]    userdata    Userdata passed to playlist_add_callbacks()
    fn track_seen_changed(pl: &playlist, position: isize, seen: bool, userdata: &T);

    /// Called when playlist description has changed
    ///
    /// Parameters:
    ///     [in]    pl          Playlist object
    ///     [in]    desc        New description
    ///     [in]    userdata    Userdata passed to playlist_add_callbacks()
    fn description_changed(pl: &playlist, desc: &str, userdata: &T);

    /// Called when playlist image has changed
    ///
    /// Parameters:
    ///     [in]    pl          Playlist object
    ///     [in]    image       New image
    ///     [in]    userdata    Userdata passed to playlist_add_callbacks()
    fn image_changed(pl: &playlist, image: [u8], userdata: &T);

    /// Called when message attribute for a playlist entry changes.
    ///
    /// Parameters:
    ///     [in]    pl          Playlist object
    ///     [in]    position    Position in playlist
    ///     [in]    message     UTF-8 encoded message
    ///     [in]    userdata    Userdata passed to playlist_add_callbacks()
    fn track_message_changed(pl: &playlist, position: isize,  message: &str, userdata: &T);

    /// Called when playlist subscribers changes (count or list of names)
    ///
    /// Parameters:
    ///     [in]    pl          Playlist object
    ///     [in]    userdata    Userdata passed to sp_playlist_add_callbacks()
    fn subscribers_changed(pl: &playlist, userdata: &T);
}
