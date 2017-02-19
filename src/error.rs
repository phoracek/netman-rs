use std::error;
use std::fmt;

use libnl;

#[derive(Debug)]
pub enum Error {
    SocketInitializationFailed(libnl::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::SocketInitializationFailed(ref err) => {
                write!(f, "Socket initialization failed: {}", err)
            }
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::SocketInitializationFailed(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::SocketInitializationFailed(ref err) => Some(err),
        }
    }
}
