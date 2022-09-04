use std::path::Path;
use std::fs::read_to_string;

use crate::player::Player;

pub fn load_level(path: &str, player:&mut Player) -> Vec<Vec<String>>{
    let mut level:Vec<Vec<String>> = Vec::new();
    let path = Path::new(path);

    let level_input = match read_to_string(&path) {
        Err(why) => panic!("{} {:?}", why, path),
        Ok(level) => level
    };

    // map function takes every item in the iterator created by split and turns it into a String.
    // collect function collects these strings into a vector so the vector is Vec<String> not Vec<&str> so it can be returned.
    let level_input:Vec<&str> = level_input.split("\n").collect();
    for y in 0..level_input.len(){
        let floor = level_input[y];
        let floor:Vec<String> = floor.split("").map(|s| s.to_string()).collect();
        for x in 0..floor.len(){
            if floor[x] == "0" {
                player.position.0 = x;
                player.position.1 = y;
            }
        }
        level.push(floor);
    }
    level
}