#[allow(dead_code)]
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
