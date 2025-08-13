mod api;

use actix_web::{web, App, HttpResponse, HttpServer};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use actix_cors::Cors;
use crate::api::routes::auth_routes::configure_auth_routes;
use crate::api::routes::rooms_routes::configure_rooms_routes;

#[derive(Clone)]
pub struct AppData {
    pub pool: Pool<Postgres>,
    pub jwt_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("SQITCH_TARGET").expect("SQITCH_TARGET must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app_data = AppData { pool, jwt_secret };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["*"])
            .allowed_header("content-type")
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(app_data.clone()))
            .configure(configure_routes)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(
                web::scope("/api")
                    .configure(configure_auth_routes)
                    .configure(configure_rooms_routes)
            )
            .service(
                web::scope("/ws")
                    .route("", web::get().to(|| HttpResponse::Ok()))
            )
    );
}