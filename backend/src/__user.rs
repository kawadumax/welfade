// use actix_web::web;
// use serde::{Deserialize, Serialize};

// #[derive(Deserialize, Serialize, Debug)]
// pub struct Info {
//     username: String,
//     email: String,
//     password: String,
//     confirm_password: String,
// }

// pub async fn info(info: web::Json<Info>) -> web::Json<Info> {
//     println!("=========={:?}=========", info);
//     web::Json(Info {
//         username: info.username.clone(),
//         email: info.email.clone(),
//         password: info.password.clone(),
//         confirm_password: info.confirm_password.clone(),
//     })
// }