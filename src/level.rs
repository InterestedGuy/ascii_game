use std::path::Path;
use std::fs::read_to_string;

pub fn load_level(path: &str) -> Vec<Vec<String>>{
    let mut level:Vec<Vec<String>> = Vec::new();
    let path = Path::new(path);

    let level_input = match read_to_string(&path) {
        Err(why) => panic!("{} {:?}", why, path),
        Ok(level) => level
    };

    // map function takes every item in the iterator created by split and turns it into a String.
    // collect function collects these strings into a vector so the vector is Vec<String> not Vec<&str> so it can be returned.
    let level_input:Vec<&str> = level_input.split("\n").collect();
    for floor in level_input{
        let floor:Vec<String> = floor.split("").map(|s| s.to_string()).collect();
        level.push(floor);
    }
    level
}