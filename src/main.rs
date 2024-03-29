use colored::*;
use log::{error, Level};
use slasyz_ru::config::Config;
use slasyz_ru::{server, token_generate, token_info, toxic_test};
use std::error::Error;
use std::{env, process};

const CONFIG_FILENAME: &str = "config.json";

enum Action {
    TokenGenerate,
    TokenInfo,
    ToxicTest,
    Run,
}

fn get_action(mut args: env::Args) -> Result<Action, Box<dyn Error>> {
    args.next().ok_or("Cannot get program name.")?;

    let action = match args.next() {
        Some(val) => val,
        None => return Ok(Action::Run),
    };

    match action.as_str() {
        "generate" => Ok(Action::TokenGenerate),
        "info" => Ok(Action::TokenInfo),
        "test" => Ok(Action::ToxicTest),
        _ => Result::Err("Unknown subcommand.".into()),
    }
}

#[async_std::main]
async fn main() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let message = match record.level() {
                Level::Error => message.to_string().as_str().red().bold(),
                Level::Warn => message.to_string().as_str().yellow(),
                Level::Info => message.to_string().as_str().blue(),
                Level::Debug => message.to_string().as_str().cyan(),
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
        .level_for("isahc", log::LevelFilter::Info)
        .level_for("surf", log::LevelFilter::Warn)
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

    let action = get_action(env::args()).unwrap_or_else(|err| {
        error!("Error getting action: {}", err.to_string());
        process::exit(1);
    });

    let result = match action {
        Action::Run => server::run(config).await,
        Action::TokenGenerate => token_generate(config),
        Action::TokenInfo => token_info(config),
        Action::ToxicTest => toxic_test(config).await,
    };
    if let Err(err) = result {
        error!("{}", err.to_string());
        process::exit(1);
    }
}
