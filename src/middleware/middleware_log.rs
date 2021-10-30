use log::info;
use tide::{Middleware, Next, Request};

pub struct LogMiddleware {}

impl LogMiddleware {
    pub fn new() -> LogMiddleware {
        LogMiddleware {}
    }
}

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for LogMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> tide::Result {
        // 127.0.0.1:56175 - "POST /api/test HTTP/1.1" 200
        let version = match req.version() {
            Some(val) => val.to_string(),
            None => "<unknown>".to_string(),
        };
        let output = format!(
            "{} - \"{} {} {}\"",
            req.peer_addr().or(Some("<unknown>")).unwrap(),
            req.method(),
            req.url(),
            version,
        );

        let res = next.run(req).await;

        info!("{} {}", output, res.status());

        Ok(res)
    }
}
