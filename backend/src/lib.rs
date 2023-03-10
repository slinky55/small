pub mod handlers;

use std::sync::Mutex;
use actix_web::web;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};

use entity::user;

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
pub struct FormCreds {
    pub email: String,
    pub password: String,
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

pub fn hash_str(s: &str) -> Option<String> {
    let bytes = s.as_bytes();

    let hash = bcrypt::hash(bytes, bcrypt::DEFAULT_COST);
    if hash.is_err() {
        eprintln!("Failed to hash password: {}", hash.unwrap_err());
        return None;
    }

    Some(hash.unwrap())
}
