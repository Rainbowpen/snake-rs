//
// ░██████╗███╗░░██╗░█████╗░██╗░░██╗███████╗░░░░░░██████╗░░██████╗
// ██╔════╝████╗░██║██╔══██╗██║░██╔╝██╔════╝░░░░░░██╔══██╗██╔════╝
// ╚█████╗░██╔██╗██║███████║█████═╝░█████╗░░█████╗██████╔╝╚█████╗░
// ░╚═══██╗██║╚████║██╔══██║██╔═██╗░██╔══╝░░╚════╝██╔══██╗░╚═══██╗
// ██████╔╝██║░╚███║██║░░██║██║░╚██╗███████╗░░░░░░██║░░██║██████╔╝
// ╚═════╝░╚═╝░░╚══╝╚═╝░░╚═╝╚═╝░░╚═╝╚══════╝░░░░░░╚═╝░░╚═╝╚═════╝░
//
// A snake game written in rust.
// Created by Simon Zheng 2025.

//
// To-do
// - different bot logic.
// - zoo mode.
//

use snake_rs::utils::*;
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use terminal_size::{terminal_size, Height, Width};

fn main() {
    let size = terminal_size();
    let terminal_w: u16;
    let terminal_h: u16;
    if let Some((Width(w), Height(h))) = size {
        terminal_w = w;
        terminal_h = h;
    } else {
        println!("Unable to get terminal size");
        return;
    }
    println!(
        "Your terminal is {} cols wide and {} lines tall",
        terminal_w, terminal_h
    );

    let snake_game = Arc::new(Mutex::new(Game::new(terminal_w.into(), terminal_h.into())));

    let game_data = Arc::clone(&snake_game);
    let init_game = thread::spawn(move || {
        start_game(&game_data);
    });

    let game_keypress_data = Arc::clone(&snake_game);
    let keypress = thread::spawn(move || {
        read_keypress(&game_keypress_data);
    });

    keypress.join().unwrap();
    init_game.join().unwrap();
}
