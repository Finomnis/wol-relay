use env_logger::Env;
use wol_relay::WolReceiver;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    for target_mac in WolReceiver::new().run().unwrap() {
        log::info!("Relaying WoL packet for {target_mac}");
        if let Err(e) =
            wol::send_magic_packet(target_mac, secure_on, (Ipv4Addr::BROADCAST, 9).into())
        {
            log::error!("Failed to send WoL packet: {}", e);
        }
    }
}
