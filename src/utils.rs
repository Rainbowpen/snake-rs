use rand::prelude::*;
use std::io::{stdin, stdout, Write};
use std::iter;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Debug)]
pub struct Snake {
    pub id: u32,
    pub name: String,
    direction: &'static str,
    pub position: Vec<(i32, i32)>,
    pub old_position: Vec<(i32, i32)>,
    pub lenght: i32,
    pub moved: bool,
    pub dead: bool,
}

impl Clone for Snake {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            direction: self.direction,
            position: self.position.clone(),
            old_position: self.old_position.clone(),
            lenght: self.lenght,
            moved: self.moved,
            dead: self.dead,
        }
    }
}
impl Snake {
    pub fn new(x_min: usize, x_max: usize, y_min: usize, y_max: usize) -> Self {
        let mut rng = rand::rng();
        Self {
            id: rng.random_range(1..1000),
            name: String::from("Si"),
            direction: {
                match rng.random_range(0..4) {
                    0 => "left",
                    1 => "right",
                    2 => "up",
                    3 => "down",
                    _ => "left",
                }
            },
            position: {
                let x = rng.random_range(x_min as i32..x_max as i32);
                let y = rng.random_range(y_min as i32..y_max as i32);
                vec![(x, y)]
            },
            old_position: vec![],
            lenght: 1,
            moved: false,
            dead: false,
        }
    }

    pub fn change_direction(&mut self, direc: &'static str) {
        match direc {
            "left" => {
                if self.direction != "right" && self.moved {
                    self.moved = false;
                    self.direction = "left"
                }
            }
            "right" => {
                if self.direction != "left" && self.moved {
                    self.moved = false;
                    self.direction = "right"
                }
            }
            "up" => {
                if self.direction != "down" && self.moved {
                    self.moved = false;
                    self.direction = "up"
                }
            }
            "down" => {
                if self.direction != "up" && self.moved {
                    self.moved = false;
                    self.direction = "down"
                }
            }
            _ => (),
        }
    }
    pub fn step_forward(&mut self) {
        // check if snake is dead.
        if self.dead {
            // make a cool animation.
            self.delete_one_from_head();
            return;
        }

        let new_x: i32;
        let new_y: i32;
        self.old_position = self.position.clone();
        match self.direction {
            "left" => {
                new_x = self.position[0].0 - 1;
                new_y = self.position[0].1;
            }
            "right" => {
                new_x = self.position[0].0 + 1;
                new_y = self.position[0].1;
            }
            "up" => {
                new_x = self.position[0].0;
                new_y = self.position[0].1 - 1;
            }
            "down" => {
                new_x = self.position[0].0;
                new_y = self.position[0].1 + 1;
            }
            _ => {
                new_x = self.position[0].0;
                new_y = self.position[0].1;
            }
        }
        self.position.insert(0, (new_x, new_y));
        self.position.remove(self.position.len() - 1);
        self.moved = true;
    }

    pub fn add_one(&mut self) {
        let new_x: i32;
        let new_y: i32;
        match self.direction {
            "left" => {
                new_x = self.position[0].0 - 1;
                new_y = self.position[0].1;
            }
            "right" => {
                new_x = self.position[0].0 + 1;
                new_y = self.position[0].1;
            }
            "up" => {
                new_x = self.position[0].0;
                new_y = self.position[0].1 - 1;
            }
            "down" => {
                new_x = self.position[0].0;
                new_y = self.position[0].1 + 1;
            }
            _ => {
                new_x = self.position[0].0;
                new_y = self.position[0].1;
            }
        }
        self.position.insert(0, (new_x, new_y));
        self.lenght += 1;
    }

    pub fn delete_one_from_head(&mut self) {
        if self.position.len() >= 1 {
            self.position.remove(0);
        }
    }

    pub fn get_direction(&self) -> &'static str {
        self.direction
    }
}

#[derive(Debug)]
pub struct Apple {
    pub x: usize,
    pub y: usize,
}

impl Apple {
    pub fn new(x_min: usize, x_max: usize, y_min: usize, y_max: usize) -> Self {
        let mut rng = rand::rng();
        let x: usize = rng.random_range(x_min..x_max);
        let y: usize = rng.random_range(y_min..y_max);
        Self { x: x, y: y }
    }
}

