use actix_web::{HttpResponse, Responder};
use actix_web::web::{Json, Path};
use crate::api::utils::api_errors::ApiError;

pub async fn handle_get_rooms() -> Result<impl Responder, ApiError> {
    Ok(HttpResponse::NotImplemented().json("Get /Rooms"))
}

pub async fn handle_post_rooms() -> Result<impl Responder, ApiError> {
    Ok(HttpResponse::NotImplemented().json("Post /Rooms"))
}

pub async fn handle_get_room(room_id: Path<i64>) -> Result<impl Responder, ApiError> {
    Ok(HttpResponse::NotImplemented().json(format!("Get /Rooms/{}", room_id.into_inner())))
}

pub async fn handle_put_room(room_id: Path<i64>) -> Result<impl Responder, ApiError> {
    Ok(HttpResponse::NotImplemented().json(format!("Put /Rooms/{}", room_id.into_inner())))
}