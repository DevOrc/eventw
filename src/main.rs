#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket::request::State;
use std::sync::{Mutex};

struct Event{
    teams: Vec<Team>
}

impl Event{

    ///Creates a new event
    pub fn new() -> Event{
        let vec: Vec<Team> = Vec::new();

        Event {teams: vec}
    }

    ///Adds a team to the event
    pub fn add_team(&mut self, team: Team){
        self.teams.push(team);

        println!("Teams: {:?}", self.teams);
    }

}

#[derive(Debug, Clone)]
struct Team{
    name: String, 
    number: u32
}

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
fn create_team(name: String, number: u32, event: State<Mutex<Event>>){
    let team_created = Team {name: name, number: number};
    println!("Created Team: {:?}", team_created);

    let mut event_lock = event.lock().unwrap();

    event_lock.add_team(team_created);
}

fn main() {
    let event: Mutex<Event> = Mutex::new(Event::new());

    rocket::ignite()
        .mount("/", routes![index, files, event_name, create_team])
        .manage(event)
        .launch();
}
