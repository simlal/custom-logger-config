use custom_logger_config::logger_config;
use log::{debug, error, info};
fn main() {
    logger_config::init();
    error!("my_err");
    println!("####TEST println macro####");
    info!("my_info");
    debug!("my_debug");
}
