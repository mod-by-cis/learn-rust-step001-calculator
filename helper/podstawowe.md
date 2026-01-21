# Komendy Podstawowe i UI (Windows/PC)

## Deno (Automatyzacja Frontend)
- `deno task build-ui`
  Buduje plik `bundle.js` z TypeScripta i automatycznie kopiuje `index.html` z folderu logic do build.
- `deno task start`
  Uruchamia wersję **hybrydową** (najpierw konsola, GUI na żądanie pod klawiszem [g]).
- `deno task gui`
  Uruchamia bezpośrednio okno **GUI** bez przechodzenia przez menu konsolowe.

## Cargo (Rust - Manualnie)
- `cargo run --bin calc-hybrid`
  Ręczne uruchomienie wersji hybrydowej.
- `cargo run --bin calc-gui`
  Ręczne uruchomienie wersji tylko-GUI.
- `cargo build --release`
  Kompiluje ostateczne pliki `.exe` w folderze `target/release/`.

## Rozwiązywanie problemów
- `Invoke-WebRequest -Uri "https://tauri.app/favicon.ico" -OutFile "icons/icon.ico"`
  Pobiera ikonę projektu, jeśli jej brakuje (zapobiega błędom kompilacji zasobów Windows).