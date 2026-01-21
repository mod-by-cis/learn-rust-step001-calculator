use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Write};

fn main() {
    println!("--- Kalkulator Rust (Edition 2024) + Crossterm ---");
    let mut current_result: Option<f64> = None;

    loop {
        // 1. Logika liczby (Przypisujemy wynik match bezpośrednio do num1)
        let num1 = match current_result {
            Some(res) => {
                println!("\nKontynuacja z wynikiem: {}", res);
                res // Zwracamy wartość do zmiennej num1
            }
            None => {
                print!("\nPodaj pierwsza liczbe: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Blad! Reset...");
                        continue;
                    }
                }
            }
        };

        // 2. Pobieranie operatora
        print!("Podaj operator (+, -, *, /): ");
        io::stdout().flush().unwrap();
        let mut op_input = String::new();
        io::stdin().read_line(&mut op_input).unwrap();
        let op = op_input.trim(); // Używamy referencji do stringa, nie musimy klonować do String

        // 3. Pobieranie drugiej liczby
        print!("Podaj druga liczbe: ");
        io::stdout().flush().unwrap();
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).unwrap();
        let num2: f64 = match input2.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Blad! Reset...");
                current_result = None;
                continue;
            }
        };

        // 4. Obliczenia (Match jako wyrażenie przypisane do 'wynik')
        let wynik = match op {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 != 0.0 {
                    num1 / num2
                } else {
                    println!("Dzielenie przez zero!");
                    0.0
                }
            }
            _ => {
                println!("Zly operator!");
                0.0
            }
        };

        println!("WYNIK: {}", wynik);

        // --- SEKCJA WYBORU (CROSSTERM) ---
        println!("\nNACISNIJ KLAWISZ: [c] Kontynuuj | [r] Reset | [w] Wyjdz");

        enable_raw_mode().unwrap();
        loop {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char('c') => {
                        disable_raw_mode().unwrap();
                        current_result = Some(wynik);
                        break;
                    }
                    KeyCode::Char('r') => {
                        disable_raw_mode().unwrap();
                        current_result = None;
                        println!("\r--- Zresetowano ---");
                        break;
                    }
                    KeyCode::Char('w') => {
                        disable_raw_mode().unwrap();
                        println!("\rZamykanie...");
                        return;
                    }
                    _ => {}
                }
            }
        }
    }
}