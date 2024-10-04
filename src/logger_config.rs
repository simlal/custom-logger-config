use chrono::Local;
use colored::*;
use env_logger::{Builder, Env};
use std::io::Write;

pub fn init() {
    // Default to info level when not set
    let env = Env::default().filter_or("MY_LOG", "info");
    let mut builder = Builder::from_env(env);

    // Customize format
    builder.format(|buf, record| {
        let minimal_timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let level = match record.level() {
            log::Level::Error => "ERROR".red(),
            log::Level::Warn => "WARN".yellow(),
            log::Level::Info => "INFO".green(),
            log::Level::Debug => "DEBUG".blue(),
            log::Level::Trace => "TRACE".purple(),
        };
        writeln!(buf, "[{} - {}] {}", minimal_timestamp, level, record.args())
    });
    builder.init();
}
