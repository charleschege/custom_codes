use std::error::Error;
use std::fmt;
use std::io::ErrorKind;

/// Eenable downcasting to a borrowed string
#[derive(Debug)]
pub struct BorrowedStr<'se>(pub &'se str);

impl<'se> fmt::Display for BorrowedStr<'se> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'se> Error for BorrowedStr<'se> {}

/// Convert a `String`, `&'_ str`, `OsStr` or `OsString` into this type to enable downcasting to a string
#[derive(Debug)]
pub struct StringifyError(pub String);

impl<'se> fmt::Display for StringifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'se> Error for StringifyError {}

/// All common errors for use in crates. They mirror std::io::ErrorKind;
#[derive(Debug)]
pub enum DownCastErrors<'se> {
    /// An entity was not found, often a file.
    NotFound,
    /// The operation lacked the necessary privileges to complete.
    PermissionDenied,
    /// The connection was refused by the remote server.
    ConnectionRefused,
    /// The connection was reset by the remote server.
    ConnectionReset,
    /// The connection was aborted (terminated) by the remote server.
    ConnectionAborted,
    /// The network operation failed because it was not connected yet.
    NotConnected,
    /// A socket address could not be bound because the address is already in use elsewhere.
    AddrInUse,
    /// A nonexistent interface was requested or the requested address was not local.
    AddrNotAvailable,
    /// The operation failed because a pipe was closed.
    BrokenPipe,
    /// An entity already exists, often a file.
    AlreadyExists,
    /// The operation needs to block to complete, but the blocking operation was requested to not occur.
    WouldBlock,
    /// A parameter was incorrect.
    InvalidInput,
    ///Data not valid for the operation were encountered.
    ///
    ///Unlike `InvalidInput`, this typically means that the operation parameters were valid, however the error was caused by malformed input data.
    ///
    ///For example, a function that reads a file into a string will error with `InvalidData` if the file's contents are not valid UTF-8.
    InvalidData,
    /// The I/O operation's timeout expired, causing it to be canceled.
    TimedOut,
    /// An error returned when an operation could not be completed because a call to write returned `Ok(0)`.
    ///
    ///This typically means that an operation could only succeed if it wrote a particular number of bytes but only a smaller number of bytes could be written.
    WriteZero,
    /// This operation was interrupted.
    ///
    /// Interrupted operations can typically be retried.
    Interrupted,
    /// Any I/O error not part of this list.
    Other,
    /// An error returned when an operation could not be completed because an "end of file" was reached prematurely.
    ///
    ///This typically means that an operation could only succeed if it read a particular number of bytes but only a smaller number of bytes could be read.
    UnexpectedEof,
    /// This error is not specified yet
    Unspecified,
    /// Any `StripPrefix`  error
    StripPrefixError(std::path::StripPrefixError),
    /// An `OsString` Error
    Stringify(String),
    /// Returns an error as a borrowed string
    BorrowedStr(&'se str),
    /// The file is invalid
    InvalidFile,
    /// Name of a file is invalid
    InvalidFileName,
    /// Path is not a directory
    InvalidFolder,
    /// Path is not valid
    InvalidPath,
    /// No matches were found when downcasting the error to `std::io::Error` so it is not an `I/O` error
    Unmatched(anyhow::Error),
}

/// This method tries to downcast an `anyhow::Error` to return a `DownCastErrors` enum for common error handling
pub fn try_downcast<'se>(error: anyhow::Error) -> DownCastErrors<'se> {
    if let Some(ioerror) = error.root_cause().downcast_ref::<std::io::Error>() {
        let kind = ioerror.kind();

        match kind {
            ErrorKind::NotFound => DownCastErrors::NotFound,
            ErrorKind::PermissionDenied => DownCastErrors::PermissionDenied,
            ErrorKind::ConnectionRefused => DownCastErrors::ConnectionRefused,
            ErrorKind::ConnectionReset => DownCastErrors::ConnectionReset,
            ErrorKind::ConnectionAborted => DownCastErrors::ConnectionAborted,
            ErrorKind::NotConnected => DownCastErrors::NotConnected,
            ErrorKind::AddrInUse => DownCastErrors::AddrInUse,
            ErrorKind::AddrNotAvailable => DownCastErrors::AddrNotAvailable,
            ErrorKind::BrokenPipe => DownCastErrors::BrokenPipe,
            ErrorKind::AlreadyExists => DownCastErrors::AlreadyExists,
            ErrorKind::WouldBlock => DownCastErrors::WouldBlock,
            ErrorKind::InvalidInput => DownCastErrors::InvalidInput,
            ErrorKind::InvalidData => DownCastErrors::InvalidData,
            ErrorKind::TimedOut => DownCastErrors::TimedOut,
            ErrorKind::WriteZero => DownCastErrors::WriteZero,
            ErrorKind::Interrupted => DownCastErrors::Interrupted,
            ErrorKind::Other => DownCastErrors::Other,
            ErrorKind::UnexpectedEof => DownCastErrors::UnexpectedEof,
            _ => DownCastErrors::Unspecified,
        }
    } else if let Some(strip_prefix_error) = error
        .root_cause()
        .downcast_ref::<std::path::StripPrefixError>()
    {
        DownCastErrors::StripPrefixError(strip_prefix_error.clone())
    } else if let Some(os_string_error) = error.root_cause().downcast_ref::<StringifyError>() {
        DownCastErrors::Stringify(os_string_error.0.clone())
    } else if let Some(borrowed_str) = error.root_cause().downcast_ref::<BorrowedStr>() {
        DownCastErrors::BorrowedStr(borrowed_str.0)
    } else if let Some(_) = error.root_cause().downcast_ref::<InvalidFile>() {
        DownCastErrors::InvalidFile
    } else if let Some(_) = error.root_cause().downcast_ref::<InvalidFileName>() {
        DownCastErrors::InvalidFileName
    } else if let Some(_) = error.root_cause().downcast_ref::<InvalidFolder>() {
        DownCastErrors::InvalidFolder
    } else if let Some(_) = error.root_cause().downcast_ref::<InvalidPath>() {
        DownCastErrors::InvalidPath
    } else {
        DownCastErrors::Unmatched(error)
    }
}

/// Enables downcasting an invalid file to produce `DownCastError`
#[derive(Debug)]
pub struct InvalidFile;

impl fmt::Display for InvalidFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for InvalidFile {}

/// Enables downcasting an invalid file name to produce `DownCastError`
#[derive(Debug)]
pub struct InvalidFileName;

impl fmt::Display for InvalidFileName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for InvalidFileName {}

/// Enables downcasting an invalid folder to produce `DownCastError`
#[derive(Debug)]
pub struct InvalidFolder;

impl fmt::Display for InvalidFolder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for InvalidFolder {}
/// Enables downcasting an invalid folder to produce `DownCastError`
#[derive(Debug)]
pub struct InvalidPath;

impl fmt::Display for InvalidPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for InvalidPath {}
