use actix_web::{cookie::time::Duration, get, post, web, HttpResponse, Responder, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::Utc;
use log::{debug, error, info};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::user::schema::{SessionAuthRequest, UserAuthRequest};
use crate::{user::model::User, SharedState};

use super::error::Error;

#[post("register")]
async fn register_user(
    body: web::Json<UserAuthRequest>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Failed hashing")
        .to_string();

    let salt_str = salt.to_string();

    match sqlx::query_as!(
        User,
        "INSERT INTO users VALUES($1,$2,$3,$4,$5,$6) RETURNING *;",
        Uuid::new_v4(),
        body.name,
        password_hash,
        Utc::now(),
        Utc::now(),
        salt_str
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(user) => {
            info!("User {} registered successfully", body.name);
            let session_id = uuid::Uuid::new_v4();
            let session_id_string = session_id.to_string();
            let cookie = actix_web::cookie::Cookie::build("session_id", session_id_string)
                .max_age(Duration::days(3))
                .path("/")
                .finish();
            data.user_sessions.lock().await.insert(session_id, user.id);
            Ok(HttpResponse::Created()
                .cookie(cookie)
                .json(user.as_reponse()))
        }
        Err(err) => {
            error!("{err}");
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

#[post("login")]
async fn login_user(
    body: web::Json<UserAuthRequest>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    match auth_user(body.name.clone(), body.password.clone(), data.db.clone()).await {
        Ok(user) => {
            let session_id = uuid::Uuid::new_v4();
            let session_id_string = session_id.to_string();
            let cookie = actix_web::cookie::Cookie::build("session_id", session_id_string)
                .max_age(Duration::days(3))
                .path("/")
                .finish();
            data.user_sessions.lock().await.insert(session_id, user.id);

            Ok(HttpResponse::Ok().cookie(cookie).json(user.as_reponse()))
        }
        Err(e) => Err(actix_web::error::ErrorUnauthorized(e)),
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

#[post("/auth")]
async fn auth_session(
    body: web::Json<SessionAuthRequest>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    debug!("Sessions{:?}", data.user_sessions.lock().await);
    if let Some(user_id) = data.user_sessions.lock().await.get(&body.session_id) {
        match sqlx::query_as!(User, "SELECT * FROM users WHERE(id = $1);", user_id)
            .fetch_one(&data.db)
            .await
        {
            Ok(user) => Ok(HttpResponse::Ok().json(user.as_reponse())),
            Err(_) => {
                error!("User with id {} does not exist", user_id);
                Err(actix_web::error::ErrorNotFound("Invalid session"))
            }
        }
    } else {
        Err(actix_web::error::ErrorNotFound("Invalid session"))
    }
}

async fn auth_user(username: String, password: String, db: Pool<Postgres>) -> Result<User, Error> {
    match sqlx::query_as!(User, "SELECT * FROM users WHERE(name LIKE $1);", username)
        .fetch_one(&db)
        .await
    {
        Ok(user) => {
            debug!("User {} found.", username);
            let salt = SaltString::from_b64(&user.salt).expect("Failed decoding salt");
            let argon2 = Argon2::default();
            let password_hash = argon2
                .hash_password(password.as_bytes(), &salt)
                .expect("Failed hashing")
                .to_string();
            if user.password_hash == password_hash {
                Ok(user)
            } else {
                Err(Error::AuthFailed)
            }
        }
        Err(_) => {
            error!("User {} does not exist", username);
            Err(Error::UserNotFound)
        }
    }
}

pub fn user_service(conf: &mut web::ServiceConfig) {
    let scope = web::scope("api/users")
        .service(register_user)
        .service(login_user)
        .service(get_users)
        .service(auth_session);

    conf.service(scope);
}
