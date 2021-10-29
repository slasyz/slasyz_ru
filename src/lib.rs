pub mod config;
mod handlers;
mod jwt;
mod toxic;

use crate::config::Config;
use crate::jwt::JWT;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::TimeZone;
use log::info;
use serde_json::json;
use serde_json::to_string_pretty;
use std::error::Error;
use std::io;
use std::io::BufRead;
use std::net::SocketAddr;

fn read_line(prompt: &str) -> Result<String, Box<dyn Error>> {
    eprint!("{}", prompt);

    let mut subject = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut subject)?;

    Ok(String::from(subject.trim()))
}

pub fn token_generate(config: Config) -> Result<(), Box<dyn Error>> {
    let jwt = JWT::new(config.jwt.secret)?;

    let subject = read_line("Enter subject name: ")?;
    let until = read_line("Enter expiration date in UTC (e.g. 2020-02-20 00:00:00): ")?;
    let until = chrono::NaiveDateTime::parse_from_str(&until, "%Y-%m-%d %H:%M:%S")?;
    let until = chrono::Utc.from_utc_datetime(&until);

    let token = jwt.generate(subject, until)?;

    let line = std::iter::repeat("-").take(token.len()).collect::<String>();
    eprintln!("{}", line);
    eprintln!("{}", token);
    eprintln!("{}", line);

    Ok(())
}

pub fn token_info(config: Config) -> Result<(), Box<dyn Error>> {
    let jwt = JWT::new(config.jwt.secret)?;

    let token = read_line("Enter token: ")?;
    let token = jwt.get_info(token)?;

    eprintln!("Subject: {}", token.subject);
    eprintln!("Valid until: {}", token.until.format("%Y-%m-%d %H:%M:%S"));

    Ok(())
}

pub async fn toxic_test(config: Config) -> Result<(), Box<dyn Error>> {
    let toxic = toxic::ToxicClient::new(config.toxic.url);

    let payload = json!({
        "test_key": "test_value"
    });

    let response = toxic.post(String::from("test"), payload).await?;
    let response = to_string_pretty(&response)?;

    eprintln!("{}", response);
    Ok(())
}

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let _jwt = jwt::JWT::new(config.jwt.secret)?;

    let toxic_client = toxic::ToxicClient::new(config.toxic.url);
    let toxic_app_data =
        web::Data::new(handlers::handler_toxic::ToxicHandlerData::new(toxic_client));

    info!("Starting server");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(toxic_app_data.clone())
            .route("/test", web::get().to(&handlers::handler_test::get))
            .route(
                "/api/toxic/{id}",
                web::post().to(&handlers::handler_toxic::post),
            )
    })
    .bind(config.server.address)?
    .run()
    .await?;
    Ok(())
}
