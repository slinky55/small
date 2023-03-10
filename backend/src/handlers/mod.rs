use std::collections::BTreeMap;
use std::ops::Deref;

use actix_web::{
    get,
    post,
    web,
    HttpResponse,
    Responder
};

use actix_web::cookie::{Cookie, Expiration, time::OffsetDateTime, time::Duration, SameSite};

use hmac::{Hmac, Mac};
use sha2::Sha256;
use jwt::SignWithKey;

use sea_orm::{EntityTrait};
use sea_orm::entity::prelude::*;

use entity::prelude::User;
use entity::user;

use crate::{
    NewUser,
    FormCreds,
    save_user,
    State
};

#[get("/ping")]
pub async fn ping() -> impl Responder {
    println!("GET /api/ping 200");
    HttpResponse::Ok().body("pong")
}

#[post("/user/create")]
pub async fn user_create(state: web::Data<State>,
                         user: web::Json<NewUser>) -> impl Responder
{
    let db = state.db.lock().unwrap();
    let res = save_user(&db, user).await;

    match res {
        Ok(_) => {
            println!("POST /api/user/create 200");
            HttpResponse::Ok().body("User created")
        },
        Err(err) => {
            println!("POST /api/user/create 500");
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

#[post("/user/login")]
pub async fn user_login(state: web::Data<State>,
                        user:  web::Json<FormCreds>) -> impl Responder
{
    let db = state.db.lock().unwrap();

    let form = user.into_inner();

    let user = User::find()
        .filter(user::Column::Email.eq(form.email))
        .one(db.deref())
        .await;

    if user.is_err() {
        eprintln!("Error: {}", user.unwrap_err());
        println!("POST /api/user/login 401");
        return HttpResponse::InternalServerError().body("Internal server error");
    }

    let user = user.unwrap();
    if user.is_none() {
        println!("POST /api/user/login 401");
        return HttpResponse::Unauthorized().body("Invalid credentials");
    }

    let user = user.unwrap();

    let verify = bcrypt::verify(form.password, &user.password).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        false
    });

    if !verify {
        println!("POST /api/user/login 401");
        return HttpResponse::Unauthorized().body("Invalid credentials");
    }

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();

    let mut claims = BTreeMap::new();
    claims.insert("id", user.id.to_string());
    claims.insert("name", user.name);
    claims.insert("email", user.email);

    let token = claims.sign_with_key(&key);
    if token.is_err() {
        eprintln!("Error: {}", token.unwrap_err());
        println!("POST /api/user/login 500");
        return HttpResponse::InternalServerError().body("Internal server error");
    }

    let token = token.unwrap();

    let exp =
        Expiration::from( OffsetDateTime::now_utc() + std::time::Duration::from_secs(3600));

    let cookie = Cookie::build("jwt", token)
        .domain("localhost")
        .path("/")
        .expires(exp)
        .max_age(Duration::hours(1))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .finish();

    println!("POST /api/user/login 200");
    HttpResponse::Ok().cookie(cookie).body("Logged in")
}

#[get("/user/me")]
pub async fn user_me(_state: web::Data<State>) -> impl Responder {
    HttpResponse::Ok().body("me")
}

#[get("/user/profile/{id}")]
pub async fn user_profile(path: web::Path<i32>,
                      state: web::Data<State>) -> impl Responder {
    let id = path.into_inner();
    let db = state.db.lock().unwrap();

    let user = User::find_by_id(id).one(db.deref()).await;

    match user {
        Ok(_) => {
            println!("GET /api/user/profile/{} 200", id);
            HttpResponse::Ok().body("user found")
        },
        Err(err) => {
            println!("GET /api/user/profile/{} 500", id);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}