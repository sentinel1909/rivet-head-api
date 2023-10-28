//src/lib/middleware/auth_key_middleware.rs

// dependencies
use crate::domain::appstate::AppState;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error,
};
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

// struct to represent the API key
#[derive(Debug, Clone)]
pub struct ApiKey;

// implement the transform trait for the ApiKey struct
impl<S, B> Transform<S, ServiceRequest> for ApiKey
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ApiKeyMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    // create a new instance of the ApiKeyMiddleware struct
    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyMiddleware { service }))
    }
}

// struct to represent the ApiKeyMiddleware
pub struct ApiKeyMiddleware<S> {
    service: S,
}

// implement the service trait for the ApiKeyMiddleware struct
impl<S, B> Service<ServiceRequest> for ApiKeyMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // get the x-api-key header from the request
        let x_api_key = req
            .headers()
            .get("x-api-key")
            .and_then(|value| value.to_str().ok());

        // get the api key from the app state
        let api_key = req
            .app_data::<Data<AppState>>()
            .map(|data| data.api_key.clone());

        // check if the api key is valid
        if x_api_key != api_key.as_deref() {
            return Box::pin(async move {
                Err(actix_web::error::ErrorUnauthorized(
                    "Unauthorized: Invalid API key",
                ))
            });
        }

        // call the service
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
