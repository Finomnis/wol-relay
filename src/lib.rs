use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
};

use wol::MacAddr6;

mod wol_message;

pub struct WolReceiver {
    addr: SocketAddr,
}

impl WolReceiver {
    /// Creates a new WoL receiver, listening on `0.0.0.0:9`.
    pub fn new() -> Self {
        Self {
            addr: SocketAddr::from((Ipv4Addr::UNSPECIFIED, 9)),
        }
    }

    /// Sets the IP address to listen at.
    ///
    /// Default: `0.0.0.0`.
    ///
    /// # Arguments
    ///
    /// * `ip` - The IP address.
    ///
    pub fn with_ip(mut self, ip: IpAddr) -> Self {
        self.addr.set_ip(ip);
        self
    }

    /// Sets the port to listen at.
    ///
    /// Default: `9`.
    ///
    /// # Arguments
    ///
    /// * `port` - The port number.
    ///
    pub fn with_port(mut self, port: u16) -> Self {
        self.addr.set_port(port);
        self
    }

    /// Start listening.
    ///
    /// # Return
    ///
    /// An iterator of WoL messages
    pub fn run(self) -> io::Result<WolIter> {
        Ok(WolIter {
            socket: UdpSocket::bind(self.addr)?,
        })
    }
}

impl Default for WolReceiver {
    fn default() -> Self {
        Self::new()
    }
}

pub struct WolIter {
    socket: UdpSocket,
}

impl Iterator for WolIter {
    type Item = MacAddr6;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut buf = vec![0u8; wol_message::WOL_MAX_SIZE];
            match self.socket.recv_from(&mut buf) {
                Ok((size, addr)) => {
                    if let Some(buf) = buf.get(..size) {
                        if let Some(target_mac) = wol_message::parse_wol_message(buf) {
                            log::debug!("Received WoL from {} to wake {}", addr.ip(), target_mac);
                            return Some(target_mac);
                        } else {
                            log::debug!("Received non-WoL message: {buf:x?}");
                        }
                    };
                }
                Err(e) => {
                    log::error!("Error while listening for WoL Packets: {e}");
                    return None;
                }
            }
        }
    }
}