impl Clone for Apple {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub new_game: bool,
    pub quit: bool,
    pub stop: bool,
    pub game_over_status: bool,
    pub terminal_w: usize,
    pub terminal_h: usize,
    pub map: Vec<Vec<&'static str>>,
    // pub snake: Snake,
    pub snakes: Vec<Snake>,
    // pub bot_snakes: Vec<Snake>,
    pub apple: Vec<Apple>,
}

impl Game {
    pub fn new(map_x: usize, map_y: usize) -> Self {
        Self {
            new_game: true,
            quit: false,
            stop: false,
            game_over_status: false,
            terminal_w: map_x,
            terminal_h: map_y,
            map: {
                let mut new_map = vec![vec![""; map_y]; map_x];
                for i in 0..map_x {
                    for j in 0..map_y {
                        if i == 1 || j == 1 || i == map_x - 1 || j == map_y - 1 {
                            new_map[i][j] = "wall";
                        }
                    }
                }
                new_map
            },
            apple: vec![Apple::new(2, map_x - 2, 2, map_y - 2)],
            snakes: vec![Snake::new(2, map_x - 2, 2, map_y - 2)],
        }
    }

    pub fn new_snake(&mut self) -> Snake {
        // generate a new snake on the opposite side of player.
        // the positions as the following diagram:
        //+---+---+
        //| 1 | 2 |
        //+---+---+
        //| 3 | 4 |
        //+---+---+
        //play at 1, new snake will be at 4
        //play at 2, new snake will be at 3
        //play at 3, new snake will be at 2
        //play at 4, new snake will be at 1
        let min_x: usize = 10;
        let max_x: usize = self.terminal_w - 10;
        let min_y: usize = 10;
        let max_y: usize = self.terminal_h - 10;
        if self.snakes.len() == 0 {
            return Snake::new(min_x, max_x, min_y, max_y);
        }
        let player = &self.snakes[0];
        if player.position.len() > 0 {
            if (player.position[0].0 <= (self.terminal_w / 2) as i32
                && player.position[0].1 <= (self.terminal_h / 2) as i32)
            {
                //1
                Snake::new(min_x, self.terminal_w / 2, min_y, self.terminal_h)
            } else if (player.position[0].0 >= (self.terminal_w / 2) as i32
                && player.position[0].1 <= (self.terminal_h / 2) as i32)
            {
                //2
                Snake::new(self.terminal_w / 2, max_x, min_y, self.terminal_h)
            } else if (player.position[0].0 >= (self.terminal_w / 2) as i32
                && player.position[0].1 >= (self.terminal_h / 2) as i32)
            {
                //3
                Snake::new(min_x, self.terminal_w / 2, self.terminal_h / 2, max_y)
            } else {
                //4
                Snake::new(self.terminal_w / 2, max_x, self.terminal_h / 2, max_y)
            }
        } else {
            Snake::new(min_x, max_x, min_y, max_y)
        }
    }

    pub fn start_new_game(&mut self) {
        self.new_game = true;
        self.stop = false;
        self.game_over_status = false;
        self.snakes = vec![Snake::new(
            10,
            self.terminal_w - 10,
            10,
            self.terminal_h - 10,
        )];
        for _i in 0..5 {
            let tmp = self.new_snake();
            self.snakes.push(tmp);
        }

        self.apple.clear();
        for _i in 0..40 {
            self.add_new_apple();
        }
    }

    pub fn add_new_apple(&mut self) {
        self.apple
            .push(Apple::new(2, self.terminal_w - 2, 2, self.terminal_h - 2));
    }

    pub fn delete_apple(&mut self, index: usize) {
        self.apple.remove(index);
    }

    pub fn stop_game(&mut self) {
        self.stop = true;
    }
    pub fn resume_game(&mut self) {
        self.stop = false;
    }

    pub fn quit_game(&mut self) {
        self.quit = true;
    }

