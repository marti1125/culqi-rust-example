#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
extern crate serde_json;
extern crate culqi;

#[macro_use] extern crate serde_derive;

use std::path::{Path, PathBuf};
use rocket_contrib::Template;
use rocket_contrib::{JSON};
use rocket::response::NamedFile;

#[derive(Debug, FromForm)]
struct NewCharge {
    token: String,
    installments: i32
}

#[get("/<file..>", rank = 5)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[error(404)]
fn not_found() -> Template {
    let context = ();
    Template::render("404", &context)
}

#[post("/charge?<charge>")]
fn charge(charge: NewCharge) -> JSON<String> {
    let secret_key = "sk_test_UTCQSGcXW8bCyU59";
    let client = culqi::Client::new(&secret_key);
    let new_charge = culqi::Charge::new("3500", "PEN", "will@me.com", &charge.token);
    let get_charge = culqi::Charge::create(&client, &new_charge);
    JSON(get_charge)
}

#[get("/")]
fn index() -> Template {
    let context = ();
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![files, index, charge])
    .catch(errors![not_found])
    .launch();
}
