// Deklarujemy pliki znajdujące się w tym samym folderze
pub mod input;
pub mod operations;

// Używamy aliasu 'ops', żeby kod w 'execute' był krótszy i czytelniejszy
//use crate::logic::operations as ops;
use operations::{self as ops, Operator}; // Importujemy Enum i funkcje

/// Główny dispatcher logiki
/// Przyjmuje dwie liczby i symbol operacji, zwraca Result z wynikiem lub opisem błędu.
//pub fn execute(a: f64, b: f64, op: &str) -> Result<f64, String> {
//    match op {
//        "+" => ops::add(a, b),
//        "-" => ops::sub(a, b),
//        "*" => ops::mul(a, b),
//        "/" => ops::div(a, b),
//        "%" => ops::modulo(a, b),
//        "^" => ops::power(a, b),
//        "v" => ops::root(a, b),
//        _   => Err(format!("Error: Unknown operator '{}'", op)),
//    }
//}
pub fn execute(a: f64, b: f64, op: Operator) -> Result<f64, String> {
    // Zauważ: matchuje teraz po Enumie, nie po tekście!
    match op {
        Operator::Add => ops::add(a, b),
        Operator::Sub => ops::sub(a, b),
        Operator::Mul => ops::mul(a, b),
        Operator::Div => ops::div(a, b),
        Operator::Mod => ops::modulo(a, b),
        Operator::Pow => ops::power(a, b),
        Operator::Root => ops::root(a, b),
    }
}
