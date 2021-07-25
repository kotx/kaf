extern crate kaf;
extern crate log;

fn main() {
    kaf::from_str(&std::env::var("RUST_LOG").unwrap_or("trace".to_string()));

    log::trace!("trace");
    log::debug!("debug");
    log::info!("info");
    log::warn!("warn");
    log::error!("error");
}
