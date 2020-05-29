use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Deserialize, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
}