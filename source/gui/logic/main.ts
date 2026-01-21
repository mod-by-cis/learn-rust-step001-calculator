import { h, render } from "preact";
import { useState, useEffect } from "preact/hooks";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [display, setDisplay] = useState("0");
  const [prevValue, setPrevValue] = useState<number | null>(null);
  const [operator, setOperator] = useState<string | null>(null);
  const [waitingForNext, setWaitingForNext] = useState(false);

  // --- 1. SYNCHRONIZACJA Z RUSTEM ---
  useEffect(() => {
    // Pobieramy wartość, która była w konsoli w momencie kliknięcia [g]
    invoke<number>("get_initial_value")
      .then((val) => setDisplay(val.toString()))
      .catch((err) => console.error("Błąd synchronizacji:", err));
  }, []);

  // --- 2. LOGIKA OBLICZEŃ (Backend Rust) ---
  const runCalculation = async (a: number, b: number, op: string) => {
    try {
      const res: number = await invoke("oblicz_tauri", { a, b, op });
      setDisplay(res.toString());
      setPrevValue(null);
      setOperator(null);
      setWaitingForNext(true); // Wynik jest gotowy, kolejna cyfra go zastąpi
    } catch (err) {
      setDisplay("Błąd");
    }
  };

  const handleNumber = (num: string) => {
    if (waitingForNext) {
      setDisplay(num);
      setWaitingForNext(false);
    } else {
      setDisplay(display === "0" ? num : display + num);
    }
  };

  const handleOperator = (op: string) => {
    setPrevValue(parseFloat(display));
    setOperator(op);
    setWaitingForNext(true);
  };

  const performEquals = () => {
    if (operator && prevValue !== null) {
      runCalculation(prevValue, parseFloat(display), operator);
    }
  };

  // --- 3. OBSŁUGA KLAWIATURY ---
  useEffect(() => {
    const onKeyDown = (e: KeyboardEvent) => {
      if (/[0-9]/.test(e.key)) handleNumber(e.key);
      if (["+", "-", "*", "/"].includes(e.key)) handleOperator(e.key);
      if (e.key === "Enter" || e.key === "=") performEquals();
      if (e.key === "Escape") setDisplay("0");
      if (e.key === "Backspace") setDisplay(d => d.length > 1 ? d.slice(0, -1) : "0");
      if (e.key === "." || e.key === ",") handleNumber(".");
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [display, prevValue, operator, waitingForNext]);

  // --- 4. STYLE ---
  const btnBase = "padding: 20px; font-size: 1.2rem; border: none; border-radius: 4px; cursor: pointer; color: white;";
  const numBtn = btnBase + " background: #444;";
  const opBtn = btnBase + " background: #f39c12;";
  const specBtn = btnBase + " background: #c0392b;";

  return h("div", { 
    style: "display: flex; flex-direction: column; height: 100vh; background: #1e1e1e; font-family: sans-serif; padding: 10px; color: white;" 
  }, [
    // Wyświetlacz
    h("div", { 
      style: "background: #000; padding: 30px; text-align: right; font-size: 3rem; font-family: monospace; margin-bottom: 10px; border-radius: 4px;" 
    }, display),

    // Siatka przycisków
    h("div", { 
      style: "display: grid; grid-template-columns: repeat(4, 1fr); gap: 8px; flex-grow: 1;" 
    }, [
      // Wiersz 1
      h("button", { style: specBtn + " grid-column: span 3;", onClick: () => setDisplay("0") }, "AC (Esc)"),
      h("button", { style: opBtn, onClick: () => handleOperator("/") }, "/"),

      // Wiersz 2
      h("button", { style: numBtn, onClick: () => handleNumber("7") }, "7"),
      h("button", { style: numBtn, onClick: () => handleNumber("8") }, "8"),
      h("button", { style: numBtn, onClick: () => handleNumber("9") }, "9"),
      h("button", { style: opBtn, onClick: () => handleOperator("*") }, "*"),

      // Wiersz 3
      h("button", { style: numBtn, onClick: () => handleNumber("4") }, "4"),
      h("button", { style: numBtn, onClick: () => handleNumber("5") }, "5"),
      h("button", { style: numBtn, onClick: () => handleNumber("6") }, "6"),
      h("button", { style: opBtn, onClick: () => handleOperator("-") }, "-"),

      // Wiersz 4
      h("button", { style: numBtn, onClick: () => handleNumber("1") }, "1"),
      h("button", { style: numBtn, onClick: () => handleNumber("2") }, "2"),
      h("button", { style: numBtn, onClick: () => handleNumber("3") }, "3"),
      h("button", { style: opBtn, onClick: () => handleOperator("+") }, "+"),

      // Wiersz 5
      h("button", { style: numBtn + " grid-column: span 2;", onClick: () => handleNumber("0") }, "0"),
      h("button", { style: numBtn, onClick: () => handleNumber(".") }, "."),
      h("button", { style: opBtn + " background: #27ae60;", onClick: performEquals }, "=")
    ])
  ]);
}

// Renderowanie do #app zdefiniowanego w ./source/gui/build/index.html
const root = document.getElementById("app");
if (root) render(h(App, null), root);