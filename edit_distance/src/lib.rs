pub fn edit_distance(source: &str, target: &str) -> usize {
    let n = source.chars().count();
    let m = target.chars().count();

    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 0..=n {
        dp[i][0] = i;
    }

    for j in 0..=m {
        dp[0][j] = j;
    }

    for (i, source_char) in source.chars().enumerate() {
        for (j, target_char) in target.chars().enumerate() {
            let cost = if source_char == target_char { 0 } else { 1 };
            dp[i + 1][j + 1] = dp[i][j + 1].min(dp[i + 1][j]).min(dp[i][j] + cost);
        }
    }

    dp[n][m]
}
