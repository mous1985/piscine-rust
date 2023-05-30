use std::io;

fn main() {
    let mut tries = 0;
    let answer = "e";

    println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?
I don't know");

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Erreur read input");

        let guess = guess.trim();

        tries += 1;

        if guess == answer {
            println!("Bonne réponse !");
            break;
        } else {
            println!("Mauvaise réponse. Essayez encore.");
        }
    }

    println!("Number of trials: {}", tries);
}
