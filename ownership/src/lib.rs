pub fn first_subword(mut s: String) -> String {
    // Trouver la position du premier caractère en majuscule ou du caractère de soulignement
    let first_upper_or_underscore = s
        .find(|c: char| c.is_ascii_uppercase() || c == '_')
        .unwrap_or_else(|| s.len());

    // Extraire la première sous-chaîne jusqu'à cette position
    let subword: String = s.drain(..first_upper_or_underscore).collect();

    // Si la sous-chaîne extraite est vide, renvoyer la chaîne originale
    if subword.is_empty() {
        s
    } else {
        subword
    }
}
