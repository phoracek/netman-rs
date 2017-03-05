extern crate netman;

use std::env;
use std::net;

use netman::address::{Addresses, IPv4Address};
use netman::socket::Socket;

static USAGE: &'static str = "Usage: ./add_address INTERFACE_INDEX";

fn main() {
    let ifindex_str = env::args().nth(1).expect(USAGE);
    let ifindex = ifindex_str.parse::<i32>().expect(USAGE);

    let socket = Socket::new().unwrap();
    let address = IPv4Address::new()
        .set_ifindex(ifindex)
        .set_local(net::Ipv4Addr::new(192, 168, 0, 1))
        .set_prefixlen(24);
    match socket.add_address(address) {
        Ok(_) => println!("Success"),
        Err(err) => println!("Failed: {}", err),
    }
}
