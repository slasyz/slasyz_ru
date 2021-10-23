use lazy_static::lazy_static;
use std::collections::HashSet;
use warp::{Rejection, Reply};

lazy_static! {
    static ref METHODS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("test");
        set.insert("chat_spotify");
        set.insert("chat_shazam");
        set
    };
}

pub struct ToxicHandler {}

impl ToxicHandler {
    pub fn new() -> ToxicHandler {
        ToxicHandler {}
    }

    pub async fn call(&mut self, method: String) -> Result<impl Reply, Rejection> {
        if !METHODS.contains(method.as_str()) {
            return Err(warp::reject::not_found());
        }

        // TODO: request to the bot

        return Ok(warp::reply::html("blabla"));
    }
}
