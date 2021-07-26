extern crate kaf;
extern crate log;

fn main() {
    kaf::init();

    let some: Option<&str> = Some("ok");
    let none: Option<&str> = None;

    log::trace!("{:?}", some);
    log::debug!("{:?}", some);
    log::info!("{:?}", some);
    log::warn!("{:?}", some);
    log::error!("{:?}", none);
}
