use log::LevelFilter;

extern crate kaf;
extern crate log;

fn main() {
    kaf::with_filter(
        Box::new(|target, _level| {
            (target == "filter" || target.starts_with("filter::")) && !target.ends_with("::silent")
        }),
        LevelFilter::Info,
    );

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

    mod silent {
        pub(crate) fn log() {
            log::trace!("trace");
            log::debug!("debug");
            log::info!("info");
            log::warn!("warn");
            log::error!("error");
        }
    }

    inner::log();
    silent::log();
}
