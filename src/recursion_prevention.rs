use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};

/// Checks if an address is our own machine.
///
/// This can be used to prevent broadcast recursion.
pub fn is_our_ip(addr: SocketAddr) -> bool {
    let source_addr: IpAddr = match addr {
        SocketAddr::V4(_) => Ipv4Addr::UNSPECIFIED.into(),
        SocketAddr::V6(_) => Ipv6Addr::UNSPECIFIED.into(),
    };

    // Create UDP socket to send to this address
    if let Ok(socket) = UdpSocket::bind((source_addr, 0)) {
        // Connect. Note that this still does not send data over the network, UDP connection does not require a handshake
        if socket.connect(addr).is_err() {
            return false;
        }

        if let Ok(local_addr) = socket.local_addr() {
            local_addr.ip() == addr.ip()
        } else {
            false
        }
    } else {
        false
    }
}
