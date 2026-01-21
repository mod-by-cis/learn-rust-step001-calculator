// WERSJA-1/source/core/main.rs
mod calc;
mod gui;
mod utils;

fn main() {
    println!("--- Kalkulator Rust v2026 (Perfect Structure) ---");
    let mut current_result: Option<f64> = None;

    loop {
        // 1. WYBÓR TRYBU (tylko jeśli nie mamy wyniku)
        if current_result.is_none() {
            println!("\n[ WYBÓR TRYBU ]: [c] Konsola | [g] GUI | [w] Wyjdź");
            match utils::wait_for_key(&['c', 'g', 'w']) {
                'g' => {
                    gui::run_gui(0.0);
                    continue;
                }
                'w' => break, // Używamy break zamiast return dla czystości
                _ => println!("\r--- Tryb Konsolowy ---"),
            }
        }

        // 2. POBIERANIE DANYCH
        let num1 = match current_result {
            Some(res) => {
                println!("\nWynik bieżący: {}", res);
                res
            }
            None => utils::get_input_number("Podaj pierwszą liczbę: "),
        };

        let op = utils::get_input_string("Podaj operator (+, -, *, /): ");
        let num2 = utils::get_input_number("Podaj drugą liczbę: ");

        // 3. LOGIKA I WYNIK
        match calc::execute(num1, num2, &op) {
            Ok(wynik) => {
                println!("WYNIK: {}", wynik);

                println!("\n[ KROK ]: [c] Dalej | [r] Reset | [g] GUI | [w] Wyjdź");
                match utils::wait_for_key(&['c', 'r', 'g', 'w']) {
                    'c' => current_result = Some(wynik),
                    'r' => current_result = None,
                    'g' => {
                        gui::run_gui(wynik);
                        current_result = Some(wynik);
                    }
                    'w' => break,
                    _ => {}
                }
            }
            Err(e) => {
                println!("BŁĄD: {}", e);
                current_result = None;
            }
        }
    }
    println!("\rDo zobaczenia!");
}