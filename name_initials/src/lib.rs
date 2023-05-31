pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut initials: Vec<String> = Vec::new();

    for name in names {
        let initials_str: String = name
            .split_whitespace()
            .map(|word| word.chars().next().unwrap().to_ascii_uppercase())
            .collect();

        initials.push(initials_str);
    }

    initials
}
