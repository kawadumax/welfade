use serde::{ Deserialize, Serialize };
use crate::schema::users;
use crate::entity::user::UserEntity;
use crate::entity::user::UserIdEntity;

#[derive(Deserialize, Serialize, Queryable)]
pub struct UserModel {
    pub id: UserIdEntity,
    pub name: String,
    pub email: String,
    pub password: String    
}



#[table_name="users"]
#[derive(Deserialize, Serialize, Insertable)]
pub struct NewUserModel {
    pub name: String,
    pub email: String,
    pub password: String
}