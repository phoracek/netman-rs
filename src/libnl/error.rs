use std::error;
use std::ffi;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NlSockAllocationFailed,
    NlSockConnectionFailed(NetlinkError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NlSockAllocationFailed => {
                write!(f, "nl_sock allocation failed (returned null pointer).")
            }
            Error::NlSockConnectionFailed(ref err) => {
                write!(f, "nl_sock connection failed: {}", err)
            }
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NlSockAllocationFailed => "nl_sock allocation failed",
            Error::NlSockConnectionFailed(_) => "nl_sock connection failed",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::NlSockAllocationFailed => None,
            Error::NlSockConnectionFailed(ref err) => Some(err),
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
