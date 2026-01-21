// ./src-rust/src/logic/operations.rs

#[derive(Debug, Clone, Copy)] // Pozwala na łatwe kopiowanie i debugowanie
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Root,
}
impl Operator {
    pub fn as_symbol(&self) -> &str {
        match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::Mod => "%",
            Operator::Pow => "^",
            Operator::Root => "v",
        }
    }
}

/// Dodawanie dwóch liczb
pub fn add(a: f64, b: f64) -> Result<f64, String> {
    Ok(a + b)
}

/// Odejmowanie (a - b)
pub fn sub(a: f64, b: f64) -> Result<f64, String> {
    Ok(a - b)
}

/// Mnożenie dwóch liczb
pub fn mul(a: f64, b: f64) -> Result<f64, String> {
    Ok(a * b)
}

/// Dzielenie (a / b).
/// Zwraca None w przypadku próby dzielenia przez zero.
pub fn div(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Błąd: Nie można dzielić przez zero!".to_string())
    } else {
        Ok(a / b)
    }
}

/// Reszta z dzielenia (modulo)
pub fn modulo(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Błąd: Modulo przez zero!".to_string())
    } else {
        Ok(a % b)
    }
}

/// Potęgowanie (a do potęgi b)
pub fn power(a: f64, b: f64) -> Result<f64, String> {
    Ok(a.powf(b))
}

/// Pierwiastkowanie n-tego stopnia (a pod pierwiastkiem, n to stopień)
/// Zwraca None jeśli:
/// 1. Stopień pierwiastka (n) to 0.
/// 2. Próbujemy wyciągnąć pierwiastek stopnia parzystego z liczby ujemnej.
pub fn root(a: f64, n: f64) -> Result<f64, String> {
    if n == 0.0 {
        return Err("Błąd: Stopień pierwiastka nie może wynosić 0!".to_string());
    }

    // Sprawdzenie: pierwiastek parzystego stopnia z liczby ujemnej
    if a < 0.0 && n % 2.0 == 0.0 {
        return Err("Błąd: Pierwiastek parzystego stopnia z liczby ujemnej!".to_string());
    }

    // Specjalna obsługa dla pierwiastków nieparzystych z liczb ujemnych
    // (standardowe powf może zwrócić NaN dla ujemnej podstawy)
    let result = if a < 0.0 {
        -(-a).powf(1.0 / n)
    } else {
        a.powf(1.0 / n)
    };

    Ok(result)
}
