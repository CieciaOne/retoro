use actix_web::{get, post, web, HttpResponse, Responder, Result};
use chrono::Utc;
use log::{error, info};

use uuid::Uuid;

use crate::common::common::Filter;
use crate::post::model::Post;
use crate::thread::model::Thread;
use crate::thread::schema::AddThreadRequest;
use crate::SharedState;

#[post("")]
async fn add_thread(
    body: web::Json<AddThreadRequest>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    match sqlx::query_as!(
        Thread,
        "INSERT INTO threads VALUES($1,$2,$3) RETURNING *;",
        Uuid::new_v4(),
        body.name,
        Utc::now(),
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(result) => {
            info!("Thread \"{}\" added successfully", body.name);
            Ok(HttpResponse::Created().json(result))
        }
        Err(err) => {
            error!("{err}");
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

#[get("")]
async fn get_threads(
    data: web::Data<SharedState>,
    query: web::Query<Filter>,
) -> Result<impl Responder> {
    let query_string = query.prepare_query("threads".to_string());
    let query_result: Vec<Thread> = match sqlx::query_as(&query_string).fetch_all(&data.db).await {
        Ok(users) => users,
        Err(err) => {
            error!("{err}");
            return Err(actix_web::error::ErrorInternalServerError(err));
        }
    };
    Ok(HttpResponse::Ok().json(query_result))
}

#[get("/{id}")]
async fn get_thread_posts(
    id: web::Path<Uuid>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    let thread_id = Uuid::parse_str(&id.to_string()).map_err(actix_web::error::ErrorBadRequest)?;
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
        .service(get_threads);

    conf.service(scope);
}
