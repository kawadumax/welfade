use crate::gateway::models;

pub trait Repository {
    // add code here
    type FindUserResponse;
    fn find_user_by_id(user_id: models::UserIdModel) -> Self::FindUserResponse;

    type InsertUserResponse;
    fn insert_new_user(new_user: models::NewUserModel) -> Self::InsertUserResponse;
}