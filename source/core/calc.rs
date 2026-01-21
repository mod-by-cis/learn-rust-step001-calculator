// source/core/calc.rs

/// Główna funkcja wykonująca operacje matematyczne.
/// Przyjmuje dwie liczby i operator jako tekst.
pub fn execute(a: f64, b: f64, op: &str) -> Result<f64, String> {
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b != 0.0 {
                Ok(a / b)
            } else {
                Err("Dzielenie przez zero!".into())
            }
        }
        _ => Err(format!("Nieznany operator: {}", op)),
    }
}
