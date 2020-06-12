use crate::entity::user::User;

// TODO: usecaseでactixをuseすべきでない
use actix_web::Result;

pub trait FindUserUseCase {
    fn find(&self, id: i32) -> Result<User>;
}

pub trait CreateUserUseCase {
    fn create(&self, new_user: NewUser) -> Result<User>;
}

pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}