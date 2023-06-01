pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    if diagonals("O", &table) || horizontal("O", &table) || vertical("O", &table) {
        return String::from("player O won");
    } else if diagonals("X", &table) || horizontal("X", &table) || vertical("X", &table) {
        return String::from("player X won");
    } else {
        return String::from("tie");
    }
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let diagonal1: Vec<&str> = vec![table[0][0], table[1][1], table[2][2]];
    let diagonal2: Vec<&str> = vec![table[0][2], table[1][1], table[2][0]];

    diagonal1.iter().all(|&cell| cell == player) || diagonal2.iter().all(|&cell| cell == player)
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    for row in table {
        if row.iter().all(|&cell| cell == player) {
            return true;
        }
    }
    false
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    for col in 0..3 {
        if table[0][col] == player && table[1][col] == player && table[2][col] == player {
            return true;
        }
    }
    false
}
