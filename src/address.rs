use std::net;

use error::Error;
use libnl;
use libnl::nl_addr::{AddrFamily, NlAddr};
use libnl::rtnl_addr::RtnlAddr;
use socket::Socket;

#[derive(Debug)]
pub struct IPv4Address {
    ifindex: Option<i32>,
    prefixlen: Option<u8>,
    local: Option<net::Ipv4Addr>,
    rtnl_addr: Option<RtnlAddr>,
}

impl IPv4Address {
    pub fn new() -> IPv4Address {
        IPv4Address {
            ifindex: None,
            prefixlen: None,
            local: None,
            rtnl_addr: None,
        }
    }

    pub fn set_ifindex(mut self, ifindex: i32) -> Self {
        self.ifindex = Some(ifindex);
        self
    }

    pub fn set_prefixlen(mut self, prefixlen: u8) -> Self {
        self.prefixlen = Some(prefixlen);
        self
    }

    pub fn set_local(mut self, address: net::Ipv4Addr) -> Self {
        self.local = Some(address);
        self
    }

    pub fn alloc_rtnl_addr(&mut self) -> Result<(), libnl::error::Error> {
        let mut rtnl_addr = try!(RtnlAddr::new());
        rtnl_addr.set_family(AddrFamily::IPv4);
        if let Some(ifindex) = self.ifindex {
            rtnl_addr.set_ifindex(ifindex);
        }
        if let Some(prefixlen) = self.prefixlen {
            rtnl_addr.set_prefixlen(prefixlen as i32);
        }
        if let Some(local) = self.local {
            let address = try!(NlAddr::new(AddrFamily::IPv4, &local.octets()));
            try!(rtnl_addr.set_local(address));
        }
        self.rtnl_addr = Some(rtnl_addr);
        Ok(())
    }
}

pub trait Addresses {
    fn add_address(&self, address: IPv4Address) -> Result<(), Error>;
    fn delete_address(&self, address: IPv4Address) -> Result<(), Error>;
}

impl Addresses for Socket {
    fn add_address(&self, mut address: IPv4Address) -> Result<(), Error> {
        try!(address.alloc_rtnl_addr().map_err(Error::AddressAdditionFailed));
        try!(address.rtnl_addr
            .unwrap()
            .add(&self.nl_sock)
            .map_err(Error::AddressAdditionFailed));
        Ok(())
    }

    fn delete_address(&self, mut address: IPv4Address) -> Result<(), Error> {
        try!(address.alloc_rtnl_addr().map_err(Error::AddressAdditionFailed));
        try!(address.rtnl_addr
            .unwrap()
            .delete(&self.nl_sock)
            .map_err(Error::AddressRemovalFailed));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_ipv4() {
        IPv4Address::new()
            .set_ifindex(0)
            .set_local(net::Ipv4Addr::new(192, 168, 0, 1))
            .set_prefixlen(24);
    }
}
