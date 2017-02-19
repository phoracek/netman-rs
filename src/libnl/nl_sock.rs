use libnl::error::{Error, NetlinkError};

#[derive(Debug)]
pub struct NlSock {
    pub nl_sock: *const nl::nl_sock,
}

impl NlSock {
    pub fn new() -> Result<NlSock, Error> {
        let nl_sock = try!(nl_socket_alloc());
        match nl_connect(nl_sock) {
            Ok(_) => Ok(NlSock { nl_sock: nl_sock }),
            Err(err) => {
                nl_socket_free(nl_sock);
                Err(err)
            }
        }
    }
}

impl Drop for NlSock {
    fn drop(&mut self) {
        nl_close(self.nl_sock);
        nl_socket_free(self.nl_sock);
    }
}

fn nl_socket_alloc() -> Result<*const nl::nl_sock, Error> {
    let nl_sock = unsafe { nl::nl_socket_alloc() };
    if nl_sock.is_null() {
        return Err(Error::NlSockAllocationFailed);
    }
    Ok(nl_sock)
}

fn nl_socket_free(nl_sock: *const nl::nl_sock) {
    unsafe {
        nl::nl_socket_free(nl_sock);
    }
}

fn nl_connect(nl_sock: *const nl::nl_sock) -> Result<(), Error> {
    let rc = unsafe { nl::nl_connect(nl_sock, 0u32) };
    if rc != 0 {
        return Err(Error::NlSockConnectionFailed(NetlinkError(rc)));
    }
    Ok(())
}

fn nl_close(nl_sock: *const nl::nl_sock) {
    unsafe {
        nl::nl_close(nl_sock);
    }
}

#[allow(non_camel_case_types)]
mod nl {
    use libc;

    pub type nl_sock = libc::c_void;

    #[link(name="nl-3")]
    extern "C" {
        pub fn nl_socket_alloc() -> *const nl_sock;
        pub fn nl_socket_free(nl_sock: *const nl_sock);
        pub fn nl_connect(nl_sock: *const nl_sock, protocol: u32) -> i32;
        pub fn nl_close(nl_sock: *const nl_sock);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize() {
        let nl_sock = NlSock::new();
        assert!(nl_sock.is_ok());
    }
}
