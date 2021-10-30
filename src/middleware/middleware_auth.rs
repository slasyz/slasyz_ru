use crate::jwt::JWT;
use log::warn;
use std::error::Error;
use tide::{Middleware, Next, Request, StatusCode};

const HEADER_NAME: &str = "Authorization";
const BEARER_PREFIX: &str = "Bearer ";

pub struct AccessMiddleware {
    jwt: JWT,
}

impl AccessMiddleware {
    pub fn new(jwt: JWT) -> AccessMiddleware {
        AccessMiddleware { jwt }
    }

    fn check<State: Clone + Send + Sync + 'static>(
        &self,
        req: &Request<State>,
    ) -> Result<(), Box<dyn Error>> {
        let mut values = req
            .header(HEADER_NAME)
            .ok_or("Empty Authorization header.")?
            .iter();
        let value = values
            .next()
            .ok_or("Empty Authorization header.")?
            .to_string();
        if values.next().is_some() {
            return Err("Too many Authorization headers.".into());
        }

        let token = value
            .strip_prefix(BEARER_PREFIX)
            .ok_or("Authorization header does not start with Bearer.")?;

        let is_valid = self.jwt.is_valid(token.to_string())?;
        if !is_valid {
            return Err("Invalid token.".into());
        }

        Ok(())
    }
}

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for AccessMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> tide::Result {
        if let Err(err) = self.check(&req) {
            warn!("Unauthorized {}: {}", req.url(), err);
            return Err(tide::Error::from_str(
                StatusCode::Unauthorized,
                "Unauthorized.",
            ));
        }

        let res = next.run(req).await;
        Ok(res)
    }
}
