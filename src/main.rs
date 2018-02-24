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
    pub fn add_team(&mut self, team: Team) -> String{
        
        //Check if team number is valid
        for t in &self.teams{
            if t.number == team.number{
                println!("Two Teams with duplicate number! Team 1: {:?}, Team 2: {:?}", t, team);
                return format!("Error: Team {} already has number {}", t.name, t.number);
            }
        }

        self.teams.push(team);

        println!("Added Team: {:?}", self.teams);
        return "Team added!".to_string();
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

#[get("/eventName")]
fn event_name() -> String {
     format!("SumoBots Central Regional!")
}

#[get("/teams")]
fn get_teams(event_mutex: State<Mutex<Event>>) -> String{
    let event = event_mutex.lock().unwrap();
    let mut response = String::new();

    for team in &event.teams {
        response = format!("{}{} {}\n", response, team.name, team.number);
    }
    response
}

#[post("/createTeam/<name>/<number>")]
fn create_team(name: String, number: u32, event_mutex: State<Mutex<Event>>) -> String{
    let team_created = Team {name: name, number: number};
    println!("Created Team: {:?}", team_created);

    let mut event = event_mutex.lock().unwrap();

    event.add_team(team_created)
}

fn main() {
    let event: Mutex<Event> = Mutex::new(Event::new());

    rocket::ignite()
        .mount("/api/get/", routes![get_teams, event_name])
        .mount("/api/post/", routes![create_team])
        .mount("/", routes![files, index])
        .manage(event)
        .launch();
}
