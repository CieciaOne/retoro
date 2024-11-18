use actix_web::{delete, get, post, web, HttpResponse, Responder, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use log::{error, info};
use uuid::Uuid;

use crate::user::schema::AuthneticateUserSchema;
use crate::{user::model::User, SharedState};

#[post("register")]
async fn register_user(
    body: web::Json<AuthneticateUserSchema>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Failed hashing")
        .to_string();

    let salt_str = salt.as_str();

    match sqlx::query_as!(
        RegisterUserSchema,
        "INSERT INTO users VALUES($1,$2,$3,$4,$5);",
        Uuid::new_v4(),
        body.name,
        password_hash,
        Utc::now(),
        salt_str
    )
    .execute(&data.db)
    .await
    {
        Ok(_) => {
            info!("User {} registered successfully", body.name);
            Ok(HttpResponse::Ok())
        }
        Err(err) => {
            error!("{err}");
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

#[post("login")]
async fn login_user(
    body: web::Json<AuthneticateUserSchema>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Failed hashing")
        .to_string();

    let s = salt.as_str();

    match sqlx::query_as!(User, "SELECT * FROM users WHERE(name LIKE $1);", body.name)
        .fetch_all(&data.db)
        .await
    {
        Ok(_) => {
            info!("User {} logged in successfully", body.name);
            Ok(HttpResponse::Ok())
        }
        Err(err) => {
            error!("{err}");
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

#[get("")]
async fn get_users(data: web::Data<SharedState>) -> Result<impl Responder> {
    let query_result = match sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&data.db)
        .await
    {
        Ok(users) => users,
        Err(err) => {
            error!("{err}");
            Vec::new()
        }
    };
    Ok(HttpResponse::Ok().json(query_result))
}

#[delete("/{id}")]
async fn delete_user(id: web::Path<Uuid>, data: web::Data<SharedState>) -> Result<impl Responder> {
    match sqlx::query_as!(User, "DELETE FROM users WHERE id=$1;", id.clone())
        .execute(&data.db)
        .await
    {
        Ok(_) => {
            info!("User {id} deleted successfully");
            Ok(HttpResponse::Ok())
        }
        Err(err) => {
            error!("Deleting user {id} failed: {err} ");
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

pub fn user_service(conf: &mut web::ServiceConfig) {
    let scope = web::scope("api/user")
        .service(register_user)
        .service(get_users);

    conf.service(scope);
}
