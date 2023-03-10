pub mod handlers;

use std::sync::Mutex;
use actix_web::web;
use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use argon2::{Argon2};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};

use entity::{post, user};

pub struct State {
    pub db: Mutex<DatabaseConnection>
}

#[derive(serde::Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct NewPost {
    pub title: String,
    pub content: String,
}

#[derive(serde::Deserialize)]
pub struct FormCreds {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct SessionData {
    pub id: i32,
    pub name: String,
}

#[derive(serde::Serialize)]
pub struct JsonSuccess {
    pub success: String
}

#[derive(serde::Serialize)]
pub struct JsonError {
    pub error: String
}

pub async fn save_user(db: &DatabaseConnection,
                       user: web::Json<NewUser>) -> Result<(), DbErr>
{
    let user = user.into_inner();

    let pass = hash_str(&user.password);
    if pass.is_none() {
        return Err(DbErr::Custom("Failed to hash password".to_string()));
    }
    
    let user = user::ActiveModel {
        name: Set(user.name),
        email: Set(user.email),
        password: Set(pass.unwrap()),
        ..Default::default()
    };

    user.save(db).await?;

    Ok(())
}

pub async fn save_post(db: &DatabaseConnection,
                       post: web::Json<NewPost>,
                       id: i32) -> Result<(), DbErr>
{
    let post = post.into_inner();

    let post = post::ActiveModel {
        title: Set(post.title),
        text: Set(post.content),
        user_id: Set(id),
        ..Default::default()
    };

    post.save(db).await?;

    Ok(())
}

pub fn hash_str(s: &str) -> Option<String> {
    let bytes = s.as_bytes();

    let argon = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hash = argon.hash_password(bytes, &salt);

    if hash.is_err() {
        eprintln!("Failed to hash password: {}", hash.unwrap_err());
        return None;
    }

    Some(hash.unwrap().to_string())
}
