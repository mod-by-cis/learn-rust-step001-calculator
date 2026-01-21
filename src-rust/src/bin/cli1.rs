// ./src-rust/src/bin/cli1.rs

use crossterm::style::Stylize;
// Importujemy typy i funkcje wystawione w lib.rs
use lekcja003_kalkulator_cli_lib::{
    execute, get_number, get_operator, wait_for_action, Action, Operator,
};

fn main() {
    // 1. Inicjalizacja wizualna
    print_welcome_screen();

    // Zmienna 'pamięci' kalkulatora (Accumulator)
    let mut current_value: Option<f64> = None;

    'main_loop: loop {
        // --- KROK 1: USTALANIE PIERWSZEJ LICZBY (A) ---
        let a = match current_value {
            Some(val) => {
                println!(
                    "\n{} {}",
                    "Memory value:".dark_grey(),
                    val.to_string().yellow().bold()
                );
                val
            }
            None => get_number("\nPodaj pierwszą liczbę: "),
        };

        // --- KROK 2: POBIERANIE OPERATORA (ENUM) ---
        // Ta funkcja zwraca teraz typ Operator, nie String!
        let op = get_operator();

        // --- KROK 3: POBIERANIE DRUGIEJ LICZBY (B) ---
        let b = get_number("Podaj drugą liczbę: ");

        // --- KROK 4: OBLICZENIA ---
        match execute(a, b, op) {
            Ok(wynik) => {
                display_summary(a, op, b, wynik);
                current_value = Some(wynik); // Zapisujemy do pamięci
            }
            Err(e) => {
                println!("\n{} {}", "❌ BŁĄD:".red().bold(), e);
                current_value = None; // Resetujemy pamięć w przypadku błędu
            }
        }

        // --- KROK 5: DECYZJA UŻYTKOWNIKA (Crossterm) ---
        println!("\n{}", "[C] Continue | [R] Reset | [E] Exit".dark_grey());

        match wait_for_action() {
            Action::Continue => {
                // Czyścimy linię wyboru, żeby było estetycznie (opcjonalne)
                continue 'main_loop;
            }
            Action::Reset => {
                current_value = None;
                println!("{}", "\n--- RESETOWANIE PAMIĘCI ---".yellow());
                continue 'main_loop;
            }
            Action::Exit => {
                println!(
                    "{}",
                    "\nZamykanie kalkulatora... Do zobaczenia!".green().bold()
                );
                break 'main_loop;
            }
        }
    }
}

// --- FUNKCJE POMOCNICZE DLA UI ---

fn print_welcome_screen() {
    println!("{}", "========================================".magenta());
    println!("{}", "   RUST HYBRID CALCULATOR v2026 ".bold().cyan());
    println!("{}", "      Perfect Structure Edition".italic().dark_grey());
    println!("{}", "========================================".magenta());
}

fn display_summary(a: f64, op: Operator, b: f64, wynik: f64) {
    println!("{}", "----------------------------------------".dark_grey());
    // Używamy op.as_symbol() z naszej implementacji w operations.rs
    println!(
        "{} {} {} {} {}",
        a.to_string().white(),
        op.as_symbol().cyan(),
        b.to_string().white(),
        "=".cyan(),
        wynik.to_string().green().bold()
    );
    println!("{}", "----------------------------------------".dark_grey());
}
