// ./src-tauri/src/lib.rs

use lekcja003_kalkulator_cli_lib::{execute, Operator}; // Teraz to zadziała, bo dodaliśmy ścieżkę w Cargo.toml

// --- KOMENDA 1: OBLICZENIA ---
#[tauri::command]
fn calculate_command(a: f64, b: f64, op: &str) -> Result<f64, String> {
    // 1. Zamieniamy tekstowy operator z GUI na nasz bezpieczny Enum
    let operator = match op {
        "+" => Operator::Add,
        "-" => Operator::Sub,
        "*" => Operator::Mul,
        "/" => Operator::Div,
        "%" => Operator::Mod,
        "^" => Operator::Pow,
        "v" => Operator::Root,
        _ => return Err("Nieznany operator".to_string()),
    };

    // 2. Wywołujemy Twoją funkcję execute z src_rust
    execute(a, b, operator)
}

// --- KOMENDA 2: SYNCHRONIZACJA (Opcjonalna) ---
#[tauri::command]
fn get_initial_value() -> f64 {
    0.0 // Możesz tu w przyszłości dodać odczyt z bazy danych lub pliku
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // REJESTRACJA KOMEND: Tutaj łączymy funkcje Rust z API Tauri
        .invoke_handler(tauri::generate_handler![
            calculate_command,
            get_initial_value
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
