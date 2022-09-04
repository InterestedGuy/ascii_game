use crate::player::Player;

pub fn render(player: &Player, level:&Vec<Vec<String>>, message:&String){
    print!("\x1B[2J\x1B[1;1H");
    for y in 0..level.len(){
        let floor = &level[y];
        for x in 0..floor.len(){
            let block = &floor[x];
            if x == player.position.0 && y == player.position.1{
                player.render_player();
            }else{
                print!("{block}");
            }
        }
        println!();
    }
    println!();
    println!("[HP: {}, X: {} Y: {}]", player.health, player.position.0 - 1, player.position.1 - 1);
    println!("{message}");
}