// ./src-rust/src/logic/input.rs
use crate::logic::operations::Operator;
use crossterm::style::Stylize;
use std::io::{self, Write};

/// POMOCNICZA: Pobiera surowy tekst z konsoli (odpowiednik Twojego pobierz_tekst)
pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush(); // Ignorujemy błąd flush, bo jest mało prawdopodobny

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Krytyczny błąd odczytu z klawiatury");

    buffer.trim().to_string()
}

/// GŁÓWNA: Pobiera poprawną liczbę (wykorzystuje get_input)
pub fn get_number(prompt: &str) -> f64 {
    loop {
        let text = get_input(prompt);
        match text.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!(
                "{}",
                format!("Błąd: '{}' nie jest liczbą. Spróbuj ponownie.", text).red()
            ),
        }
    }
}

/// GŁÓWNA: Pobiera operator i ZWRACA ENUM (Operator)
//pub fn get_operator() -> String {
//    let allowed_operators = ["+", "-", "*", "/", "%", "^", "v"];
//    loop {
//        print!("Wybierz operację [ + - * / % ^ v ]: ");
//        let _ = io::stdout().flush();
//
//        let mut op = String::new();
//        io::stdin().read_line(&mut op).expect("Failed to read operator");
//        let op = op.trim().to_string();
//
//        if allowed_operators.contains(&op.as_str()) {
//            return op;
//        }
//        println!("{}", "Błąd: Nieznany operator. Spróbuj ponownie.".red());
//    }
//}
pub fn get_operator() -> Operator {
    loop {
        // Używamy naszej pomocniczej funkcji get_input
        let op_text = get_input("Wybierz operację [ + - * / % ^ v ]: ");

        // Matchujemy tekst bezpośrednio na warianty Enuma
        match op_text.as_str() {
            "+" => return Operator::Add,
            "-" => return Operator::Sub,
            "*" => return Operator::Mul,
            "/" => return Operator::Div,
            "%" => return Operator::Mod,
            "^" => return Operator::Pow,
            "v" => return Operator::Root,
            _ => {
                // Jeśli wpisano coś innego, wyświetlamy błąd i pętla 'loop' pyta ponownie
                println!("{}", "Błąd: Nieznany operator. Spróbuj ponownie.".red());
            }
        }
    }
}
