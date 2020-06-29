use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::gateway::models;
use crate::gateway::repository::Repository;
use lazy_static::lazy_static;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct PostgresManager {
    pub db_pool: DbPool
}

lazy_static! {
    pub static ref DB_MANAGER: PostgresManager = PostgresManager {
        db_pool: r2d2::Pool::builder().build(
            ConnectionManager::<PgConnection>::new(
                std::env::var("DATABASE_URL").expect("DATABASE_URL")
            )
        ).expect("Failed to create pool.")
    };
}

impl Repository for PostgresManager {
    // add code here

    type FindUserResponse = Result<Option<models::UserModel>, diesel::result::Error>;
    fn find_user_by_id(_id: i32) -> Self::FindUserResponse {
        let conn = DB_MANAGER.db_pool.get().expect("couldn't get db connection from pool");
        use crate::schema::users::dsl::*;
        let user = users.filter(id.eq(_id))
            .first::<models::UserModel>(&conn)
            .optional()?;
        Ok(user)
    }

    type InsertUserResponse = Result<Option<models::UserModel>, diesel::result::Error>;
    /// Run query using Diesel to insert a new database row and return the result.
    fn insert_new_user(_new_user: models::NewUserModel) -> Self::InsertUserResponse {
        // It is common when using Diesel with Actix web to import schema-related
        // modules inside a function's scope (rather than the normal module's scope)
        // to prevent import collisions and namespace pollution.
        let conn = DB_MANAGER.db_pool.get().expect("couldn't get db connection from pool");
        use crate::schema::users::dsl::*;
        let result = diesel::insert_into(users).values(_new_user).get_result(&conn).optional()?;
        Ok(result)
    }
}