use crate::config::Config;
use crate::middleware::middleware_auth::AccessMiddleware;
use crate::middleware::middleware_log::LogMiddleware;
use crate::toxic::ToxicClient;
use crate::{handlers, jwt, toxic};
use log::info;
use std::error::Error;
use std::sync::Arc;

#[derive(Clone)]
pub struct Context {
    pub toxic_client: Arc<ToxicClient>,
}

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let jwt = jwt::JWT::new(config.jwt.secret)?;
    let toxic_client = toxic::ToxicClient::new(config.toxic.url);

    let context = Context {
        toxic_client: Arc::new(toxic_client),
    };
    let mut app = tide::with_state(context);

    // TODO: use tide::log::LogMiddleware (figure out how to print all info).
    app.with(LogMiddleware::new());

    app.at("/test").get(handlers::handler_test::get);
    app.at("/api/toxic/:method")
        .with(AccessMiddleware::new(jwt))
        .post(handlers::handler_toxic::post);

    info!("Starting server");
    app.listen(config.server.address).await?;
    Ok(())
}