    pub fn check_snake_eat_apple(&mut self, index: usize) {
        // eat apple body strong.
        // Have to use this way to avoid borrow problem.
        let indices_to_remove: Vec<usize> = self
            .apple
            .iter()
            .enumerate()
            .filter_map(|(i, apple)| {
                if self.snakes[index].position[0] == (apple.x as i32, apple.y as i32) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        for &i in indices_to_remove.iter().rev() {
            self.snakes[index].add_one();
            self.delete_apple(i);
            self.add_new_apple();
        }
    }

    pub fn check_snake_eat_itself(&mut self, index: usize) {
        // snake eat itself.
        if self.snakes[index].lenght > 1 {
            if self.snakes[index].position[1..]
                .iter()
                .any(|&i| i == self.snakes[index].position[0])
            {
                self.snakes[index].dead = true;
                // if it's the player.
                if index == 0 {
                    self.game_over();
                }
            }
        }
    }

    pub fn check_snake_hit_snake(&mut self, index: usize) {
        // snake hit other snake
        let hit_snake: Vec<usize> = self
            .snakes
            .iter()
            .enumerate()
            .filter_map(|(i, snake)| {
                if i != index
                    && snake
                        .position
                        .iter()
                        .any(|&i| i == self.snakes[index].position[0])
                {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        for &i in hit_snake.iter().rev() {
            self.snakes[index].dead = true;
            // if it's the player.
            if index == 0 {
                self.game_over();
            }
        }
    }

    pub fn check_snakes_hit_wall(&mut self, index: usize) {
        // snake hit the walls.
        let snake_x = self.snakes[index].position[0].0;
        let snake_x_usize: usize;
        let snake_y = self.snakes[index].position[0].1;
        let snake_y_usize: usize;
        let mut dead = false;

        // check if snake is out of bounds before check is it hit the walls.
        if snake_x >= 0 {
            snake_x_usize = snake_x as usize;
        } else {
            snake_x_usize = 0;
            dead = true;
        }

        if snake_y >= 0 {
            snake_y_usize = snake_y as usize;
        } else {
            snake_y_usize = 0;
            dead = true;
        }
        if snake_x_usize >= self.map.len() || snake_y_usize >= self.map[0].len() {
            dead = true;
        } else {
            if self.map[snake_x_usize][snake_y_usize] == "wall" {
                dead = true;
            }
        }

        if dead {
            self.snakes[index].dead = true;

            // only non player gets to respawn.
            if index == 0 {
                self.game_over();
            }
        }
    }

    pub fn check_rules(&mut self) {
        for i in 0..self.snakes.len() {
            // if snake is empty, don't check anything.
            if self.snakes[i].position.len() <= 0 {
                return;
            }

            self.check_snake_eat_apple(i);
            self.check_snake_eat_itself(i);
            self.check_snakes_hit_wall(i);
            self.check_snake_hit_snake(i);
        }
    }
    pub fn draw_wall(&self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        for i in 0..self.terminal_w {
            for j in 0..self.terminal_h {
                let x: u16 = i as u16;
                let y: u16 = j as u16;
                if self.map[i][j] == "wall" {
                    write!(stdout, "{}█", termion::cursor::Goto(x, y)).unwrap();
                }
            }
        }
        stdout.flush().unwrap();
    }

    pub fn clear_screen(&self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        //clearing the screen and going to top left corner
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();
        stdout.flush().unwrap();
    }

    pub fn corner_cursor(&self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(
            stdout,
            "{}",
            termion::cursor::Goto(1 as u16, (self.terminal_h + 1) as u16),
        )
        .unwrap();
        stdout.flush().unwrap();
    }

    pub fn update_footer(&self) {
        let player = &self.snakes[0];
        let mut stdout = stdout().into_raw_mode().unwrap();
        // clear the prev text
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto((0) as u16, (self.terminal_h) as u16),
            termion::clear::CurrentLine,
        )
        .unwrap();
        let info = format!(
            "Window size: {},{} | Snake: {:?} | Scores: {} | Left, Down, Up, Right:\"H, J, K, L\" | Quit: \"Q\" | New game:\"R\"",
            self.terminal_w,
            self.terminal_h,
            {
                if player.position.len() >= 1{
                    player.position[0]
                }else{
                    (0,0)
                }
            },
            player.lenght - 1
        );
        write!(
            stdout,
            "{}{}{}{}",
            termion::color::Fg(termion::color::LightGreen),
            termion::cursor::Goto(
                (self.terminal_w / 2 - info.len() / 2) as u16,
                (self.terminal_h) as u16
            ),
            info,
            termion::color::Fg(termion::color::Reset)
        )
        .unwrap();
        stdout.flush().unwrap();
    }

    pub fn game_over(&mut self) {
        self.game_over_status = true;
        let mut stdout = stdout().into_raw_mode().unwrap();
        let info = "Game Over!";
        let quit_into = "Press \"Q\" to quit or \"R\" to start a new game.";
        write!(
            stdout,
            "{}{}{}{}",
            termion::color::Fg(termion::color::LightGreen),
            termion::cursor::Goto(
                (self.terminal_w / 2 - info.len() / 2) as u16,
                (self.terminal_h / 2) as u16
            ),
            info,
            termion::color::Fg(termion::color::Reset)
        )
        .unwrap();
        write!(
            stdout,
            "{}{}{}{}",
            termion::color::Fg(termion::color::LightGreen),
            termion::cursor::Goto(
                (self.terminal_w / 2 - quit_into.len() / 2) as u16,
                (self.terminal_h / 2 + 1) as u16
            ),
            quit_into,
            termion::color::Fg(termion::color::Reset)
        )
        .unwrap();
        stdout.flush().unwrap();
    }

    pub fn update_map(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let player = &self.snakes[0];

        // clean the tails
        for j in 0..self.snakes.len() {
            for i in &self.snakes[j].old_position {
                write!(stdout, "{} ", termion::cursor::Goto(i.0 as u16, i.1 as u16),).unwrap();
            }
        }

        // draw apple.
        for (i, apple) in self.apple.iter().enumerate() {
            write!(
                stdout,
                "{}{}o{}",
                termion::color::Fg(termion::color::Red),
                termion::cursor::Goto(apple.x as u16, apple.y as u16),
                termion::color::Fg(termion::color::Reset)
            )
            .unwrap();
        }

        // draw snake.
        for i in &player.position {
            write!(
                stdout,
                "{}{}█{}",
                termion::color::Fg(termion::color::Blue),
                termion::cursor::Goto(i.0 as u16, i.1 as u16),
                termion::color::Fg(termion::color::Reset)
            )
            .unwrap();
        }

        // draw bot snakes.
        for bot_snake in &self.snakes[1..] {
            for i in &bot_snake.position {
                write!(
                    stdout,
                    "{}{}█{}",
                    termion::color::Fg(termion::color::Yellow),
                    termion::cursor::Goto(i.0 as u16, i.1 as u16),
                    termion::color::Fg(termion::color::Reset)
                )
                .unwrap();
            }
        }
        self.draw_wall();
        stdout.flush().unwrap();
    }

    pub fn is_snake_out_off_map(&self, index: usize) -> bool {
        let bot = &self.snakes[index];
        // check is bot's body exsit.
        if bot.position.len() == 0 {
            return true;
        }

        // check is bot's head in the map
        // broke it down to two parts for easy reading.
        let bot_head = &bot.position[0];
        if bot_head.0 <= 0 || bot_head.1 <= 0 {
            return true;
        }
        if bot_head.0 >= self.terminal_w as i32 || bot_head.1 >= self.terminal_h as i32 {
            return true;
        }

        return false;
    }

    pub fn bot_snakes_change_direction(&mut self, index: usize) {
        let mut rng = rand::rng();

        // check is bot out off the map
        let bot = &mut self.snakes[index];
        if bot.position.len() == 0 {
            return;
        }
        // check has bot moved.
        if !bot.moved {
            return;
        }

        let bot_head = &bot.position[0];
        let target_apple: &Apple;
        if index < self.apple.len() {
            target_apple = &self.apple[index];
        } else {
            target_apple = &self.apple[0];
        }
        if self.apple.len() > 0 {
            if bot_head.0 < target_apple.x as i32 && bot_head.1 < target_apple.y as i32 {
                //1
                if bot.direction == "right" || bot.direction == "left" {
                    bot.change_direction("down");
                } else {
                    bot.change_direction("right");
                }
            } else if bot_head.0 > target_apple.x as i32 && bot_head.1 < target_apple.y as i32 {
                //2
                if bot.direction == "left" || bot.direction == "right" {
                    bot.change_direction("down");
                } else {
                    bot.change_direction("left");
                }
            } else if bot_head.0 < target_apple.x as i32 && bot_head.1 > target_apple.y as i32 {
                //3
                if bot.direction == "right" || bot.direction == "left" {
                    bot.change_direction("up");
                } else {
                    bot.change_direction("right");
                }
            } else if bot_head.0 > target_apple.x as i32 && bot_head.1 > target_apple.y as i32 {
                //4
                if bot.direction == "left" || bot.direction == "right" {
                    bot.change_direction("up");
                } else {
                    bot.change_direction("left");
                }
                /////
            } else if bot_head.0 == target_apple.x as i32 {
                if bot_head.1 > target_apple.y as i32 {
                    if bot.direction == "down" {
                        match rng.random_range(0..2) {
                            0 => bot.change_direction("left"),
                            1 => bot.change_direction("right"),
                            _ => bot.change_direction("left"),
                        }
                    } else {
                        bot.change_direction("up");
                    }
                } else {
                    if bot.direction == "up" {
                        match rng.random_range(0..2) {
                            0 => bot.change_direction("left"),
                            1 => bot.change_direction("right"),
                            _ => bot.change_direction("right"),
                        }
                    } else {
                        bot.change_direction("down");
                    }
                }
            } else if bot_head.1 == target_apple.y as i32 {
                if bot_head.0 > target_apple.x as i32 {
                    if bot.direction == "right" {
                        match rng.random_range(0..2) {
                            0 => bot.change_direction("up"),
                            1 => bot.change_direction("down"),
                            _ => bot.change_direction("down"),
                        }
                    } else {
                        bot.change_direction("left");
                    }
                } else {
                    if bot.direction == "left" {
                        match rng.random_range(0..2) {
                            0 => bot.change_direction("up"),
                            1 => bot.change_direction("down"),
                            _ => bot.change_direction("down"),
                        }
                    } else {
                        bot.change_direction("right");
                    }
                }
            }
        }
    }

    pub fn snakes_step_forward(&mut self) {
        for i in 0..self.snakes.len() {
            if i != 0 {
                self.bot_snakes_change_direction(i);
                if self.snakes[i].dead && self.snakes[i].position.len() == 0 {
                    self.snakes[i] = self.new_snake();
                }
                if !self.game_over_status {
                    self.snakes[i].step_forward();
                }
            } else {
                self.snakes[i].step_forward();
            }
        }
    }
}

pub fn start_game(game: &Arc<Mutex<Game>>) {
    loop {
        if let Ok(ref mut game_data) = game.try_lock() {
            if game_data.new_game {
                game_data.start_new_game();
                game_data.clear_screen();
                game_data.new_game = false;
            }
            game_data.snakes_step_forward();
            game_data.check_rules();
            game_data.update_map();
            game_data.update_footer();
            game_data.corner_cursor();
            if game_data.quit {
                break;
            }
        }
        sleep(Duration::from_millis(80));
    }
}

pub fn read_keypress(game: &Arc<Mutex<Game>>) {
    let stdin = stdin();

    //detecting press key events
    'read_key: for c in stdin.keys() {
        let key = match c {
            Ok(k) => k,
            Err(_) => continue, // Skip invalid keys
        };

        'lock_game: loop {
            if let Ok(ref mut game_data) = game.try_lock() {
                let player = &mut game_data.snakes[0];
                match key {
                    Key::Char('h') => {
                        player.change_direction("left");
                    }
                    Key::Char('j') => {
                        player.change_direction("down");
                    }
                    Key::Char('k') => {
                        player.change_direction("up");
                    }
                    Key::Char('l') => {
                        player.change_direction("right");
                    }
                    Key::Char('r') => {
                        game_data.start_new_game();
                    }
                    Key::Char('q') => {
                        game_data.quit_game();
                        // println!("kill the game");
                        break 'read_key;
                    }
                    Key::Ctrl('c') => (),
                    _ => (),
                }
                break 'lock_game;
            }
        }
    }
}
