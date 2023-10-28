// src/lib/routse/health_check.rs

use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

// struct which embodies our health_check response in json
#[derive(Serialize)]
struct HealthCheckResponse {
    message: String,
}

// health_check endpoint
#[get("/health_check")]
async fn health_check() -> impl Responder {
    let health_check_response = HealthCheckResponse {
        message: String::from("OK"),
    };

    HttpResponse::Ok().json(health_check_response)
}
