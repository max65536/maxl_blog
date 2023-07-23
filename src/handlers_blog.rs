extern crate serde_json;
use std::collections::HashMap;
use rocket_db_pools::{Database, Connection};
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket_dyn_templates::{Template, context};
use rocket_dyn_templates::tera::Context;
use serde_derive::Serialize;

use crate::utils::{Page, get_page_index};
use crate::models::Blog;

#[derive(Database)]
#[database("awesome")]
struct BlogsDB(sqlx::MySqlPool);

#[get("/<id>")]
async fn read(mut db: Connection<BlogsDB>, id:String) -> Option<Json<Blog>> {
    let row = sqlx::query_as!{Blog, "SELECT * FROM blogs WHERE id = ?", id}
        .fetch_one(&mut *db)        
        .await;

    row.ok().map(|r| Json(r))

}
#[get("/details")]
async fn details(mut db: Connection<BlogsDB>) -> Option<Json<Vec<Blog>>> {
    let rows = sqlx::query_as!{Blog, "SELECT * FROM blogs"}
        .fetch_all(&mut *db).await;

    match rows {
        Ok(data) => Some(Json(data)),
        Err(_) => None
    }
}

#[derive(Serialize)]
enum ReturnValue {
    Integer(i32),
    Page(Page),
    Blogs(Vec<Blog>)
    // 可以继续添加其他需要的变体
}
#[get("/")]
async fn get_blogs(mut db: Connection<BlogsDB>) -> Template{
    let page = Some(String::from("1"));
    let (blogs, p) = api_list_blogs(db, page.clone())
        .await;
    // let mut context = Context::new();
    let mut context:HashMap<String, ReturnValue> = HashMap::new();
    context.insert("page_index".to_string(), ReturnValue::Integer(get_page_index(page.unwrap())));
    context.insert("blogs".to_string(), ReturnValue::Blogs(blogs.unwrap()));
    context.insert("page".to_string(), ReturnValue::Page(p));

    
    // context.insert("page_index", &get_page_index(page.unwrap()));
    // context.insert("blogs", &blogs);
    let template = Template::render("blogs", &context);
    template
}


#[get("/api/blogs")]
async fn api_blogs(mut db: Connection<BlogsDB>) -> Option<Json<Vec<Blog>>> {
    let rows = sqlx::query_as!{Blog, "SELECT * FROM blogs"}
        .fetch_all(&mut *db).await;

    match rows {
        Ok(data) => Some(Json(data)),
        Err(_) => None
    }
}

// #[get("/api/list/blogs?<page>")]
async fn api_list_blogs(mut db: Connection<BlogsDB>, page: Option<String>) -> (Option<Vec<Blog>>, Page) {
    // Implement the logic to fetch the list of blogs
    let page_index = get_page_index(page.unwrap());
    
    let total_rows: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM blogs")
    .fetch_one(&mut *db)
    .await
    .unwrap();

    let p = Page::new_size_10(total_rows as i32, page_index);
    let rows = sqlx::query_as!{Blog, 
        "SELECT * FROM blogs ORDER BY created_at DESC LIMIT ? OFFSET ?", &p.limit, &p.offset}
        .fetch_all(&mut *db).await;

    (match rows {
        Ok(data) => Some(data),
        Err(_) => None
    }, p)
}

#[get("/api/blogs/<id>")]
async fn api_get_blog(mut db: Connection<BlogsDB>, id:String) -> Option<Json<Blog>> {
    let row = sqlx::query_as!{Blog, "SELECT * FROM blogs WHERE id = ?", id}
        .fetch_one(&mut *db)        
        .await;

    row.ok().map(|r| Json(r))
}

// #[post("/api/blogs")]
// async fn api_create_blog(/* ... other params ... */) -> Status {
//     // Implement the logic to create a new blog
//     Status::Ok
// }

// #[post("/api/blogs/<id>")]
// async fn api_update_blog(id: i32, /* ... other params ... */) -> Option<Json<Blog>> {
//     // Implement the logic to update a specific blog by its ID
//     let blog = Blog { /* ... fields ... */ };
//     Some(Json(blog))
// }

// #[post("/api/blogs/<id>/delete")]
// async fn api_delete_blog(id: i32) -> Status {
//     // Implement the logic to delete a specific blog by its ID
//     Status::Ok
// }

#[derive(Serialize)]
struct User {
    name: String,
    age: u32,
}

#[get("/temp")]
fn temp() -> Template{
    let user = User {
        name: "Alice".to_string(),
        age: 30,
    };


    // or a struct
    let rendered = Template::render("test_temp", context! {user: user});
    rendered
}

#[get("/user_temp")]
fn user_temp() -> Template {
    let mut context = HashMap::new();
    context.insert("user_name", "Alice");
    context.insert("user_age", "30");
    
    Template::render("user", &context)
}

#[derive(Serialize)]
struct Item {
    name: String,
    description: String,
}
#[get("/items")]
fn items() -> Template {
    let items = vec![
        Item { 
            name: "Item 1".to_string(), 
            description: "Description for Item 1".to_string() 
        },
        Item { 
            name: "Item 2".to_string(), 
            description: "Description for Item 2".to_string() 
        }
    ];

    let mut context = std::collections::HashMap::new();
    context.insert("items", items);

    Template::render("items", &context)
}

#[get("/blog_temp")]
async fn blog_temp(mut db: Connection<BlogsDB>) -> Template{
    let page = Some(String::from("1"));
    let (blogs, p) = api_list_blogs(db, page.clone())
        .await;
    // let mut context = Context::new();
    // let mut context:HashMap<String, ReturnValue> = HashMap::new();
    // context.insert("page_index".to_string(), ReturnValue::Integer(get_page_index(page.unwrap())));
    // context.insert("blogs".to_string(), ReturnValue::Blogs(blogs.unwrap()));
    // context.insert("page".to_string(), ReturnValue::Page(p));

    
    // context.insert("page_index", &get_page_index(page.unwrap()));
    // context.insert("blogs", &blogs);

    let mut context = std::collections::HashMap::new();
    context.insert("blogs", blogs);

    let template = Template::render("blogs", &context);
    template
}

// #[get("/temp")]
// fn temp() -> Template {
//     let mut context = HashMap::new();
//     context.insert("my_message", "Hello, world!");

//     let template = Template::render("base", context);
//     template
//     // Template::render("base", context! { my_message: "value" })
// }

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket.attach(BlogsDB::init())
            .attach(Template::fairing())
            .mount("/", routes![read, details, 
                get_blogs, 
                api_get_blog,
                api_blogs,
                temp,
                user_temp,
                blog_temp,
                items])
    })
}
