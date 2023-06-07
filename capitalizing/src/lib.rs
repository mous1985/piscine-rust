
pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
    let mut chars = input.chars();
    if let Some(first_char) = chars.next() {
        let capitalized = first_char.to_uppercase();

        
        let rest = chars.collect::<String>();
        return capitalized.to_string() + &rest;
    }

    String::new() 
}
pub fn title_case(input: &str) -> String {
    let mut result=String::new();
    if input.is_empty() {
        return result;
    }

    for word in input.split_whitespace() {
        if let Some(first_char) = word.chars().next() {
            let uppercased = first_char.to_uppercase();
            let capitalized_word = uppercased.chain(word.chars().skip(1)).collect::<String>();
            result.push_str(&capitalized_word);
            result.push(' ');
        }
    }

    result = result.trim_end().to_string();
    result
}
pub fn change_case(input: &str) -> String {
    let mut result = String::new();

    if input.is_empty() {
        return result;
    } else {
        for c in input.chars() {
            if c.is_ascii_uppercase() {
                let lowercase = c.to_ascii_lowercase();
                result.push(lowercase);
            } else if c.is_ascii_lowercase() {
                let uppercase = c.to_ascii_uppercase();
                result.push(uppercase);
            } else {
                result.push(c);
            }
        }

        return result;
    }
}

