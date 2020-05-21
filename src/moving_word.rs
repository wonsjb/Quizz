use figlet_rs::{FIGfont};
use console::{Term, style};
use std::io;

pub struct MovingWord {
    vx: i32,
    vy: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    figure: String,
    title: String,
    title_height: i32,
    title_width: i32,
    title_x: i32,
    title_vx: i32,
}

impl MovingWord {

    fn get_figure(name: &str) -> (i32, i32, String) {
        let standard_font = FIGfont::standand().unwrap();
        let mut figure = String::new();
        for line in name.lines() {
            let fig = standard_font.convert(line).expect("Could not convert string");
            figure.push_str(&format!("{}", fig));
        }
        let figure = figure;
        let mut width = 0;
        let mut height = 0;
        for line in figure.lines() {
            if line.len() as i32 > width {
                width = line.len() as i32;
            }
            height+=1;
        }
        (width, height, figure)   
    }

    pub fn new(title: &str, name: &str) -> Self {
        let (width, height, figure) = MovingWord::get_figure(name);
        let (title_width, title_height, title) = MovingWord::get_figure(title);
        MovingWord{vx: 1, vy: 1, x: 1, y: 1, width, height, figure, title, title_height, title_width, title_x: 0, title_vx: 1}
    }

    pub fn change_word(&mut self, title: &str, name: &str) {
        let (width, height, figure) = MovingWord::get_figure(name);
        let (title_width, title_height, title) = MovingWord::get_figure(title);
        self.figure = figure;
        self.width = width;
        self.height = height;
        self.title_height = title_height;
        self.title = title;
        self.title_width = title_width;
    }

    pub fn print(&self, term: &Term) -> io::Result<()> {
        let mut count = 0;
        for (i, line) in self.title.lines().enumerate() {
            term.move_cursor_to(self.title_x as usize, i)?;
            term.write_str(&format!("{}", style(line).red().bright()))?;
            count = i;
        }
        if count > 0 {
            term.move_cursor_to(0, count + 1)?;
            let mut line = String::new();
            for _ in 0..term.size().1 {
                line.push_str("*");
            }
            term.write_str(&format!("{}", style(line).blue().bright()))?;
        }
        for (i, line) in self.figure.lines().enumerate() {
            term.move_cursor_to(self.x as usize, self.title_height as usize + self.y as usize + i)?;
            term.write_str(&format!("{}", style(line).green().bright()))?;
        }
        Ok(())
    }

    pub fn advance(&mut self, term: &Term) {
        let tryx = self.x + self.vx;
        let (terminal_y, terminal_x) = term.size(); 
        if tryx < 0 || (tryx + self.width > terminal_x as i32) {
            self.vx = -self.vx;
            self.x = std::cmp::min(self.x, terminal_x as i32 - self.width);
        }
        let tryy = self.y + self.vy;
        if tryy <= 0 || (tryy + self.height > (terminal_y as i32 - self.title_height)) {
            self.vy = -self.vy;
            self.y = std::cmp::min(self.y, terminal_y as i32 - self.title_height - self.height);
        }
        let try_tx = self.title_x + self.title_vx;
        if try_tx < 0 || (try_tx + self.title_width > terminal_x as i32) {
            self.title_vx = -self.title_vx;
            self.title_x = std::cmp::min(self.title_x, terminal_x as i32 - self.title_width);
        }
        self.x = self.x + self.vx;
        self.y = self.y + self.vy;
        self.title_x = self.title_x + self.title_vx;
    }
}