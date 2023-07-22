use std::env;
use diesel::Connection;
use diesel::prelude::MysqlConnection;
use dotenvy::dotenv;

pub fn establish_connection()-> MysqlConnection{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connectiong to {}", database_url))
}









