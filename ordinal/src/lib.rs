pub fn num_to_ordinal(x: u32) -> String {
    match x {
        0 => "0th".to_string(),
        _ => {
            let tens = x % 100;
            let ones = x % 10;
            let suffix = match (tens, ones) {
                (11..=13, _) => "th",
                (_, 1) => "st",
                (_, 2) => "nd",
                (_, 3) => "rd",
                _ => "th",
            };
            format!("{}{}", x, suffix)
        }
    }
}