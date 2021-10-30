use crate::server::Context;
use lazy_static::lazy_static;
use serde::Serialize;
use std::collections::HashSet;
use tide::{Request, StatusCode};

lazy_static! {
    static ref METHODS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("test");
        set.insert("chat_spotify");
        set.insert("chat_shazam");
        set
    };
}

#[derive(Serialize)]
struct Error {
    error: String,
}

pub async fn post(mut req: Request<Context>) -> tide::Result {
    let method = req.param("method")?.to_string();

    if !METHODS.contains(method.as_str()) {
        return Err(tide::Error::from_str(
            StatusCode::NotFound,
            "Method not found.",
        ));
    }

    let request = req.body_json().await?;

    let res = req
        .state()
        .toxic_client
        .post(method, request)
        .await
        .map_err(|err| tide::Error::from_str(StatusCode::InternalServerError, err.to_string()))?;
    Ok(res.into())
}
