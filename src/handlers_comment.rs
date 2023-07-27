use rocket::Error;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::request::FromParam;
use rocket_db_pools::{Connection};
use serde_derive::{Serialize, Deserialize};
use crate::models::Comment;
use crate::database::BlogsDB;
use crate::utils::{next_id, get_timestamp};
use rocket::fairing::AdHoc;
#[derive(Serialize, Deserialize)]
pub struct RequstData {
    pub name: String,
    pub content: String,
    pub image: String
}

pub async fn get_comments_of_blog(mut db: Connection<BlogsDB>, blog_id:&String) -> Option<Vec<Comment>> {
    let rows = sqlx::query_as!{Comment, "SELECT * FROM comments WHERE blog_id = ?", blog_id}
        .fetch_all(&mut *db)        
        .await;
    match rows {
        Ok(data) => Some(data),
        Err(_) => None
    }
}

#[post("/api/blogs/<blog_id>/comments", format="application/json", data="<data>")]
pub async fn api_create_comment(mut db: Connection<BlogsDB>, blog_id: String, data:Json<RequstData>) 
    -> Json<String>{
    let comment = Comment{
                                id:next_id(), 
                                blog_id:blog_id, 
                                user_id:"00".to_string(), 
                                user_name :data.name.clone(),
                                user_image:data.image.clone().replace("&#x2F;", "/"),
                                content:data.content.clone(),
                                created_at: get_timestamp()
                            };
    let result = sqlx::query_as!{Comment, "INSERT INTO comments 
                                              (id, user_id, user_name, user_image, blog_id, content, created_at) 
                                              VALUES (?, ?, ?, ?, ?, ?, ?)", 
                                              &comment.id ,&comment.user_id ,&comment.user_name,
                                              &comment.user_image ,&comment.blog_id ,&comment.content,
                                              &comment.created_at}
        .execute(&mut *db)
        .await; 
    match result {
        Ok(value) => {
            println!("{:?}", value);
            Json("success".to_string())
        }
        Err(error) => {
            println!("{:?}", error);
            Json("failed".to_string())
        }
    }
}

// #[post("/<id>")]
// fn new(id: usize) -> status::Accepted<String> {
//     status::Accepted(format!("id: '{}'", id))
// }

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("comment Stage", |rocket| async {
        rocket.mount("/", routes![api_create_comment])
    })
}