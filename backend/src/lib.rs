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

pub async fn save_user(db: &DatabaseConnection, user: web::Json<NewUser>) -> Result<(), DbErr> {
    let user = user.into_inner();
    
    let user = user::ActiveModel {
        name: Set(user.name),
        email: Set(user.email),
        password: Set(user.password),
        ..Default::default()
    };

    user.save(db).await?;

    Ok(())
}