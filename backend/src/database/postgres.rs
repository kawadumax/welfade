use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::gateway::models;
use crate::gateway::repository::Repository;


pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

struct PostgresManager {
    db_pool: DbPool
}

impl Repository for PostgresManager {
    // add code here

    fn find_user_by_id<i32, Result<Option<models::User>, diesel::result::Error>>(
        _id: i32,
        conn: &PgConnection,
    ) -> Result<Option<models::User>, diesel::result::Error> {
        use crate::schema::users::dsl::*;

        let user = users
            .filter(id.eq(_id))
            .first::<models::User>(conn)
            .optional()?;

        Ok(user)
    }

    /// Run query using Diesel to insert a new database row and return the result.
    fn insert_new_user<models::NewUser, Result<models::NewUser, diesel::result::Error>>(
        // prevent collision with `name` column imported inside the function
        _name: &str,
        _email: &str,
        _password: &str,
        conn: &PgConnection,
    ) -> Result<models::NewUser, diesel::result::Error> {
        // It is common when using Diesel with Actix web to import schema-related
        // modules inside a function's scope (rather than the normal module's scope)
        // to prevent import collisions and namespace pollution.
        use crate::schema::users::dsl::*;

        let new_user = models::NewUser {
            name: _name.to_owned(),
            email: _email.to_owned(),
            password: _password.to_owned(),
        };

        diesel::insert_into(users).values(&new_user).execute(conn)?;

        Ok(new_user)
    }
}