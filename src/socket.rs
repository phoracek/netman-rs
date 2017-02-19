use error::Error;
use libnl::nl_sock::NlSock;

#[derive(Debug)]
pub struct Socket {
    pub nl_sock: NlSock,
}

impl Socket {
    pub fn new() -> Result<Socket, Error> {
        match NlSock::new() {
            Ok(nl_sock) => Ok(Socket { nl_sock: nl_sock }),
            Err(err) => Err(Error::SocketInitializationFailed(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize() {
        let socket = Socket::new();
        assert!(socket.is_ok());
    }
}
