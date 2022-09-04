mod level;
mod player;
mod render;

use level::load_level;
use player::Player;
use render::render;
use std::io;

fn main() {
    let mut message = String::from("");
    let level = load_level("src/level.txt");
    let mut player = Player{
        position: (20, 20),
        health: 100,
        color: String::from("green"),
        icon: String::from("@")};
    while player.health != 0 {
        render(&player, &level, &message);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        message = player.move_player(&level, input);
    }
}