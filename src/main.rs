mod core;
mod models;
use core::{loader::{load_drivers, load_newest_chrome_driver, load_newest_gecko_driver, load_newest_opera_driver}, utils::FULL_CHRONO_FORMAT_STRING};

use chrono::Local;
use colored::Colorize;
use fern::colors::{Color, ColoredLevelConfig};
use homedir::windows::home;

// 1 client
// no unwrap
// better logs
#[tokio::main]
async fn main() {
    log_init();
    let homedir = homedir::my_home().expect("Home dir error").expect("No home dir");
    let folder = homedir.join("web_drivers");
    let _ = std::fs::create_dir(&folder);
    load_drivers(&folder).await;
}

fn log_init() {
    let colors = ColoredLevelConfig::new()
    .error(Color::Red)
    .warn(Color::Yellow)
    // we actually don't need to specify the color for debug and info, they are white by default
    .info(Color::Green)
    .debug(Color::Cyan)
    // depending on the terminals color scheme, this is the same as the background color
    .trace(Color::BrightBlack);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Local::now().format(FULL_CHRONO_FORMAT_STRING).to_string().purple(),
                colors.color(record.level()),
                record.target().blue(),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Trace)
        // - and per-module overrides
        .level_for("hyper", log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        // Apply globally
        .apply()
        .unwrap();
}
