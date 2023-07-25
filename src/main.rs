#[macro_use] extern crate rocket;
extern crate rocket_db_pools;

use rocket::{Rocket, Build};
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::tera::{Value, to_value, Result, try_get_value, Filter, Tera};
use rocket_dyn_templates::{Template, context};
use std::collections::HashMap;
use chrono::{DateTime, Utc, NaiveDateTime};

mod database;
mod handlers_blog;
mod handlers_comment;
mod handlers_test;
mod models;
mod utils;
// mod handlers_bird;



pub fn datetime_filter(value: &Value, args: &HashMap<String, Value>) -> Result<Value> {
        // 尝试从值中提取timestamp
    let timestamp = value.as_f64().unwrap();

    // 分离秒和纳秒
    let seconds = timestamp.trunc() as i64;
    let nanoseconds = (timestamp.fract() * 1_000_000_000.0) as u32;

    // 将UNIX timestamp转换为DateTime<Utc>
    let naive = NaiveDateTime::from_timestamp_opt(seconds, nanoseconds);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive.unwrap(), Utc);

    // 获取格式化参数，如果没有提供则使用默认格式
    let format = args.get("format")
        .and_then(Value::as_str)
        .unwrap_or("%Y-%m-%d %H:%M");

    Ok(Value::String(datetime.format(format).to_string()))

}

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv::dotenv().ok();
    // rocket::build().attach(Blogs::init()).mount("/", routes![read, list, details])
    rocket::build()
        .attach(Template::custom(|engines| {
            engines.tera.register_filter("datetime", datetime_filter);
        }))
        .attach(handlers_blog::stage())
        .attach(handlers_test::stage())
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/image", FileServer::from(relative!("blog_data")))
}