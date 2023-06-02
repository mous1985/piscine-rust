pub fn to_url(s: &str) -> String {
    let mut url = String::new();
    for a in s.chars() {
        if a == ' ' {
            url.push_str("%20");
        } else {
            url.push(a);
        }
    }
    url
}

