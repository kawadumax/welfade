use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use crate::gateway::models;
use crate::gateway::repository::Repository;
use lazy_static::lazy_static;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct PostgresManager {
    pub db_pool: DbPool
}

impl PostgresManager {
    fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.db_pool.get().expect("poolからDBへの接続を作れませんでした。")
    }
}

// DBへの接続情報を実行時にstatic変数に格納する
lazy_static! {
    pub static ref DB_MANAGER: PostgresManager = PostgresManager {
        db_pool: r2d2::Pool::builder().build(
            ConnectionManager::<PgConnection>::new(
                std::env::var("DATABASE_URL").expect(".envファイルからDATABASE_URLを取得できませんでした。")
            )
        ).expect("db_poolを作成できませんでした。")
    };
}

impl Repository for PostgresManager {

    type FindUserResponse = Result<Option<models::UserModel>, diesel::result::Error>;
    fn find_user_by_id(_id: i32) -> Self::FindUserResponse {
        let conn = DB_MANAGER.get_connection();
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
        let conn = DB_MANAGER.get_connection();
        use crate::schema::users::dsl::*;
        let result = diesel::insert_into(users).values(_new_user).get_result(&conn).optional()?;
        Ok(result)
    }
}