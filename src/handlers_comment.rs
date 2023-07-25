
use rocket_db_pools::{Database, Connection};
use crate::models::Comment;
use crate::database::BlogsDB;

pub async fn get_comments_of_blog(mut db: Connection<BlogsDB>, blog_id:&String) -> Option<Vec<Comment>> {
    let rows = sqlx::query_as!{Comment, "SELECT * FROM comments WHERE blog_id = ?", blog_id}
        .fetch_all(&mut *db)        
        .await;
    match rows {
        Ok(data) => Some(data),
        Err(_) => None
    }
}