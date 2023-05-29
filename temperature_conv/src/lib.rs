pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    let mut celsius : f64=0.0;
    celsius=(f - 32.0) * (5.0 / 9.0);
    celsius
    
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    let mut fahrenheit : f64 =0.0;
    fahrenheit=(c * (9.0 / 5.0)) + 32.0;
    fahrenheit
    //°F =( °C x 1,8 ) + 32
    
}

    