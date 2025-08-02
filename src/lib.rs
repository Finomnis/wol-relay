#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/Finomnis/wol-relay/issues")]

use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket},
};

use wol::MacAddr6;

mod recursion_prevention;
mod wol_message;

/// Used to configure and bind Wake-on-LAN receiver socket
pub struct WolReceiverConfig {
    addr: SocketAddr,
}

impl WolReceiverConfig {
    /// Create a new WoL receiver config.
    pub fn new() -> Self {
        Self {
            addr: SocketAddr::from((Ipv4Addr::UNSPECIFIED, 9)),
        }
    }

    /// Set the IP address to listen on.
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

    /// Set the port to listen on.
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

    /// Bind the socket and start listening.
    ///
    /// # Return
    ///
    /// An iterator of WoL messages
    pub fn bind(self) -> io::Result<WolSocket> {
        Ok(WolSocket {
            socket: UdpSocket::bind(self.addr)?,
        })
    }
}

impl Default for WolReceiverConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// A socket that listens for Wake-on-LAN packets
///
/// Implements [`Iterator`], delivering a continuous stream of
/// received WoL requests.
///
/// Filters out requests that originate from this machine, to
/// prevent recursion.
pub struct WolSocket {
    socket: UdpSocket,
}

impl WolSocket {
    /// Relay all received WoL packets to the given address.
    ///
    /// # Arguments
    ///
    /// * `host` - The target hostname/ip address. In most cases, this will be a broadcast address.
    /// * `port` - The port number. In most cases `9`.
    ///
    pub fn relay_to(&mut self, host: &str, port: u16) -> io::Result<()> {
        for target_mac in self {
            let target_mac = target_mac?;
            log::info!("Relaying WoL packet for '{target_mac}'");

            let target_addrs = match (host, port).to_socket_addrs() {
                Ok(addr) => addr,
                Err(e) => {
                    log::error!("Unable to resolve '{host}:{port}': {e}");
                    continue;
                }
            };

            for target_addr in target_addrs {
                log::debug!("Sending WoL packet for '{target_mac}' to '{target_addr}'");
                if let Err(e) = wol::send_magic_packet(target_mac, None, target_addr) {
                    log::error!("Failed to send WoL packet to '{target_addr}': {e}");
                }
            }
        }

        Ok(())
    }

    /// Returns the socket address that this socket was created from.
    ///
    /// See [`UdpSocket::local_addr`].
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }
}

impl Iterator for WolSocket {
    type Item = io::Result<MacAddr6>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut buf = vec![0u8; wol_message::WOL_MAX_SIZE];
            match self.socket.recv_from(&mut buf) {
                Ok((size, addr)) => {
                    if !recursion_prevention::is_our_ip(addr) {
                        if let Some(buf) = buf.get(..size) {
                            if let Some(target_mac) = wol_message::parse_wol_message(buf) {
                                log::debug!(
                                    "Received WoL from {} to wake {}",
                                    addr.ip(),
                                    target_mac
                                );
                                return Some(Ok(target_mac));
                            } else {
                                log::debug!("Received non-WoL message: {buf:x?}");
                            }
                        }
                    } else {
                        log::debug!("Detected recursion, skipping packet ...");
                    }
                }
                Err(e) => {
                    return Some(Err(e));
                }
            }
        }
    }
}
