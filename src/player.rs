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

    pub fn can_move(&self, level: &Vec<Vec<String>>, direction: &str, steps: usize) -> bool{
        let x = self.position.0;
        let y = self.position.1;
        let mut result = true;
        if direction == "w" {
            for i in 0..=steps{
                if level[y - i][x] != " "{
                    result = false;
                    break;
                }
                result = true;
            }
        }else if direction == "s" {
            for i in 0..=steps{
                if level[y + i][x] != " "{
                    result = false;
                    break;
                }
                result = true;
            }
        }else if direction == "a" {
            for i in 0..=steps{
                if level[y][x - i] != " "{
                    result = false;
                    break;
                }
                result = true;
            }
        }else if direction == "d" {
            for i in 0..=steps{
                if level[y][x + i] != " "{
                    result = false;
                    break;
                }
                result = true;
            }
        }
        return result;
    }
    pub fn move_player(&mut self,level:&Vec<Vec<String>> , input: String){
        let input:Vec<&str> = input.split(" ").map(|s| s.trim()).collect();
        let steps:usize = if input.len() < 2 { 1 } else { match input[1].trim().parse() { Err(_) => 0, Ok(steps) => steps } };
        let direction = input[0];
    }
}
