use actix_web::{delete, get, post, web, HttpResponse, Responder, Result};
use chrono::Utc;
use log::{error, info};

use uuid::Uuid;

use crate::post::model::Post;
use crate::thread::model::Thread;
use crate::thread::schema::{AddThreadRequest, GetNThreadsRequest};
use crate::SharedState;

#[post("")]
async fn add_thread(
    body: web::Json<AddThreadRequest>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    match sqlx::query_as!(
        Post,
        "INSERT INTO threads VALUES($1,$2,$3);",
        Uuid::new_v4(),
        body.name,
        Utc::now(),
    )
    .execute(&data.db)
    .await
    {
        Ok(_) => {
            info!("Thread \"{}\" added successfully", body.name);
            Ok(HttpResponse::Ok())
        }
        Err(err) => {
            error!("{err}");
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

#[get("")]
async fn get_last_n_threads(
    data: web::Data<SharedState>,
    body: web::Json<GetNThreadsRequest>,
) -> Result<impl Responder> {
    let query_result = match sqlx::query_as!(
        Thread,
        "SELECT * FROM threads ORDER BY created_at LIMIT $1;",
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

#[get("/{id}")]
async fn get_thread_posts(
    id: web::Path<Uuid>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    let thread_id =
        Uuid::parse_str(&id.to_string()).map_err(|e| actix_web::error::ErrorBadRequest(e))?;
    let query_result = match sqlx::query_as!(
        Post,
        "SELECT * FROM posts WHERE thread_id=$1 ORDER BY created_at",
        thread_id
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

pub fn thread_service(conf: &mut web::ServiceConfig) {
    let scope = web::scope("api/thread")
        .service(add_thread)
        .service(get_last_n_threads);

    conf.service(scope);
}
