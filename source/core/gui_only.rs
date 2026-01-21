// source/core/gui_only.rs
mod calc;
mod gui; // Rust znajdzie plik gui.rs w tym samym folderze

fn main() {
    // Od razu uruchamiamy GUI z wartością startową 0.0
    gui::run_gui(0.0);
}
