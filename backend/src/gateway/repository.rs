use crate::gateway::models::NewUserModel;
use crate::gateway::models::UserModel;
// actixをここでuseしたくない
use actix_web::Result;

pub trait Repository {
    // add code here
    fn find_user_by_id(user_id: UserModel) -> Option<UserModel>;

    /// Run query using Diesel to insert a new database row and return the result.
    fn insert_new_user(new_user: NewUserModel) -> Result<UserModel>;
}