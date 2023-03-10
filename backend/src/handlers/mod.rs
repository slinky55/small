use std::ops::Deref;

use actix_web::{
    get,
    post,
    web,
    HttpResponse,
    Responder
};

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
        return HttpResponse::InternalServerError();
    }

    let user = user.unwrap();
    if user.is_none() {
        println!("POST /api/user/login 401");
        return HttpResponse::Unauthorized();
    }

    let user = user.unwrap();
    if form.password != user.password {
        println!("POST /api/user/login 401");
        return HttpResponse::Unauthorized();
    }

    println!("POST /api/user/login 200");
    HttpResponse::Ok()
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