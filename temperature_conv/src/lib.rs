pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    let result = (f - 32.0) * (9.0 / 5.0);
    println!("{:.16}", result); // Affiche le résultat avec une précision de 16 chiffres après la virgule
    result
}


pub fn celsius_to_fahrenheit(c: f64) -> f64 {
   
    (c * (9.0 / 5.0)) + 32.0
    //°F =( °C x 1,8 ) + 32
    
}

    