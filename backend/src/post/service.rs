use actix_web::{delete, get, post, web, HttpResponse, Responder, Result};
use chrono::Utc;
use log::{error, info};

use sqlx::query;
use uuid::Uuid;

use crate::common::common::Filter;
use crate::post::model::Post;
use crate::post::schema::AddPostRequest;
use crate::SharedState;

#[post("")]
async fn add_post(
    body: web::Json<AddPostRequest>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    match sqlx::query_as!(
        Post,
        "INSERT INTO posts VALUES($1,$2,$3,$4,$5) RETURNING *;",
        Uuid::new_v4(),
        body.thread_id,
        body.author_id,
        body.content,
        Utc::now(),
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(post) => {
            info!("Post {} added successfully", body.content);
            Ok(HttpResponse::Created().json(post))
        }
        Err(err) => {
            error!("{err}");
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

#[get("")]
async fn get_posts(
    data: web::Data<SharedState>,
    query: web::Query<Filter>,
) -> Result<impl Responder> {
    let query_string = query.prepare_query("posts".to_string());
    let query_result: Vec<Post> = match sqlx::query_as(&query_string).fetch_all(&data.db).await {
        Ok(users) => users,
        Err(err) => {
            error!("{err}");
            Vec::new()
        }
    };
    Ok(HttpResponse::Ok().json(query_result))
}

#[delete("/{id}")]
async fn delete_post(id: web::Path<Uuid>, data: web::Data<SharedState>) -> Result<impl Responder> {
    match sqlx::query_as!(Post, "DELETE FROM posts WHERE id=$1;", id.clone())
        .execute(&data.db)
        .await
    {
        Ok(_) => {
            info!("Post {id} deleted successfully");
            Ok(HttpResponse::Ok())
        }
        Err(err) => {
            error!("Deleting Post {id} failed: {err} ");
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

pub fn post_service(conf: &mut web::ServiceConfig) {
    let scope = web::scope("api/post").service(add_post).service(get_posts);

    conf.service(scope);
}
