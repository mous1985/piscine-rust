use std::io;

fn main() {
    let mut tries = 0;
    let answer = "e";

    println!("Riddle: I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input.");

        let guess = guess.trim();

        tries += 1;

        if guess == answer {
            println!("Correct answer!");
            break;
        } else {
            println!("Wrong answer. Try again.");
        }
    }

    println!("Number of tries needed: {}", tries);
}
