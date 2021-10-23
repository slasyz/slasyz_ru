pub mod config;

use crate::config::Config;
use log::info;
use std::error::Error;
use std::net::SocketAddr;
use warp::Filter;

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // GET /api/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("api" / String).map(|name| format!("Hello, {}!", name));

    // TODO: access log

    let addr: SocketAddr = config.server.address.parse()?;

    info!("Starting server");
    warp::serve(hello).run(addr).await;
    Ok(())
}
