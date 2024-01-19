// src/lib/routes/info.rs

// dependencies
use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

// struct which embodies a description of the API paths available
#[derive(Serialize)]
struct Paths {
    create: String,
    retrieve: String,
    update_album: String,
    update_thoughts: String,
    delete: String,
}

// struct which embodies contact information
#[derive(Serialize)]
struct Contact {
    name: String,
    email: String,
}

// struct which embodies info describing this API
#[derive(Serialize)]
struct InfoResponse {
    openapi_version: String,
    title: String,
    summary: String,
    description: String,
    terms_of_service: String,
    contact: Contact,
    version: String,
    paths: Paths,
}

// info endpoint handler
#[tracing::instrument(name = "Info", skip())]
#[get("/info")]
async fn info() -> impl Responder {
    let api_paths = Paths {
        create: "/api/diary/new // adds a new entry, timestamp is automatically added (UTC time)".to_string(),
        retrieve: "/api/diary // retrieves and displays all entries in json format".to_string(),
        update_album: "api/diary/update/thoughts/{id} // accepts an update to the thoughts field, automatically adds timestamp (UTC time) of update".to_string(),
        update_thoughts: "api/diary/update/thoughts/{id} // accepts an update to the thoughts field, automatically adds timestamp (UTC time) of update".to_string(),
        delete: "api/diary/delete/{id} // deletes any entry by unique id".to_string(),
    };

    let contact_info = Contact {
        name: "Jeff Mitchell".to_string(),
        email: "crusty-rustacean@jeffmitchelldev.com".to_string(),
    };

    let info_response = InfoResponse {
        openapi_version: "3.0.0".to_string(),
        title: "Rivet Head API".to_string(),
        summary: "An API for tracking music listening habits".to_string(),
        description: "This is the server API for the Rivet Head app".to_string(),
        terms_of_service: "Coming soon...".to_string(),
        contact: contact_info,
        version: "1.36.1".to_string(),
        paths: api_paths,
    };

    HttpResponse::Ok().json(info_response)
}
