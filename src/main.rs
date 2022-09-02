mod level;
mod player;
mod render;

use level::load_level;
use player::Player;
use render::render;
use std::io;

fn main() {
    let level = load_level("src/level.txt");
    let mut player = Player{
        position: (20, 20),
        health: 100,
        color: String::from("green"),
        icon: String::from("@")};
    
    while player.health != 0 {
        render(&player, &level);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Err(_) => break,
            Ok(_) => {
                let output = player.move_player(input);
                if output.2 != "ok" {
                    println!("{}", output.2);
                    continue;
                }else{
                    let direction = output.0.to_lowercase();
                    let steps:usize = output.1;
                    if direction == String::from("w") && player.can_move(&level, &direction, steps){
                        player.position.1 -= steps;
                    }else if direction == String::from("a") && player.can_move(&level, &direction, steps){
                        player.position.0 -= steps;
                    }else if direction == String::from("s") && player.can_move(&level, &direction, steps){
                        player.position.1 += steps;
                    }else if direction == String::from("d") && player.can_move(&level, &direction, steps){
                        player.position.0 += steps;
                    }
                }
            }
        };
        println!("Game over !!!");
    }
}