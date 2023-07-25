extern crate serde_json;
use rocket_db_pools::{Database, Connection};
use rocket::fairing::AdHoc;
use rocket_dyn_templates::{Template, context};
use rocket::response::content;
use std::fs;

use crate::models::Blog;

#[get("/parent")]
fn parent() -> Template {
    Template::render("parent", context! {})
}

#[get("/child")]
fn child() -> Template {
    Template::render("child", context! {})
}

#[get("/list_files/<path>")]
fn list_files(path:&str) -> String {
    let path_replace = path.replace("-", "/");

    let entries = fs::read_dir(path_replace)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut file_names = Vec::new();

    for entry in entries {
        if let Some(name) = entry.file_name().and_then(|n| n.to_str()) {
            file_names.push(name.to_string());
        }
    }

    let response = file_names.join("\n");
    println!("{response}");
    response
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("test stage", |rocket| async {
        rocket         
            .mount("/test", routes![
                parent,
                child,
                list_files
            ])
    })
}
