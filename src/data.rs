use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;
use super::Event;
use super::util;
use std::fs::File;

pub fn save_event(event: &Event){
    let path = &event.config.path;

    let mut file = File::create(&path).unwrap_or_else(|e|{
        panic!("I/O Error! Could't load save file: {}", e);
    });

    for team in &event.teams{
        file.write_fmt(format_args!("{},{}\n", team.name, team.number)).unwrap_or_else(|e|{
            panic!("Error writing file: {}", e);
        });
    }
}

pub fn load_event(event: &mut Event){
    load_teams(event);
}

pub fn load_teams(event: &mut Event){
    let file = File::open(&event.config.path).unwrap_or_else(|e|{
        println!("Are you missing your save file?");
        panic!("I/O Error! Could't load save file: {}", e);
    });

    let reader = BufReader::new(&file);
    
    for line in reader.lines() {
        let line = line.unwrap_or_else(|e|{
            panic!("Error: {}", e);
        });

        let split = line.split(",");
        let vec: Vec<&str> = split.collect();

        let name = vec[0].to_string();
        let number: u32 = vec[1].parse().unwrap_or_else(|_|{
            panic!("Syntax error in save file! Team: {}", name);
        });

        event.add_team(super::Team{name: name, number: number});
    }   

}