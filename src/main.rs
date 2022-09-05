mod level;
mod player;
mod render;

use level::load_level;
use player::Player;
use render::render;
use std::io;

fn main() {
    let mut message = String::from("");
    let mut player = Player{
        position: (0, 0),
        health: 100,
        color: String::from("green"),
        icon: String::from("@")};
    let level_tuple = load_level("src/level.txt");
    let level = level_tuple.0;
    let start_position = level_tuple.2;
    let end_position = level_tuple.1;
    player.position = start_position;
    while player.health != 0 && player.position != end_position{
        render(&mut player, &level, &message);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        message = player.move_player(&level, input);
    }
    println!("You win !!");
}