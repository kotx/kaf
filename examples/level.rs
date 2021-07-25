extern crate kaf;
extern crate log;

fn main() {
    kaf::with_level(log::LevelFilter::Debug);

    log::trace!("trace");
    log::debug!("debug");
    log::info!("info");
    log::warn!("warn");
    log::error!("error");

    mod inner {
        pub(crate) fn log() {
            log::trace!("trace");
            log::debug!("debug");
            log::info!("info");
            log::warn!("warn");
            log::error!("error");
        }
    }

    inner::log();
}
