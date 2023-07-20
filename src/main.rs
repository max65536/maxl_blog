#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, temp])
        .attach(Template::fairing())
}

use rocket_dyn_templates::{Template, context};
// use tera::Context;

#[get("/temp")]
fn temp() -> Template {
    Template::render("base", context! { my_message: "value" })
}