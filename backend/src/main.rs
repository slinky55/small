use std::sync::Mutex;
use std::env;

use actix_web::{
    web,
    App,
    HttpServer,
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

    let db = web::Data::new(State {
        db: Mutex::new(Database::connect(&url).await.unwrap_or_else(|err| {
           eprintln!("Failed to connect to database: {}", err);
           std::process::exit(1);
        }))
    });

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(
                web::scope("/api")
                .service(handlers::ping)
                .service(handlers::user_create)
                .service(handlers::user_me)
                .service(handlers::user_profile)
            )
    })
        .bind(("127.0.0.1", 7100))?
        .run()
        .await
}
