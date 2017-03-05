use libc;

use libnl::error::Error;

#[derive(Debug)]
pub enum AddrFamily {
    IPv4 = 2,
}

#[derive(Debug)]
pub struct NlAddr {
    pub nl_addr: *const nl::nl_addr,
}

impl NlAddr {
    pub fn new(family: AddrFamily, bytes: &[u8]) -> Result<NlAddr, Error> {
        Ok(NlAddr { nl_addr: try!(nl_addr_build(family, bytes)) })
    }
}

impl Drop for NlAddr {
    fn drop(&mut self) {
        nl_addr_put(self.nl_addr);
    }
}

fn nl_addr_build(family: AddrFamily, bytes: &[u8]) -> Result<*const nl::nl_addr, Error> {
    let nl_addr =
        unsafe { nl::nl_addr_build(family as i32, bytes.as_ptr(), bytes.len() as libc::size_t) };
    if nl_addr.is_null() {
        return Err(Error::NlAddrAllocationFailed);
    }
    Ok(nl_addr)
}

fn nl_addr_put(nl_addr: *const nl::nl_addr) {
    unsafe {
        nl::nl_addr_put(nl_addr);
    };
}

#[allow(non_camel_case_types)]
mod nl {
    use libc;

    pub type nl_addr = libc::c_void;

    #[link(name="nl-3")]
    extern "C" {
        pub fn nl_addr_build(family: i32, buf: *const u8, size: libc::size_t) -> *const nl_addr;
        pub fn nl_addr_put(nl_addr: *const nl_addr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nl_addr() {
        let nl_addr = NlAddr::new(AddrFamily::IPv4, &[10u8, 10u8, 10u8, 0u8]);
        assert!(nl_addr.is_ok());
    }
}
