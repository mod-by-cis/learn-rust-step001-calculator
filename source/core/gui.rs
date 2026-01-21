// source/core/gui.rs
use std::sync::Mutex;

// Stan współdzielony między Rustem a TypeScriptem
pub struct AppState {
    pub current_value: Mutex<f64>,
}

// 1. Komenda pobierająca wynik startowy z konsoli
#[tauri::command]
fn get_initial_value(state: tauri::State<'_, AppState>) -> f64 {
    *state.current_value.lock().unwrap()
}

// 2. Komenda wykonująca obliczenia - TERAZ KORZYSTA Z MODUŁU CALC
#[tauri::command]
fn oblicz_tauri(a: f64, b: f64, op: String) -> Result<f64, String> {
    // Odwołujemy się do logiki w calc.rs za pomocą ścieżki crate::calc
    crate::calc::execute(a, b, &op)
}

pub fn run_gui(initial_value: f64) {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            current_value: Mutex::new(initial_value),
        })
        .invoke_handler(tauri::generate_handler![get_initial_value, oblicz_tauri])
        .run(tauri::generate_context!("tauri.conf.json"))
        .expect("Błąd podczas uruchamiania GUI");
}
