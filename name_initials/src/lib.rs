
pub fn initials(names: Vec<&str>) -> Vec<String> {
    names.iter().map(|name| {
        let mut initials = String::with_capacity(name.len());
        for word in name.split_whitespace() {
            if let Some(initial) = word.chars().next() {
                initials.push(initial.to_ascii_uppercase());
                initials.push('.');
                initials.push(' ');
            }
        }
        initials = initials.strip_suffix(". ").unwrap_or(&initials).to_string();
        initials.push('.');
        initials
    }).collect()
}