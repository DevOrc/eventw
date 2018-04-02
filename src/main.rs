extern crate clap;
extern crate eventw;

use clap::{Arg, App};
use eventw::Config;

fn main() {
    let matches = App::new("My Super Program")
        .version(eventw::VERSION)
        .author("Noah Charlton <ncharlton002@gmail.com>")
        .about("Sumobots event manager")
        .arg(Arg::with_name("name")
            .short("n")
            .long("event-name")
            .value_name("EVENT_NAME")
            .help("Sets the name of the event")
            .takes_value(true))
        .get_matches();

    let event_name = matches.value_of("name").unwrap_or("sumo_regional");
    let path = eventw::util::get_file(event_name.to_string());

    println!("Event Name: {}", event_name.to_string());
    println!("Save File: {:?}", path);

    let config = Config::new(event_name.to_string(), path);

    //Now that we have setup everything, run the server
    eventw::run(config);
}