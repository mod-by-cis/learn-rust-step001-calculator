// 1. Deklaracja modułów (powiązanie z folderami i plikami)
// Słowo 'pub' sprawia, że moduły są widoczne poza biblioteką (np. w cli1.rs)
// pub mod logic {
//     pub mod operations; // odpowiada plikowi src/logic/operations.rs
//     pub mod input;      // odpowiada plikowi src/logic/input.rs
// }
//
// pub mod ui {
//     pub mod keyboard;   // odpowiada plikowi src/ui/keyboard.rs
// }
// Szuka plików logic/mod.rs oraz ui/mod.rs
pub mod logic;
pub mod ui;

// 2. Re-eksporty (Facade Pattern)
// Pozwalają na krótszy zapis w binarkach.
// Zamiast: use src_rust::logic::operations::add;
// Możesz pisać: use src_rust::add;
// Dzięki temu w cli1.rs piszesz: use src_rust::{execute, Action};
pub use crate::logic::execute;
pub use crate::logic::input::{get_number, get_operator};
pub use crate::logic::operations::Operator;
pub use crate::logic::operations::*;
pub use crate::ui::keyboard::{wait_for_action, Action};
