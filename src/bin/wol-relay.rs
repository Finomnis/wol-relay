use clap::Parser;
use clap_verbosity_flag::{Verbosity, WarnLevel};
use std::{
    net::{IpAddr, Ipv4Addr, ToSocketAddrs},
    process::ExitCode,
};
use wol_relay::WolReceiver;

#[derive(Parser, Debug, Clone)]
struct CliArgs {
    /// Receive the magic packets on ADDR.
    #[arg(long, default_value_t = Ipv4Addr::UNSPECIFIED.into(), value_name = "ADDR")]
    listen_addr: IpAddr,
    /// Receive the magic packets on PORT.
    #[arg(short = 'p', long, default_value_t = 9, value_name = "PORT")]
    listen_port: u16,
    /// Send the magic packet to HOST.
    ///
    /// HOST may either be a DNS name, or an IPv4/IPv6 address.
    /// HOST may and most likely will be different from the
    /// target system to wake up: Instead the magic packet needs
    /// to be sent so that it physically passes the system to
    /// wake up.  As such, you will most likely want to use a
    /// broadcast or multicast address here.
    #[arg(long, default_value_t = Ipv4Addr::BROADCAST.to_string(), value_name = "HOST")]
    target_host: String,
    /// Send the magic packets to PORT.
    #[arg(long, default_value_t = 9, value_name = "PORT")]
    target_port: u16,
    #[command(flatten)]
    verbose: Verbosity<WarnLevel>,
}

fn main() -> ExitCode {
    let args = CliArgs::parse();

    env_logger::builder()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let wol_listener = match WolReceiver::new()
        .with_ip(args.listen_addr)
        .with_port(args.listen_port)
        .run()
    {
        Ok(listener) => listener,
        Err(e) => {
            log::error!(
                "Failed to listen on '{}:{}': {}",
                args.listen_addr,
                args.listen_port,
                e
            );
            return ExitCode::FAILURE;
        }
    };

    if let Ok(local_addr) = wol_listener.socket.local_addr() {
        log::info!("Listening for WoL packets on '{local_addr}'");
    } else {
        log::info!("Listening for WoL packets");
    }

    for target_mac in wol_listener {
        log::info!("Relaying WoL packet for '{target_mac}'");

        let target_addrs = match (args.target_host.as_str(), args.target_port).to_socket_addrs() {
            Ok(addr) => addr,
            Err(e) => {
                log::error!(
                    "Unable to resolve '{}:{}': {}",
                    args.target_host,
                    args.target_port,
                    e
                );
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

    ExitCode::SUCCESS
}
