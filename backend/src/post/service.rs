use actix_web::{delete, get, post, web, HttpResponse, Responder, Result};
use chrono::Utc;
use log::{error, info};

use uuid::Uuid;

use crate::post::model::Post;
use crate::post::schema::{AddPostSchema, GetNPostsSchema};
// use crate::user::model::User;
use crate::SharedState;

#[post("register")]
async fn add_post(
    body: web::Json<AddPostSchema>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    // pub id: Uuid,
    // pub thread_id: Uuid,
    // pub author_id: Uuid,
    // pub content: String,
    // pub created_at: DateTime<Utc>,
    match sqlx::query_as!(
        Post,
        "INSERT INTO posts VALUES($1,$2,$3,$4,$5);",
        Uuid::new_v4(),
        body.thread_id,
        body.author_id,
        body.content,
        Utc::now(),
    )
    .execute(&data.db)
    .await
    {
        Ok(_) => {
            info!("Post {} added successfully", body.content);
            Ok(HttpResponse::Ok())
        }
        Err(err) => {
            error!("{err}");
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

#[get("")]
async fn get_last_n_posts(
    data: web::Data<SharedState>,
    body: web::Json<GetNPostsSchema>,
) -> Result<impl Responder> {
    let query_result = match sqlx::query_as!(
        Post,
        "SELECT * FROM posts ORDER BY created_at LIMIT $1;",
        body.n
    )
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

pub fn post_service(conf: &mut web::ServiceConfig) {
    let scope = web::scope("api/post")
        .service(add_post)
        .service(get_last_n_posts);

    conf.service(scope);
}
