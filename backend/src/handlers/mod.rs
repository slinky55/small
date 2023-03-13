use std::ops::Deref;

use actix_web::{
    get,
    post,
    web,
    HttpResponse,
    Responder
};

use actix_session::Session;
use argon2::{Argon2, PasswordHash, PasswordVerifier};

use sea_orm::{EntityTrait};
use sea_orm::entity::prelude::*;

use entity::prelude::{Post, User};
use entity::user;

use crate::{
    NewUser,
    NewPost,
    FormCreds,
    save_user,
    save_post,
    State,
    SessionData,
    JsonError
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
                        user:  web::Json<FormCreds>,
                        session: Session) -> impl Responder
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

    let hash = PasswordHash::new(&user.password);
    if hash.is_err() {
        eprintln!("Error: {}", hash.unwrap_err());
        println!("POST /api/user/login 500");
        return HttpResponse::InternalServerError().body("Internal server error");
    }
    let hash = hash.unwrap();

    let verify =
        Argon2::default().verify_password(form.password.as_bytes(), &hash);
    if verify.is_err() {
        eprintln!("Error: {}", verify.unwrap_err());
        println!("POST /api/user/login 500");
        return HttpResponse::InternalServerError().body("Internal server error");
    }

    let res = session.insert("id", &user.id);
    if res.is_err() {
        eprintln!("Error: {}", res.unwrap_err());
        println!("POST /api/user/login 500");
        return HttpResponse::InternalServerError().body("Internal server error");
    }
    res.unwrap();

    let res = session.insert("name", &user.name);
    if res.is_err() {
        eprintln!("Error: {}", res.unwrap_err());
        println!("POST /api/user/login 500");
        return HttpResponse::InternalServerError().body("Internal server error");
    }

    session.renew();

    println!("POST /api/user/login 200");
    HttpResponse::Ok().json(SessionData {
        id: user.id,
        name: user.name
    })
}

#[get("/user/logout")]
pub async fn user_logout(session: Session) -> impl Responder {
    session.purge();

    println!("GET /api/user/logout 200");
    HttpResponse::Ok().body("Logged out")
}

#[get("/user/me")]
pub async fn user_me(session: Session) -> impl Responder {
    let id = match session.get::<i32>("id") {
        Ok(id) => {
            if id.is_none() {
                println!("GET /api/user/me 401");
                return HttpResponse::Unauthorized().json(JsonError {
                    error: "Unauthorized".to_string()
                })
            }
            id.unwrap()
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            println!("GET /api/user/me 500");
            return HttpResponse::Unauthorized().json(JsonError {
                error: "Internal server error".to_string()
            })
        }
    };

    let name = match session.get::<String>("name") {
        Ok(name) => {
            if name.is_none() {
                println!("GET /api/user/me 401");
                return HttpResponse::Unauthorized().json(JsonError {
                    error: "Unauthorized".to_string()
                })
            }
            name.unwrap()
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            println!("GET /api/user/me 500");
            return HttpResponse::Unauthorized().json(JsonError {
                error: "Internal server error".to_string()
            })
        }
    };

    println!("GET /api/user/me 200");
    HttpResponse::Ok().json(SessionData {
        id,
        name
    })
}

#[get("/user/profile/{id}")]
pub async fn user_profile(path: web::Path<i32>,
                          state: web::Data<State>) -> impl Responder
{
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

#[get("/user/posts")]
pub async fn user_posts(session: Session,
                        state: web::Data<State>) -> impl Responder
{
    let db = state.db.lock().unwrap();

    let id = match session.get::<i32>("id") {
        Ok(id) => {
            if id.is_none() {
                println!("GET /api/user/me 401");
                return HttpResponse::Unauthorized().json(JsonError {
                    error: "Unauthorized".to_string()
                })
            }
            id.unwrap()
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            println!("GET /api/user/me 401");
            return HttpResponse::Unauthorized().json(JsonError {
                error: "Unauthorized".to_string()
            })
        }
    };

    let user = match User::find_by_id(id).one(db.deref()).await {
        Ok(model) => {
            if model.is_none() {
                println!("GET /api/user/me 401");
                return HttpResponse::Unauthorized().json(JsonError {
                    error: "Unauthorized".to_string()
                })
            }
            model.unwrap()
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            println!("GET /api/user/me 401");
            return HttpResponse::Unauthorized().json(JsonError {
                error: "Unauthorized".to_string()
            })
        }
    };

    let posts = user.find_related(Post).all(db.deref()).await;

    match posts {
        Ok(posts) => {
            println!("GET /api/user/posts 200");
            HttpResponse::Ok().json(posts)
        },
        Err(err) => {
            println!("GET /api/user/posts 500");
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}


#[post("/post/create")]
pub async fn post_create(session: Session,
                         state: web::Data<State>,
                         post: web::Json<NewPost>) -> impl Responder
{
    let db = state.db.lock().unwrap();

    let id = match session.get::<i32>("id") {
        Ok(id) => {
            if id.is_none() {
                println!("GET /api/user/me 401");
                return HttpResponse::Unauthorized().json(JsonError {
                    error: "Unauthorized".to_string()
                })
            }
            id.unwrap()
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            println!("GET /api/user/me 401");
            return HttpResponse::Unauthorized().json(JsonError {
                error: "Unauthorized".to_string()
            })
        }
    };

    let res = save_post(&db, post, id).await;

    match res {
        Ok(_) => {
            println!("POST /api/post/create 200");
            HttpResponse::Ok().body("Post created")
        },
        Err(err) => {
            println!("Error: {}", err);
            println!("POST /api/post/create 500");
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

#[get("/post/get")]
pub async fn post_get(state: web::Data<State>) -> impl Responder {
    let db = state.db.lock().unwrap();

    let posts = Post::find().all(db.deref()).await;

    match posts {
        Ok(posts) => {
            println!("GET /api/post/get 200");
            HttpResponse::Ok().json(posts)
        },
        Err(err) => {
            println!("Error: {}", err);
            println!("GET /api/post/get 500");
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

#[get("/post/get/{id}")]
pub async fn post_get_id(state: web::Data<State>,
                         path: web::Path<i32>) -> impl Responder
{
    let db = state.db.lock().unwrap();

    let id = path.into_inner();

    let post = Post::find_by_id(id).one(db.deref()).await;

    match post {
        Ok(post) => {
            if post.is_none() {
                println!("GET /api/post/get/{} 404", id);
                return HttpResponse::NotFound().body("Post not found")
            }
            println!("GET /api/post/get/{} 200", id);
            HttpResponse::Ok().json(post.unwrap())
        },
        Err(err) => {
            println!("Error: {}", err);
            println!("GET /api/post/get/{} 500", id);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}