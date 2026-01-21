// ./src-rust/src/ui/keyboard.rs
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

#[derive(Debug, PartialEq)]
pub enum Action {
    Continue,
    Reset,
    Exit,
}

/// Czeka na naciśnięcie jednego z dozwolonych klawiszy i zwraca Action.
/// Funkcja automatycznie włącza i wyłącza tryb surowy (raw mode).
pub fn wait_for_action() -> Action {
    // Włączamy tryb surowy, by czytać klawisze bez czekania na Enter
    enable_raw_mode().expect("Nie udało się włączyć trybu raw");

    let action = loop {
        // Czytamy następne zdarzenie z kolejki
        if let Ok(Event::Key(key_event)) = event::read() {
            // W systemie Windows zdarzenia są wysyłane dwukrotnie (Press i Release).
            // Filtrujemy tylko naciśnięcia klawiszy.
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Char('c') | KeyCode::Char('C') => break Action::Continue,
                    KeyCode::Char('r') | KeyCode::Char('R') => break Action::Reset,
                    KeyCode::Char('e') | KeyCode::Char('E') => break Action::Exit,
                    _ => continue, // Ignorujemy inne klawisze
                }
            }
        }
    };

    // PAMIĘTAJ: Zawsze wyłączaj tryb surowy przed powrotem do normalnego działania!
    disable_raw_mode().expect("Nie udało się wyłączyć trybu raw");
    action
}
