// src/lib/routes/not_found.rs

// dependencies
use actix_web::{HttpResponse, Result};
use serde::Serialize;

// struct which embodies our not_found response in json
#[derive(Serialize)]
struct NotFoundResponse {
    message: String,
}

pub async fn not_found() -> Result<HttpResponse> {
    let not_found_response = NotFoundResponse {
        message: "Resource not found".to_string(),
    };

    Ok(HttpResponse::NotFound().json(not_found_response))
}
