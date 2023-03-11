use std::sync::Mutex;
use std::env;

use chrono;

use actix_session::{
    Session,
    SessionMiddleware,
    storage::RedisActorSessionStore
};


use actix_web::{
    web,
    App,
    HttpServer,
    middleware,
    cookie::Key
};

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
        App::new()
            .app_data(db.clone())
            .app_data(web::JsonConfig::default().limit(4096))
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
            )
    })
        .bind(("127.0.0.1", 7100))?
        .run()
        .await
}
