use std::f64;

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let original = c;
    let exponential = (c as f64).exp();
    let absolute_value = (c.abs() as f64).ln();

    (original, exponential, absolute_value)
}

pub fn str_function(a: String) -> (String, String) {
    let mut result = String::new();

    for c in a.chars() {
        if let Some(digit) = c.to_digit(10) {
            let exp_value = digit as f64;
            let exp_result = f64::exp(exp_value);

            result.push_str(&exp_result.to_string());
            result.push(' ');
        }
    }

    (a, result.trim().to_string())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut result = Vec::new();
    let mut log_result = Vec::new();

    for num in b {
        result.push(num);
        log_result.push(f64::ln(num.abs() as f64));
    }

    (result, log_result)
}


/* pub fn str_function(a: String) -> (String, String) {
    let mut result = String::new();

    for c in a.chars() {
        let exp_value = c.to_digit(10).and_then(|digit| digit.checked_pow(2)); // Utilisation de la méthode and_then et checked_pow pour gérer le cas où la conversion échoue

        if let Some(value) = exp_value {
            result.push_str(&value.to_string());
        } else {
            result.push('?'); // En cas d'échec de conversion, ajout d'un marqueur '?' à la chaîne de résultats
        }
    }

    (a, result)
} */


