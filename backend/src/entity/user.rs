pub type UserIdEntity = i32;

pub struct UserEntity {
    pub id: UserIdEntity,
    pub name: String,
    pub email: String,
    pub password: String,
}
