use actix_web::web;
use crate::api::handlers::rooms_handlers::{handle_get_room, handle_get_rooms, handle_post_rooms, handle_put_room};

pub fn configure_rooms_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/rooms")
            .service(
                web::resource("")
                    .route(web::get().to(handle_get_rooms))
                    .route(web::post().to(handle_post_rooms))
            )
            .service(
                web::resource("/{room_id}")
                    .route(web::get().to(handle_get_room))
                    .route(web::put().to(handle_put_room))
            )
    );
}