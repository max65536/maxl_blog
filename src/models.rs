use crate::schema::{users, posts};
use diesel::prelude::{Queryable,Insertable};
use rocket::serde::Serialize;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String, 
}

#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String, 
}

#[derive(Debug, Queryable)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub published: bool,
}

#[derive(Debug, Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub user_id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Bird {
    pub id: i32,
    pub name: String,
    pub scientific_name: String,
    pub commonwealth_status: String,
}