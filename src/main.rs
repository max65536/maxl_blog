#[macro_use] extern crate rocket;

use rocket::data::ToByteUnit;
use rocket::tokio::fs::File;
use rocket::Data;
use rocket::http::uri::Absolute;

mod paste_id;
use paste_id::PasteId;

// #[get("/<id>")]
// async fn retrieve(id: &str) -> Option<File> {
//     let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
//     let filename = Path::new(upload_dir).join(id);
//     File::open(&filename).await.ok()
// }

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<File> {
    File::open(id.file_path()).await.ok()
}


const ID_LENGTH:usize = 3;
const HOST: Absolute<'static> = uri!("http://localhist:8000");

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> std::io::Result<String>{
    let id = PasteId::new(ID_LENGTH);
    paste.open(128.kibibytes()).into_file(id.file_path()).await?;
    Ok(uri!(HOST, retrieve(id)).to_string())
}

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
    rocket::build().mount("/", routes![index, retrieve, upload])
}

use rocket_dyn_templates::Template;

#[get("/temp")]
fn temp() -> Template {
    let context = /* object-like value */;
    Template::render("index", &context)
}