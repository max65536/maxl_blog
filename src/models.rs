use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Bird {
    pub id: i32,
    pub name: String,
    pub scientific_name: String,
    pub commonwealth_status: String,
}

#[derive(Serialize, Deserialize)]
pub struct Blog {
    pub id : String,
    pub user_id: String,
    pub user_name : String,
    pub user_image: String,
    pub name : String,
    pub summary: String,
    pub content : String,
    pub created_at: f64,
    pub image: String,
    pub show: i8
}

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub id : String,
    pub blog_id : String,
    pub user_id : String,
    pub user_name : String,
    pub user_image : String,
    pub content : String,
    pub created_at: f64
}