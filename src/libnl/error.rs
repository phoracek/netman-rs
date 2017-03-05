use std::error;
use std::ffi;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NlAddrAllocationFailed,
    NlSockAllocationFailed,
    NlSockConnectionFailed(NetlinkError),
    RtnlAddrAdditionFailed(NetlinkError),
    RtnlAddrFailedToSetLocal(NetlinkError),
    RtnlAddrRemovalFailed(NetlinkError),
    RtnlAddrAllocationFailed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NlAddrAllocationFailed => {
                write!(f, "nl_addr allocation failed (returned null pointer).")
            }
            Error::NlSockAllocationFailed => {
                write!(f, "nl_sock allocation failed (returned null pointer).")
            }
            Error::NlSockConnectionFailed(ref err) => {
                write!(f, "nl_sock connection failed: {}", err)
            }
            Error::RtnlAddrAdditionFailed(ref err) => {
                write!(f, "rtnl_addr addition failed: {}", err)
            }
            Error::RtnlAddrFailedToSetLocal(ref err) => {
                write!(f, "rtnl_addr failed to set local: {}", err)
            }
            Error::RtnlAddrRemovalFailed(ref err) => {
                write!(f, "rtnl_addr removal failed: {}", err)
            }
            Error::RtnlAddrAllocationFailed => {
                write!(f, "rtnl_addr allocation failed (returned null pointer).")
            }

        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NlAddrAllocationFailed => "nl_addr allocation failed",
            Error::NlSockAllocationFailed => "nl_sock allocation failed",
            Error::NlSockConnectionFailed(_) => "nl_sock connection failed",
            Error::RtnlAddrAdditionFailed(_) => "rtnl_addr addition failed",
            Error::RtnlAddrFailedToSetLocal(_) => "rtnl_addr failed to set local",
            Error::RtnlAddrRemovalFailed(_) => "rtnl_addr removal failed",
            Error::RtnlAddrAllocationFailed => "rtnl_addr allocation failed",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::NlAddrAllocationFailed => None,
            Error::NlSockAllocationFailed => None,
            Error::NlSockConnectionFailed(ref err) => Some(err),
            Error::RtnlAddrAdditionFailed(ref err) => Some(err),
            Error::RtnlAddrFailedToSetLocal(ref err) => Some(err),
            Error::RtnlAddrRemovalFailed(ref err) => Some(err),
            Error::RtnlAddrAllocationFailed => None,
        }
    }
}

#[derive(Debug)]
pub struct NetlinkError(pub i32);

impl fmt::Display for NetlinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.", nl_geterror(self.0))
    }
}

impl error::Error for NetlinkError {
    fn description(&self) -> &str {
        "libnl function end up with a non-zero return code"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

fn nl_geterror(error: i32) -> String {
    unsafe {
        ffi::CStr::from_ptr(nl::nl_geterror(error))
            .to_string_lossy()
            .into_owned()
    }
}

mod nl {
    use libc::c_char;

    #[link(name="nl-3")]
    extern "C" {
        pub fn nl_geterror(error: i32) -> *const c_char;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_libnl_error_message() {
        assert_eq!(format!("{}", NetlinkError(2)), "Interrupted system call.");
    }
}
