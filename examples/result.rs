use std::convert::Infallible;

extern crate kaf;
extern crate log;

fn main() {
    kaf::init();

    let ok: Result<&str, Infallible> = Ok("ok");
    let err: Result<(), &str> = Err("err");

    log::trace!("{:?}", ok);
    log::debug!("{:?}", ok);
    log::info!("{:?}", ok);
    log::warn!("{:?}", ok);
    log::error!("{:?}", err);
}
