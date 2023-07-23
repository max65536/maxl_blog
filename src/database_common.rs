use rocket_db_pools::Database;

#[derive(Database)]
#[database("maxl_blog")]
pub struct BlogsDB(sqlx::MySqlPool);