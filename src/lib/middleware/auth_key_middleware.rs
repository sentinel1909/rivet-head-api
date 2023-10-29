//src/lib/middleware/auth_key_middleware.rs

// dependencies
use crate::domain::appstate::AppState;
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, HttpResponse,
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
    type Response = ServiceResponse<EitherBody<B>>;
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
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        // get the x-api-key header from the request
        let x_api_key = request
            .headers()
            .get("x-api-key")
            .and_then(|value| value.to_str().ok());

        // get the api key from the app state
        let api_key = request
            .app_data::<Data<AppState>>()
            .map(|data| data.api_key.clone());

        // check if the api key is valid
        if x_api_key != api_key.as_deref() {
            // return a 401 unauthorized response
            let (request, _payload) = request.into_parts();
            let response = HttpResponse::Unauthorized().finish().map_into_right_body();
            return Box::pin(async move { Ok(ServiceResponse::new(request, response)) });
        }

        // return the request if the API key is valid
        let response = self.service.call(request);
        Box::pin(async move { response.await.map(ServiceResponse::map_into_left_body) })
    }
}
