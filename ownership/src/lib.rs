pub fn first_subword(mut s: String) -> String {
    
    let first_upper_or_underscore = s
        .find(|c: char| c.is_ascii_uppercase() || c == '_')
        .unwrap_or_else(|| s.len());

    
    let subword = s.drain(..first_upper_or_underscore).collect();

    subword
}

