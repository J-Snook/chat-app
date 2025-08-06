use actix_web::web;
use crate::api::handlers::auth_handlers::handle_register;

pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(
                web::resource("/register")
                    .route(web::post().to(handle_register))
            )
    );
}