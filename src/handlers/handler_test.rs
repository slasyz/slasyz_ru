use crate::server::Context;
use serde::Serialize;
use tide::Body;

#[derive(Serialize)]
struct TestResult {
    status: String,
}

pub async fn get(mut _req: tide::Request<Context>) -> tide::Result {
    let res = Body::from_json(&TestResult {
        status: "ok".to_string(),
    })?;
    Ok(res.into())
}
