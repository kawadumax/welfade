use actix_web::{get, post, web, Result ,Error, HttpResponse};
// DBPoolは注入、メソッドはtraitだけ参照したい
use crate::database::postgres::DbPool;
use crate::database::postgres;
use crate::gateway::models;


/// Finds user by UID.
#[get("/user/{user_id}")]
pub async fn get_user(
    pool: web::Data<DbPool>,
    params: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_id = params.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || postgres::find_user_by_id(user_id, &conn))
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
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUserModel>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || postgres::insert_new_user(&form.name, &form.email, &form.password , &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(user))
}