use std::net::SocketAddr;
use tokio::net::TcpStream;

/// Trait that connected IO resources implement.
///
/// The goal for this trait is to allow users to implement
/// custom IO types that can still provide the same connection
/// metadata.
pub trait Connected {
    /// Return the remote address this IO resource is connected too.
    fn remote_addr(&self) -> Option<SocketAddr> {
        None
    }
}

impl Connected for TcpStream {
    fn remote_addr(&self) -> Option<SocketAddr> {
        self.peer_addr().ok()
    }
}
