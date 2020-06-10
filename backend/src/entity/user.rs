pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}