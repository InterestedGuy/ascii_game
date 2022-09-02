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

    pub fn can_move(&self, level: &Vec<Vec<String>>, movement: (&str, usize)) -> bool{
        let x = self.position.0;
        let y = self.position.1;
        let direction = movement.0;
        let steps = movement.1;
        let mut result = true;
        if direction == "w" {
            for i in 0..=steps{
                if level[y - i][x] != " "{
                    result = false;
                    break;
                }
                result = true;
            }
        }
        return result;
    }
}