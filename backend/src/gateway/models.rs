use serde::{ Deserialize, Serialize };
use crate::schema::users;
use crate::entity::user::UserIdEntity;

pub type UserIdModel = UserIdEntity;

#[derive(Deserialize, Serialize, Queryable, Debug)]
pub struct UserModel {
    pub id: UserIdModel,
    pub name: String,
    pub email: String,
    pub password: String,
    pub sesstion_token: Option<String>
}

#[table_name="users"]
#[derive(Deserialize, Serialize, Insertable, Debug)]
pub struct NewUserModel {
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize)]
pub struct Pong {
    pub body: String
}
