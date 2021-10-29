use serde_json::Value;
use std::error::Error;
use std::ops::Add;

pub struct ToxicClient {
    url: String,
    client: reqwest::Client,
}

impl ToxicClient {
    pub fn new(url: String) -> ToxicClient {
        ToxicClient {
            url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn post(&self, method: String, value: Value) -> Result<Value, Box<dyn Error>> {
        let url = self.url.clone().add("/api/").add(&method);

        let req = self.client.post(url).json(&value);
        let resp = req.send().await?.json::<Value>().await?;
        Ok(resp)
    }
}
