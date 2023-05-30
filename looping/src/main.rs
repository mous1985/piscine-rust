use std::io::{self, Write};

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "The letter e";
    let mut tries = 0;

    loop {
        println!("{}", riddle);
        io::stdout().flush().expect("Failed to flush stdout.");

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line.");

        tries += 1;

        if user_input.trim() == answer {
            break;
        }
    }

    println!("Number of trials: {}", tries);
}
