mod api;

use actix_web::{web, App, HttpResponse, HttpServer};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use actix_cors::Cors;
use crate::api::routes::auth_routes::configure_auth_routes;

#[derive(Clone)]
struct AppData {
    pool: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app_data = AppData { pool };

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
            )
            .service(
                web::scope("/ws")
                    .route("", web::get().to(|| HttpResponse::Ok()))
            )
    );
}