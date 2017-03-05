extern crate netman;

use std::env;

use netman::address::{Addresses, IPv4Address};
use netman::socket::Socket;

static USAGE: &'static str = "Usage: ./delete_address INTERFACE_INDEX";

fn main() {
    let ifindex_str = env::args().nth(1).expect(USAGE);
    let ifindex = ifindex_str.parse::<i32>().expect(USAGE);

    let socket = Socket::new().unwrap();
    let address = IPv4Address::new().set_ifindex(ifindex);
    match socket.delete_address(address) {
        Ok(_) => println!("Success"),
        Err(err) => println!("Failed: {}", err),
    }
}
