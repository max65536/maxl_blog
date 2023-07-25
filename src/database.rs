use rocket_db_pools::Database;

#[derive(Database)]
#[database("awesome")]
pub struct BlogsDB(sqlx::MySqlPool);