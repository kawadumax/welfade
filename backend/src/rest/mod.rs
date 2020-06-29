use actix_web::{get, post, web, Result ,Error, HttpResponse};

use crate::database::postgres::PostgresManager;
use crate::gateway::models;
use crate::gateway::repository::Repository;

// #[derive(Debug)]
struct RestManager {
    db_manager: PostgresManager
}

#[get("/user/{user_id}")]
pub async fn get_user(
    params: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_id = params.into_inner();


    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || PostgresManager::find_user_by_id(user_id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No user found with id: {}", user_id));
        Ok(res)
    }
}

/// Inserts new user with name defined in form.
#[post("/user")]
pub async fn add_user(
    params: web::Json<models::NewUserModel>,
) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || PostgresManager::insert_new_user(params.into_inner()))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(user))
}