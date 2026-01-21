# 🧮 Rust Hybrid Calculator v2026

Projekt zaawansowanego kalkulatora konsolowego napisanego w Rust, z wykorzystaniem architektury modułowej oraz biblioteki `crossterm` do obsługi zdarzeń klawiatury w trybie rzeczywistym.

## Polecenia Rust

| Akcja | Folder projektu (`src-rust`) | Folder główny (Workspace) |
| :--- | :--- | :--- |
| 🚀 **Szybki start** | `cargo run --bin lekcja003-kalkulator-cli` | `cargo run -p src_rust --bin lekcja003-kalkulator-cli` |
| ✅ **Szybki Check** | `cargo check` | `cargo check -p src_rust` |
| 🧹 **Formatowanie** | `cargo fmt` | `cargo fmt -p src_rust` |
| 🏗️ **Build (Debug)** | `cargo build` | `cargo build -p src_rust` |
| 📦 **Build (Release)** | `cargo build --release` | `cargo build -p src_rust --release` |
| 🔍 **Analiza (Clippy)** | `cargo clippy` | `cargo clippy -p src_rust` |
| 📚 **Dokumentacja (Podgląd)** | `cargo doc --open` | `cargo doc -p src_rust --open` |
| 🌳 **Drzewo zależności** | `cargo tree` | `cargo tree -p src_rust` |
| ➕ **Dodanie biblioteki** | `cargo add <nazwa>` | `cargo add -p src_rust <nazwa>` |
| 🔄 **Aktualizacja bibliotek** | `cargo update` | `cargo update -p src_rust` |
| 🔊 **Testy (Podgląd)** | `cargo test -- --nocapture` | `cargo test -p src_rust -- --nocapture` |
| 🛠️ **Testowanie** | `cargo test` | `cargo test -p src_rust` |
| 🗑️ **Czyszczenie** | `cargo clean` | `cargo clean -p src_rust` |

---

## 📁 Struktura Modułowa (Perfect Structure)

Projekt jest podzielony na logiczne warstwy:

* **`logic/mod.rs`**: Zarządca logiki (Dispatcher) – zawiera funkcję `execute`, która spaja dane z operacjami.
  * **`logic/operations.rs`**: Rdzeń matematyczny – definicja typu `Enum Operator` oraz atomowe funkcje obliczeniowe.
  * **`logic/input.rs`**: Walidacja wejścia – bezpieczne pobieranie i parsowanie liczb oraz operatorów z konsoli.
* **`ui/mod.rs`**: Zarządca warstwy wizualnej i interakcji.
  * **`ui/keyboard.rs`**: Obsługa klawiatury – wykorzystuje `crossterm` do pracy w trybie `raw mode`.
* **`lib.rs`**: Fasada biblioteki – główny punkt eksportu narzędzi dla zewnętrznych binarek.
* **`bin/cli1.rs`**: Punkt wejścia aplikacji CLI – zarządza pętlą główną i wyświetlaniem wyników.

---

## 📦 Budowanie Standalone (Windows)

Dzięki konfiguracji w `.cargo/config.toml`, projekt kompiluje się z flagą `+crt-static`.
Oznacza to, że wygenerowany plik `.exe` jest w pełni przenośny i nie wymaga instalacji bibliotek MSVC (Visual C++ Redistributable).

---

## 🎹 Obsługa programu

* **C**: Kontynuuj obliczenia z wykorzystaniem poprzedniego wyniku.
* **R**: Resetuj pamięć kalkulatora.
* **E**: Zamknij aplikację.

---
