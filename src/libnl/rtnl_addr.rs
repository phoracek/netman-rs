use libnl::error::{Error, NetlinkError};
use libnl::nl_addr::{AddrFamily, NlAddr};
use libnl::nl_sock::NlSock;

#[derive(Debug)]
pub struct RtnlAddr {
    pub rtnl_addr: *const nl::rtnl_addr,
}

impl RtnlAddr {
    pub fn new() -> Result<RtnlAddr, Error> {
        Ok(RtnlAddr { rtnl_addr: try!(rtnl_addr_alloc()) })
    }

    pub fn set_ifindex(&mut self, ifindex: i32) {
        rtnl_addr_set_ifindex(self.rtnl_addr, ifindex);
    }

    pub fn set_prefixlen(&mut self, prefixlen: i32) {
        rtnl_addr_set_prefixlen(self.rtnl_addr, prefixlen);
    }

    pub fn set_local(&mut self, local: NlAddr) -> Result<(), Error> {
        Ok(try!(rtnl_addr_set_local(self.rtnl_addr, local.nl_addr)))
    }

    pub fn set_family(&mut self, family: AddrFamily) {
        rtnl_addr_set_family(self.rtnl_addr, family as i32);
    }

    pub fn add(self, nl_sock: &NlSock) -> Result<(), Error> {
        Ok(try!(rtnl_addr_add(nl_sock.nl_sock, self.rtnl_addr)))
    }

    pub fn delete(self, nl_sock: &NlSock) -> Result<(), Error> {
        Ok(try!(rtnl_addr_delete(nl_sock.nl_sock, self.rtnl_addr)))
    }
}

impl Drop for RtnlAddr {
    fn drop(&mut self) {
        rtnl_addr_put(self.rtnl_addr);
    }
}

fn rtnl_addr_alloc() -> Result<*const nl::rtnl_addr, Error> {
    let rtnl_addr = unsafe { nl::rtnl_addr_alloc() };
    if rtnl_addr.is_null() {
        return Err(Error::RtnlAddrAllocationFailed);
    }
    Ok(rtnl_addr)
}

fn rtnl_addr_put(rtnl_addr: *const nl::rtnl_addr) {
    unsafe {
        nl::rtnl_addr_put(rtnl_addr);
    };
}

fn rtnl_addr_set_ifindex(rtnl_addr: *const nl::rtnl_addr, ifindex: i32) {
    unsafe {
        nl::rtnl_addr_set_ifindex(rtnl_addr, ifindex);
    }
}

fn rtnl_addr_set_prefixlen(rtnl_addr: *const nl::rtnl_addr, prefixlen: i32) {
    unsafe {
        nl::rtnl_addr_set_prefixlen(rtnl_addr, prefixlen);
    }
}

fn rtnl_addr_set_local(rtnl_addr: *const nl::rtnl_addr,
                       prefixlen: *const nl::nl_addr)
                       -> Result<(), Error> {
    let rc = unsafe { nl::rtnl_addr_set_local(rtnl_addr, prefixlen) };
    if rc != 0 {
        return Err(Error::RtnlAddrFailedToSetLocal(NetlinkError(rc)));
    }
    Ok(())
}

fn rtnl_addr_set_family(rtnl_addr: *const nl::rtnl_addr, family: i32) {
    unsafe {
        nl::rtnl_addr_set_family(rtnl_addr, family);
    }
}

pub fn rtnl_addr_add(nl_sock: *const nl::nl_sock,
                     rtnl_addr: *const nl::rtnl_addr)
                     -> Result<(), Error> {
    let rc = unsafe { nl::rtnl_addr_add(nl_sock, rtnl_addr, 0) };
    if rc != 0 {
        return Err(Error::RtnlAddrAdditionFailed(NetlinkError(rc)));
    }
    Ok(())
}

pub fn rtnl_addr_delete(nl_sock: *const nl::nl_sock,
                        rtnl_addr: *const nl::rtnl_addr)
                        -> Result<(), Error> {
    let rc = unsafe { nl::rtnl_addr_delete(nl_sock, rtnl_addr, 0) };
    if rc != 0 {
        return Err(Error::RtnlAddrRemovalFailed(NetlinkError(rc)));
    }
    Ok(())
}

#[allow(non_camel_case_types)]
mod nl {
    use libc;

    pub type nl_addr = libc::c_void;
    pub type nl_sock = libc::c_void;
    pub type rtnl_addr = libc::c_void;

    #[link(name="nl-route-3")]
    extern "C" {
        pub fn rtnl_addr_alloc() -> *const rtnl_addr;
        pub fn rtnl_addr_put(rtnl_addr: *const rtnl_addr);
        pub fn rtnl_addr_set_ifindex(rtnl_addr: *const rtnl_addr, ifindex: i32);
        pub fn rtnl_addr_set_prefixlen(rtnl_addr: *const rtnl_addr, prefixlen: i32);
        pub fn rtnl_addr_set_local(rtnl_addr: *const rtnl_addr, local: *const nl_addr) -> i32;
        pub fn rtnl_addr_set_family(rtnl_addr: *const rtnl_addr, family: i32);
        pub fn rtnl_addr_add(nl_sock: *const nl_sock,
                             rtnl_addr: *const rtnl_addr,
                             flags: i32)
                             -> i32;
        pub fn rtnl_addr_delete(nl_sock: *const nl_sock,
                                rtnl_addr: *const rtnl_addr,
                                flags: i32)
                                -> i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libnl::nl_addr::AddrFamily;

    #[test]
    fn rtnl_addr() {
        let mut rtnl_addr = RtnlAddr::new().unwrap();
        rtnl_addr.set_ifindex(1);
        rtnl_addr.set_prefixlen(24);
        let nl_addr = NlAddr::new(AddrFamily::IPv4, &[10u8, 10u8, 10u8, 0u8]).unwrap();
        rtnl_addr.set_local(nl_addr).unwrap();
    }
}
