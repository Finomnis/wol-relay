use env_logger::Env;
use wol_relay::WolReceiver;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    for target_mac in WolReceiver::new().run().unwrap() {
        log::info!("Received WoL: {target_mac}");
    }
}
