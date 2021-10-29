use crate::toxic::ToxicClient;
use actix_web::web::Json;
use actix_web::{web, HttpResponse, Responder};
use lazy_static::lazy_static;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashSet;

lazy_static! {
    static ref METHODS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("test");
        set.insert("chat_spotify");
        set.insert("chat_shazam");
        set
    };
}

pub struct ToxicHandlerData {
    client: ToxicClient,
}

impl ToxicHandlerData {
    pub fn new(client: ToxicClient) -> ToxicHandlerData {
        ToxicHandlerData { client }
    }
}

#[derive(Serialize)]
struct Error {
    error: String,
}

pub async fn post(
    data: web::Data<ToxicHandlerData>,
    method: web::Path<String>,
    body: Json<Value>,
) -> HttpResponse {
    if !METHODS.contains(method.as_str()) {
        return HttpResponse::NotFound().json(Error {
            error: "Method not found.".to_string(),
        });
    }

    match data
        .client
        .post(method.into_inner(), body.into_inner())
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => HttpResponse::InternalServerError().json(Error {
            error: err.to_string(),
        }),
    }
}
