# Dokumentacja Architektury Projektu

Projekt został podzielony na moduły zgodnie z zasadą separacji odpowiedzialności (Separation of Concerns).

## 1. Katalog: `source/core/` (Backend - Rust)
To serce aplikacji, gdzie rezyduje logika i zarządzanie procesami.

* **`main.rs`**
    Dyrygent aplikacji. Zarządza pętlą główną, menu wyboru trybu (CLI/GUI) i przepływem danych w konsoli.
* **`calc.rs`**
    "Mózg" matematyczny. Zawiera czystą logikę obliczeń (funkcja `execute`). Jest niezależny od interfejsu (używany przez CLI i GUI).
* **`utils.rs`**
    Zmysły terminalowe. Obsługuje niskopoziomowe operacje wejścia/wyjścia, czytanie klawiszy przez `crossterm` i formatowanie komunikatów.
* **`gui.rs`**
    Most do świata wizualnego. Konfiguruje okno Tauri, zarządza stanem (`AppState`) i eksponuje komendy dla TypeScripta.
* **`gui_only.rs`**
    Alternatywny punkt wejścia. Binarka służąca wyłącznie do błyskawicznego odpalenia interfejsu graficznego.

## 2. Katalog: `source/gui/` (Frontend - TS/HTML)
Warstwa wizualna i interakcja z użytkownikiem przez WebView.

* **`logic/main.ts`**
    Logika interfejsu w Preact. Obsługuje stan wyświetlacza, zdarzenia klawiatury i wysyła zapytania (`invoke`) do Rusta.
* **`logic/index.html`**
    Wzorzec struktury dokumentu. Kopiowany do folderu build podczas budowania.
* **`build/`**
    Folder wynikowy (artefakty). Zawiera skompilowany `bundle.js` i gotowy `index.html`. To stąd Tauri pobiera pliki.

## 3. Katalog: `helper/` (Wsparcie)
* **`podstawowe.md`** - Ściąga z komendami dla Windows/Rust.
* **`android.md`** - Instrukcje kompilacji mobilnej i SDK.
* **`struktura.md`** - Niniejszy opis architektury.