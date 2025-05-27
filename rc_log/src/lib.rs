use simple_logger::SimpleLogger;

pub use log;
pub use tracing_subscriber;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/26
///

pub fn init_log() {
    //env_logger::init();
    //env_logger::Builder::from_default_env().init();

    // Quick start: use default initialization
    //colog::init();

    SimpleLogger::new().init().unwrap();

    // log::error!("error message");
    // log::error!("error with fmt: {}", 42);
    // log::warn!("warn message");
    // log::info!("info message");
    // log::debug!("debug message");
    // log::trace!("trace message");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
