use std::error;
use std::fmt;

use libnl;

#[derive(Debug)]
pub enum Error {
    AddressAdditionFailed(libnl::error::Error),
    AddressRemovalFailed(libnl::error::Error),
    SocketInitializationFailed(libnl::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::AddressAdditionFailed(ref err) => {
                write!(f, "Address addition failed: {}", err)
            }
            Error::AddressRemovalFailed(ref err) => {
                write!(f, "Address removal failed: {}", err)
            }
            Error::SocketInitializationFailed(ref err) => {
                write!(f, "Socket initialization failed: {}", err)
            }
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::AddressAdditionFailed(ref err) => err.description(),
            Error::AddressRemovalFailed(ref err) => err.description(),
            Error::SocketInitializationFailed(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::AddressAdditionFailed(ref err) => Some(err),
            Error::AddressRemovalFailed(ref err) => Some(err),
            Error::SocketInitializationFailed(ref err) => Some(err),
        }
    }
}
