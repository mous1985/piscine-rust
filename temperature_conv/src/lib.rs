pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    let celsius = (f - 32.0) * (5.0 / 9.0);
    let rounded_celsius = (celsius * 100.0).round() / 100.0;
    rounded_celsius
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    let fahrenheit = (c * (9.0 / 5.0)) + 32.0;
    let rounded_fahrenheit = (fahrenheit * 100.0).round() / 100.0;
    rounded_fahrenheit
}

    