#[macro_use] extern crate rocket;
extern crate rocket_db_pools;

use rocket::{Rocket, Build};
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};


#[derive(Database)]
#[database("maxl_blog")]
struct Blogs(sqlx::MySqlPool);

#[derive(serde::Serialize)]
pub struct Bird {
    pub id: i32,
    pub name: String,
    pub scientific_name: String,
    pub commonwealth_status: String,
}


#[get("/<id>")]
async fn read(mut db: Connection<Blogs>, id:i32) -> Option<String> {
    sqlx::query("SELECT name FROM bird WHERE id=?").bind(id)
        .fetch_one(&mut *db).await
        .and_then(|r| Ok(r.try_get(0)?))
        .ok()
}

#[get("/all_birds")]
async fn list(mut db: Connection<Blogs>) -> Option<Json<Vec<String>>> {
    let rows = sqlx::query("SELECT name FROM bird")
    .fetch_all(&mut *db).await;

    match rows {
        Ok(data) => {
            let names: Vec<String> = data.into_iter().map(|r| r.try_get(0).unwrap_or_default()).collect();
            Some(Json(names))
        },
        Err(_) => None
    }
}

#[get("/details")]
async fn details(mut db: Connection<Blogs>) -> Option<Json<Vec<Bird>>> {
    let rows = sqlx::query_as!{Bird, "SELECT * FROM bird"}
        .fetch_all(&mut *db).await;

    match rows {
        Ok(data) => Some(Json(data)),
        Err(_) => None
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv::dotenv().ok();
    rocket::build().attach(Blogs::init()).mount("/", routes![read, list, details])
}