use std::sync::Mutex;
use std::env;

use actix_session::{
    SessionMiddleware,
    storage::RedisActorSessionStore
};

use actix_web::{
    http,
    web,
    App,
    HttpServer,
    middleware,
    cookie::Key
};

use actix_cors::Cors;

use sea_orm::Database;

use backend::{
    State,
    handlers
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        eprintln!("DATABASE_URL is not set");
        std::process::exit(1);
    });

    let secret = Key::generate();

    let db = web::Data::new(State {
        db: Mutex::new(Database::connect(&url).await.unwrap_or_else(|err| {
           eprintln!("Failed to connect to database: {}", err);
           std::process::exit(1);
        }))
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
            .allowed_header(http::header::AUTHORIZATION)
            .allowed_headers(vec![http::header::COOKIE, http::header::SET_COOKIE])
            .supports_credentials()
            .max_age(3600);


        App::new()
            .app_data(db.clone())
            .app_data(web::JsonConfig::default().limit(4096))
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(
                SessionMiddleware::new(
                    RedisActorSessionStore::new("127.0.0.1:6379"),
                    secret.clone()
                )
            )
            .service(
                web::scope("/api")
                .service(handlers::ping)
                .service(handlers::user_create)
                .service(handlers::user_me)
                .service(handlers::user_profile)
                .service(handlers::user_login)
                .service(handlers::user_logout)
                .service(handlers::post_create)
                .service(handlers::post_get)
                .service(handlers::user_posts)
                .service(handlers::post_get_id)
            )
    })
        .bind(("localhost", 7100))?
        .run()
        .await
}
