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

    pub fn can_move(&self, level: &Vec<Vec<String>>, steps:usize) -> bool{
        true
    }
}