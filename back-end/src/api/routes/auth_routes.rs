use actix_web::web;
use crate::api::handlers::auth_handlers::{handle_login, handle_logout, handle_me, handle_refresh, handle_register};
use crate::api::middlewares::logged_in_middleware::IsLoggedIn;

pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(
                web::resource("/register")
                    .route(web::post().to(handle_register))
            )
            .service(
                web::resource("/login")
                    .route(web::post().to(handle_login))
            )
            .service(
                web::resource("/logout")
                    .wrap(IsLoggedIn)
                    .route(web::post().to(handle_logout))
            )
            .service(
                web::resource("/refresh")
                    .route(web::post().to(handle_refresh))
            )
            .service(
                web::resource("/me")
                    .wrap(IsLoggedIn)
                    .route(web::get().to(handle_me))
            )
    );
}