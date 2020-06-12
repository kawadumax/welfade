use serde::{ Deserialize, Serialize };
use crate::schema::users;

#[derive(Deserialize, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[table_name="users"]
#[derive(Deserialize, Serialize, Insertable)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String
}