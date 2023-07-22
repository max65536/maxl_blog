



// use schema::bird::dsl::*;

// mod database;
// mod models;
// mod schema;

// use rocket_sync_db_pools::diesel;
// use self::diesel::prelude::*;

// use models::Bird;

// use rocket::Rocket;
// use rocket::Build;



// // use rocket_contrib::json::Json;



// #[get("/")]
// fn index() -> &'static str {
//     "
//     USAGE

//       POST /

//           accepts raw data in the body of the request and responds with a URL of
//           a page containing the body's content

//       GET /<id>

//           retrieves the content for the paste with id `<id>`
//     "
// }

// #[get("/birds")]
// fn get_birds() -> Json<Vec<Bird>>{
//     let mut connection = database::establish_connection();
//     let result = bird
//         .limit(5)
//         .load::<Bird>(&mut connection)
//         .map(Json)
//         .expect("Error loading birds");
//     result
// }

// #[get("/")]
// async fn list(db: Db) -> Result<Json<Vec<i64>>> {
//     let ids = db.run(|conn| {
//         conn.prepare("SELECT id FROM posts")?
//             .query_map(params![], |row| row.get(0))?
//             .collect::<Result<Vec<i64>, _>>()
//     }).await?;

//     Ok(Json(ids))
// }


// #[launch]
// fn rocket() -> Rocket<Build> {
//     rocket::build().mount("/", routes![index, get_birds])
// }

// #[launch]
// fn rocket() -> Rocket<Build> {
//     rocket::build().mount("/", routes![index,  get_birds])
//         // .attach(Template::fairing())
// }

// use std::collections::HashMap;
// use rocket_dyn_templates::Template;
// use tera::Context;

// #[get("/temp")]
// fn temp() -> Template {
//     let mut context = HashMap::new();
//     context.insert("my_message", "Hello, world!");

//     let template = Template::render("base", context);
//     template
//     // Template::render("base", context! { my_message: "value" })
// }

// fn main() {
//     let mut connection = database::establish_connection();
//     let result = bird
//         .limit(5)
//         .load::<Bird>(&mut connection)
//         .expect("Error loading posts");
//     println!("{:?}", result);
// }