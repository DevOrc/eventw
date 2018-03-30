use std::io::Write;
use super::Event;
use super::util;
use std::fs::File;

pub fn save_event(event: &Event){
    let path = util::get_file("sumo_regional".to_string());

    let mut file = File::create(&path).unwrap_or_else(|e|{
        panic!("I/O Error! Could't load save file: {}", e);
    });

    for team in &event.teams{
        file.write_fmt(format_args!("{},{}\n", team.name, team.number)).unwrap_or_else(|e|{
            panic!("Error writing file: {}", e);
        });
    }
}
