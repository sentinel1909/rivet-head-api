// src/main.rs

// dependencies
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::http::header;
use actix_web::web::{self, ServiceConfig};
use anyhow::anyhow;
use rivet_head_api_lib::domain::appstate::AppState;
use rivet_head_api_lib::instrumentation::{get_subscriber, init_subscriber};
use rivet_head_api_lib::middleware::ApiKey;
use rivet_head_api_lib::routes::{
    diary_album_put, diary_delete, diary_get, diary_post, diary_thoughts_put, health_check, info,
    not_found,
};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use shuttle_secrets::SecretStore;
use sqlx::{Executor, PgPool};
use tracing::info;
use tracing_actix_web::TracingLogger;

// main function, annotated for Shuttle
#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // initialize telemetry
    let subscriber = get_subscriber("rivet-head-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // run database migrations
    info!("Running the database migrations...");
    pool.execute(include_str!("../../schema/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    // load the API key from Secrets.toml
    info!("Loading the API key...");
    let api_key = if let Some(secret) = secret_store.get("RH_API_KEY") {
        secret
    } else {
        return Err(anyhow!("The API key was not found, unable to start the API.").into());
    };

    // create the app state to hold the database pool and the API key
    info!("Creating the app state...");
    let state = web::Data::new(AppState { pool, api_key });

    // create the cross-origin resource sharing config and app routes
    info!("Creating CORS configuration...");
    let config = move |cfg: &mut ServiceConfig| {
        // create cross-origin resource sharing config
        let cors_conf = Cors::default()
            .allowed_origin("https://rivet-head-api.shuttleapp.rs")
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".shuttleapp.rs"))
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        // governor configuration
        info!("Creating governor configuration...");
        let governor_conf = GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(5)
            .finish()
            .expect("Unable to create governor configuration, unable to start API.");

        // load tracing, cors, API key, and governor middleware, create app routes
        cfg.service(
            web::scope("/api")
                .wrap(TracingLogger::default())
                .wrap(cors_conf)
                .wrap(ApiKey)
                .wrap(Governor::new(&governor_conf))
                .service(info)
                .service(health_check)
                .service(diary_delete)
                .service(diary_get)
                .service(diary_post)
                .service(diary_album_put)
                .service(diary_thoughts_put)
                .default_service(web::route().to(not_found))
                .app_data(state),
        );
    };

    // spin up the API
    info!("Rivet Head API is running!");
    Ok(config.into())
}
