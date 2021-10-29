use crate::toxic::ToxicClient;
use actix_web::web::Json;
use actix_web::{web, HttpResponse, Responder};
use lazy_static::lazy_static;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashSet;

#[derive(Serialize)]
struct TestResult {
    status: String,
}

pub async fn get() -> HttpResponse {
    HttpResponse::Ok().json(TestResult {
        status: "ok".to_string(),
    })
}
