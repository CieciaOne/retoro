use actix_web::{delete, get, post, web, HttpResponse, Responder, Result};
use chrono::Utc;
use log::{error, info};

use uuid::Uuid;

use crate::common::filter::Filter;
use crate::common::id::IdQuery;
use crate::post::model::Post;
use crate::post::schema::{AddPostRequest, PostResponse};
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
    let query_string = query.prepare(
        "SELECT 
            posts.id, 
            posts.thread_id, 
            COALESCE(posts.author_id, '00000000-0000-0000-0000-000000000000') AS author_id,
            CASE 
                WHEN posts.author_id IS NULL THEN 'Anonymous' 
                ELSE users.name 
            END AS author_name,
            posts.content, 
            posts.created_at
        FROM 
            posts
        LEFT JOIN 
            users 
        ON 
                posts.author_id = users.id"
            .to_string(),
    );
    let query_result: Vec<PostResponse> =
        match sqlx::query_as(&query_string).fetch_all(&data.db).await {
            Ok(users) => users,
            Err(err) => {
                error!("{err}");
                return Err(actix_web::error::ErrorInternalServerError(err));
            }
        };
    Ok(HttpResponse::Ok().json(query_result))
}

#[delete("")]
async fn delete(
    query: web::Query<IdQuery>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    match sqlx::query_as!(Post, "DELETE FROM posts WHERE id=$1;", query.id.clone())
        .execute(&data.db)
        .await
    {
        Ok(_) => {
            info!("Post {} deleted successfully", query.id);
            Ok(HttpResponse::Ok())
        }
        Err(err) => {
            error!("Deleting Post {} failed: {err}", query.id);
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

pub fn post_service(conf: &mut web::ServiceConfig) {
    let scope = web::scope("api/posts").service(add_post).service(get_posts);

    conf.service(scope);
}
