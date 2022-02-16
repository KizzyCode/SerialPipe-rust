//! Implements the error types

use ebacktrace::define_error;
use std::{
    io, result, str, num::ParseIntError,
    fmt::{ self, Debug, Display, Formatter },
};

/// Creates a new variant
#[macro_export] macro_rules! e {
    ($kind:expr, $($arg:tt)*) => ({ $crate::error::ErrorImpl::with_string($kind, format!($($arg)*)) })
}
/// Creates a new `Error::InvalidValue` kind
#[macro_export] macro_rules! einval {
    ($($arg:tt)*) => ({ e!($crate::error::ErrorKind::InvalidValue, $($arg)*) });
}
/// Creates a new `Error::TimedOut` kind
#[macro_export] macro_rules! etimedout {
    ($($arg:tt)*) => ({ e!($crate::error::ErrorKind::TimedOut, $($arg)*) });
}
/// Creates a new `Error::InOutError` kind
#[macro_export] macro_rules! eio {
    ($($arg:tt)*) => ({ e!($crate::error::ErrorKind::InOutError, $($arg)*) });
}


/// An error kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorKind {
    /// A value is invalid
    InvalidValue,
    /// An operation timed out
    TimedOut,
    /// An in-out-error occurred
    InOutError
}
impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ErrorKind::InvalidValue => write!(f, "A value is invalid"),
            ErrorKind::TimedOut => write!(f, "An operation timed out"),
            ErrorKind::InOutError => write!(f, "An I/O-error occurred")
        }
    }
}

// Define our custom error
define_error!(ErrorImpl);
impl From<ParseIntError> for ErrorImpl<ErrorKind> {
    fn from(error: ParseIntError) -> Self {
        Self::with_string(ErrorKind::InvalidValue, error)
    }
}
impl From<io::Error> for ErrorImpl<ErrorKind> {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::TimedOut => Self::with_string(ErrorKind::TimedOut, error),
            io::ErrorKind::WouldBlock => Self::with_string(ErrorKind::TimedOut, error),
            _ => Self::with_string(ErrorKind::InOutError, error)
        }
    }
}

/// A typealias for results with our error as error-variant
pub type Result<T = ()> = result::Result<T, ErrorImpl<ErrorKind>>;
