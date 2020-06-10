use crate::entity::user;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait UserUseCase {
    async fn find(&self, id: i32) -> Result<User, Error>;
    async fn create(&self, new_user: NewUser) -> Result<User, Error>;
}
