#[macro_use] extern crate rocket;
extern crate rocket_db_pools;

use rocket::{Rocket, Build};

mod database_common;
mod handlers_blog;
mod models;
mod utils;
// mod handlers_bird;





#[launch]
fn rocket() -> Rocket<Build> {
    dotenv::dotenv().ok();
    // rocket::build().attach(Blogs::init()).mount("/", routes![read, list, details])
    rocket::build()
        .attach(handlers_blog::stage())
}