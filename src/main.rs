#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("assets/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}

#[get("/api/get/eventName")]
fn event_name() -> String {
     format!("SumoBots Central Regional!")
}

#[post("/api/post/createTeam/<name>/<number>")]
fn create_team(name: String, number: u32) -> String{
    println!("Team Created! {}: {}", name, number);
    format!("{}: {}", name, number)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, files, event_name, create_team])
}

fn main() {
    rocket().launch(); 
}
