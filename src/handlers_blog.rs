extern crate serde_json;
extern crate comrak;

use std::collections::HashMap;
use rocket_db_pools::{Database, Connection};
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket_dyn_templates::{Template, context};
use serde_derive::Serialize;
use rand::seq::SliceRandom;
use comrak::markdown_to_html;

use crate::utils::{Page, get_page_index, list_files_in_directory};
use crate::models::{Blog, Comment};
use crate::database::BlogsDB;

use crate::handlers_comment::get_comments_of_blog;

//----- get data from database -----//
async fn get_one_blog(mut db: Connection<BlogsDB>, id:&String) -> (Option<Blog>, Connection<BlogsDB>) {
    let row = sqlx::query_as!{Blog, "SELECT * FROM blogs WHERE id = ?", id}
        .fetch_one(&mut *db)        
        .await;
    (row.ok(), db)
}

//----- return a template -----//
#[get("/?<page>")]
async fn get_blogs(db: Connection<BlogsDB>, page:Option<String>) -> Template{
    // let page = Some(String::from(pagenum));
    let (blogs, p) = api_list_blogs(db, page.clone())
        .await;

    #[derive(Serialize)]
    struct TemplateContext {
        blogs: Vec<Blog>,
        page: Page,
    }
    let context = TemplateContext { blogs:blogs.unwrap_or(Vec::new()), page:p };
    let template = Template::render("blogs", &context);
    template
}

#[get("/blog/<id>")]
async fn get_blog_details(db: Connection<BlogsDB>, id:String) -> Template {
    // let db_blog = Rc::new(db);
    // let db_comment = Rc::clone(&db_blog);
    let (blog, db_return) = get_one_blog(db, &id).await;
    match blog {
        Some(b) => {

            let options = comrak::ComrakOptions::default();
            // options = true;  // 启用HTML自动转义
            let html_str = markdown_to_html(&b.content, &options);
            let blog_with_markdown = Blog{content:html_str, ..b};
            
            let comments = get_comments_of_blog(db_return, &blog_with_markdown.id).await;
            #[derive(Serialize)]
            struct TemplateContext {
                blog: Blog,
                comments: Vec<Comment>,
                image: String
            }
            let imagelist = list_files_in_directory("blog_data/user");
            let image = imagelist.choose(&mut rand::thread_rng());
            let context = TemplateContext{blog:blog_with_markdown, 
                                comments:comments.unwrap_or(Vec::new()),
                                image: format!("/image/user/{}", image.unwrap_or(&String::from("")))
                            };
            let template = Template::render("blog_details", &context);
            template
        },
        None => {
            return Template::render("404", context! {})        
        }
    }

}

//----- apis -----//
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
    let page_index = get_page_index(page.unwrap_or("1".to_string()));
    
    let total_rows: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM blogs")
    .fetch_one(&mut *db)
    .await
    .unwrap();

    let p: Page = Page::new_size_10(total_rows as i32, page_index);

    println!("{:?}", p);
    // println!("page {:?}", &page);
    println!("page_index {page_index}");
    

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

//----------- TESTs ----------//

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
async fn blog_temp(db: Connection<BlogsDB>) -> Template{
    let page = Some(String::from("1"));
    let (blogs, p) = api_list_blogs(db, page.clone())
        .await;

    #[derive(Serialize)]
    struct TemplateContext {
        blogs: Vec<Blog>,
        page: Page,
    }

    let context = TemplateContext { blogs:blogs.unwrap(), page:p };
    // let mut context = std::collections::HashMap::new();
    // context.insert("blogs", blogs);
    // context.insert("page", p);

    let template = Template::render("blogs", &context);
    template
}

#[get("/base")]
fn base() -> Template {
    Template::render("base", context! {})
}
#[get("/helo")]
fn helo() -> Template {
    Template::render("helo", context! {})
}

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


pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket.attach(BlogsDB::init())            
            .mount("/", routes![read, details, 
                get_blogs, 
                api_get_blog,
                api_blogs,
                temp,
                user_temp,
                blog_temp,
                items,
                base,
                helo,
                get_blog_details])
    })
}
