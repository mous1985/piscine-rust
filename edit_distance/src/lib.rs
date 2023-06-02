pub fn edit_distance(source: &str, target: &str) -> usize {
    let mut matrix = vec![vec![0; target.len() + 1]; source.len() + 1];
    
    for i in 0..=source.len() {
        matrix[i][0] = i;
    }    
    for j in 0..=target.len() {
        matrix[0][j] = j;
    }
    
    
    for i in 1..=source.len() {
        for j in 1..=target.len() {
            
            let cost = if source.chars().nth(i - 1) == target.chars().nth(j - 1) {
                0
            } else {
                1
            };
    
            matrix[i][j] = std::cmp::min(
                matrix[i - 1][j] + 1,
                std::cmp::min(
                    matrix[i][j - 1] + 1,
                    matrix[i - 1][j - 1] + cost
                )
            );
        }
    }
    
    matrix[source.len()][target.len()]
}