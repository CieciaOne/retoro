use actix_web::{delete, get, post, web, HttpResponse, Responder, Result};
use chrono::Utc;
use log::{debug, error, info};

use uuid::Uuid;

use crate::common::filter::Filter;
use crate::common::id::IdQuery;
use crate::thread::model::Thread;
use crate::thread::schema::AddThreadRequest;
use crate::SharedState;

#[post("")]
async fn add(
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
async fn get(data: web::Data<SharedState>, query: web::Query<Filter>) -> Result<impl Responder> {
    let query_string = query.prepare("SELECT * FROM threads".to_string());
    let query_result: Vec<Thread> = match sqlx::query_as(&query_string).fetch_all(&data.db).await {
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
    debug!("{}", query.id);
    match sqlx::query_as!(
        Thread,
        "DELETE FROM threads WHERE id=$1 RETURNING *;",
        query.id.clone()
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(thread) => {
            info!("Post {} deleted successfully", query.id);
            Ok(HttpResponse::Ok().json(thread))
        }
        Err(err) => {
            error!("Deleting Post {} failed: {err}", query.id);
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

pub fn thread_service(conf: &mut web::ServiceConfig) {
    let scope = web::scope("api/threads")
        .service(add)
        .service(get)
        .service(delete);

    conf.service(scope);
}
