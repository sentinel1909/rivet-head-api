// src/lib/routse/health_check.rs

use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

// struct which embodies our health_check response in json
#[derive(Serialize)]
struct HealthCheckResponse {
    code: u8,
    message: String,
}

// health_check endpoint
#[tracing::instrument(name = "Health Check", skip())]
#[get("/health_check")]
async fn health_check() -> impl Responder {
    let health_check_response = HealthCheckResponse {
        code: 200,
        message: "Ok".to_string(),
    };

    HttpResponse::Ok().json(health_check_response)
}
