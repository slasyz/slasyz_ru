use colored::*;
use log::{error, Level};
use slasyz_ru::config::Config;
use slasyz_ru::run;
use std::process;

const CONFIG_FILENAME: &str = "config.json";

#[tokio::main]
async fn main() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let message = match record.level() {
                Level::Error => message.to_string().as_str().red().bold(),
                Level::Info => message.to_string().as_str().blue(),
                Level::Debug => message.to_string().as_str().yellow(),
                _ => message.to_string().as_str().blue(),
            };
            out.finish(format_args!(
                "{} {} {} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%z"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("hyper", log::LevelFilter::Info)
        .chain(std::io::stderr())
        .apply()
        .unwrap_or_else(|err| {
            error!("Error configuring logger: {}", err.to_string());
            process::exit(1);
        });

    let config = Config::new(CONFIG_FILENAME).unwrap_or_else(|err| {
        error!("Error parsing config: {}", err.to_string());
        process::exit(1);
    });

    if let Err(err) = run(config).await {
        error!("{}", err.to_string());
        process::exit(1);
    }
}
