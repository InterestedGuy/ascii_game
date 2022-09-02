use colored::{Colorize};

pub struct Player {
    pub position: (usize, usize),
    pub health: i32,
    pub color: String,
    pub icon: String
}

impl Player{
    pub fn render_player(&self){
        let color = self.color.clone();
        print!("{}", self.icon.color(color));
    }

    pub fn can_move(&self, level: &Vec<Vec<String>> , direction: &str, steps:usize) -> bool{
        let x = self.position.0;
        let y = self.position.1;
        if direction == "w"{
            match y.checked_sub(steps) {
                Some(_) => {
                    if level[y - steps][x] != " "{
                        return false;
                    }else{
                        return true;
                    }
                },
                None => return false}
        }else if direction == "a"{
            match x.checked_sub(steps) {
                Some(_) => {
                    if level[y][x - steps] != " "{
                        return false;
                    }else{
                        return true;
                    }
                }
                None => return false}
        }else if direction == "s"{
            if y + steps >= level.len(){
                return false;
            }else{
                if level[y + steps][x] != " "{
                    return false;
                }else{
                    return true;
                }
            }
        }else if direction == "d"{
            if x + steps >= level[y].len(){
                return false;
            }else{
                if level[y][x + steps] != " "{
                    return false;
                }else{
                    return true;
                }
            }
        }else{
            return false;
        }
    }

    pub fn move_player(&self, input: String) -> (String, usize, String) {
            let input:Vec<&str> = input.split(" ").map(|s| s.trim()).collect();
            let direction = match input[0]{
                "w" => String::from("w"),
                "a" => String::from("a"),
                "s"=> String::from("s"),
                "d" => String::from("d"),
                _ => String::from("invalid"),
            };
            let steps:usize = match input.get(1) {
                None => 1,
                Some(steps) => match steps.parse() {
                    Err(_) => 1,
                    Ok(steps) => steps
                }
            };
            if direction == "invalid" {
                (direction, steps, String::from("Invalid direction !"))
            }else{
                (direction, steps, String::from("ok"))
            }
    }
}