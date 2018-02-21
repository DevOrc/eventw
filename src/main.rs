#[macro_use] extern crate nickel;

use nickel::{Nickel, StaticFilesHandler, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("assets/"));
    server.get("/api/get/eventName", middleware!("SumoBots Central OH Regional"));

    match server.listen("127.0.0.1:6767"){
        Ok(_) => println!(""),
        Err(e) => println!("Error: {}", e)
    }
}