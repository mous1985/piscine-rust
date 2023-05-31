pub fn first_subword(s: String) -> String {
    let mut subword = String::new();

    for c in s.chars() {
        if c.is_ascii_uppercase() || c == '_' {
            if !subword.is_empty() {
                break;
            }
        }
        if c != '_' {
            subword.push(c);
        }
    }

    subword
}
