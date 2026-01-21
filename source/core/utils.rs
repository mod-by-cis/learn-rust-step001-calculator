// WERSJA-1/source/core/utils.rs
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Write};

pub fn wait_for_key(valid_keys: &[char]) -> char {
    enable_raw_mode().unwrap();
    let key = loop {
        // Używamy match z "guardem" (if), co jest najbardziej idiomatyczne
        match event::read() {
            Ok(Event::Key(KeyEvent { code: KeyCode::Char(c), .. })) if valid_keys.contains(&c) => {
                break c;
            }
            _ => continue, // Ignorujemy wszystko inne i kręcimy się dalej
        }
    };
    disable_raw_mode().unwrap();
    key
}

pub fn get_input_string(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn get_input_number(msg: &str) -> f64 {
    get_input_string(msg).parse().unwrap_or(0.0)
}