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
    let level_tuple = load_level("src/level.txt", &mut player);
    let level = level_tuple.0;
    let end_position = level_tuple.1;
    while player.health != 0 && player.position != end_position{
        /* println!("{:?}", (end_position));
        break; */
        render(&mut player, &level, &message);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        message = player.move_player(&level, input);
    }
    println!("You win !!");
}