use actix_web::{
    get,
    post,
    web,
    App,
    HttpResponse,
    HttpServer,
    Responder
};

#[get("/ping")]
async fn ping() -> impl Responder {
    println!("GET /api/ping 200");
    HttpResponse::Ok().body("pong")
}

#[post("/user/create")]
async fn user_create() -> impl Responder {
    println!("POST /api/user/create 200");
    HttpResponse::Ok().body("user created")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                .service(ping)
            )
    })
        .bind(("127.0.0.1", 7100))?
        .run()
        .await
}
