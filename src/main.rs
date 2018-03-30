#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket::request::State;
use std::cmp::Ordering;
use std::sync::{Mutex};

mod util;

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
        if team.name.contains(","){
            return "Commas are not allowed in team names!".to_string();
        }

        //Check if team number is valid
        for t in &self.teams{
            if t.number == team.number{
                println!("Two Teams with duplicate number! Team 1: {:?}, Team 2: {:?}", t, team);
                return format!("Error: Team {} already has number {}", t.name, t.number);
            }
        }

        self.teams.push(team);
        self.sort_teams();

        return "Team added!".to_string();
    }

    pub fn remove_team(&mut self, number: u32) -> String{
        let mut index = 0;

        for t in &self.teams{
            if t.number == number{
                break;
            }

            index+=1;
        }

        if index < self.teams.len(){
            self.teams.remove(index);
            return "Team Deleted".to_string();
        }
        return "Team Not Found!".to_string();
    }

    fn sort_teams(&mut self){
        self.teams.sort_by(|a, b| a.cmp(b));
    }

}

#[derive(Debug, Clone)]
struct Team{
    name: String, 
    number: u32
}

impl Team{
     fn cmp(&self, b: &Team) -> Ordering{
         if b.number > self.number{
             return Ordering::Less;
         }else if b.number < self.number{
             return Ordering::Greater;
         }
         return Ordering::Equal;
    }
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("assets/index.html")
}

#[get("/<file..>", rank = 2)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}

#[get("/eventName")]
fn event_name() -> String {
     format!("SumoBots Central Regional!")
}

#[get("/teamName/<number>")]
fn get_name_from_num(number: u32, event_mutex: State<Mutex<Event>>) -> String{
    let event = event_mutex.lock().unwrap();

    for t in &event.teams{
        if t.number == number{
            return t.name.clone();
        }
    }
    "404".to_string()
}

#[get("/teams")]
fn get_teams(event_mutex: State<Mutex<Event>>) -> String{
    let event = event_mutex.lock().unwrap();
    let mut response = String::new();

    for team in &event.teams {
        response = format!("{}{},{}\n", response, team.name, team.number);
    }
    response
}

#[delete("/team/<number>")]
fn delete_team( number: u32, event_mutex: State<Mutex<Event>>) -> String{
    let mut event = event_mutex.lock().unwrap();

    event.remove_team(number)
}

#[post("/createTeam/<name>/<number>")]
fn create_team(name: String, number: u32, event_mutex: State<Mutex<Event>>) -> String{
    let team_created = Team {name: name, number: number};
    println!("Created Team: {:?}", team_created);

    let mut event = event_mutex.lock().unwrap();

    event.add_team(team_created)
}

#[post("/shutdown")]
fn shutdown(){
    ::std::process::exit(0);
}

fn main() {
    println!("Save Directory: {:?}", util::get_file("sumo_regional".to_string()));

    let event: Mutex<Event> = Mutex::new(Event::new());

    for i in 1..40{
        let mut e = event.lock().unwrap();

        let num = i * 5;
        let name = format!("TestTeam {}", num);
        e.add_team(Team{number: num, name: name});
    }

    rocket::ignite()
        .mount("/api/get/", routes![get_teams, event_name, get_name_from_num])
        .mount("/api/post/", routes![create_team, shutdown])
        .mount("/api/delete/", routes![delete_team])
        .mount("/", routes![files, index])
        .manage(event)
        .launch();
}
