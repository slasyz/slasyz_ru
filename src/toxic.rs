use serde_json::Value;
use std::error::Error;
use std::ops::Add;

pub struct ToxicClient {
    url: String,
    client: surf::Client,
}

impl ToxicClient {
    pub fn new(url: String) -> ToxicClient {
        ToxicClient {
            url,
            client: surf::Client::new(),
        }
    }

    pub async fn post(&self, method: String, value: Value) -> Result<Value, Box<dyn Error>> {
        let url = self.url.clone().add("/api/").add(&method);

        let resp = self.client.post(url).body_json(&value)?.recv_json().await?;
        Ok(resp)
    }
}
