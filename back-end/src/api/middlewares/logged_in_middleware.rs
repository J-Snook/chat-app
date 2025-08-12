use std::future::{ready, Ready};
use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage, ResponseError};
use actix_web::body::EitherBody;
use actix_web::dev::forward_ready;
use actix_web::web::Data;
use futures_util::future::LocalBoxFuture;
use crate::api::services::jwt_services::JwtService;
use crate::api::utils::api_errors::ApiError;
use crate::AppData;

pub struct IsLoggedIn;

impl<S,B> Transform<S, ServiceRequest> for IsLoggedIn
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>,Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = IsLoggedInMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(IsLoggedInMiddleware { service }))
    }
}

pub struct IsLoggedInMiddleware<S> {
    service: S,
}

impl<S,B> Service<ServiceRequest> for IsLoggedInMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = match req.cookie("access_token") {
            Some(t) => t.value().to_string(),
            None => {
                let (request, _pl) = req.into_parts();
                let response = ApiError::Unauthorized("Invalid or Expired Access Token".to_string());
                let response = response.error_response().map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response))});
            }
        };

        let app_data = match req.app_data::<Data<AppData>>() {
            Some(data) => data,
            None => {
                let (request, _pl) = req.into_parts();
                let err = ApiError::Other(String::from("App data missing in middleware"));
                let response = err.error_response().map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response))});
            }
        };

        let claims = match JwtService::verify_token(&app_data.jwt_secret,&token) {
            Ok(claims) => claims,
            Err(err) => {
                let (request, _pl) = req.into_parts();
                let response = err.error_response().map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response))});
            }
        };

        req.extensions_mut().insert(claims);
        let fut = self.service.call(req);
        Box::pin(async move { fut.await.map(ServiceResponse::map_into_left_body)})
    }
}
