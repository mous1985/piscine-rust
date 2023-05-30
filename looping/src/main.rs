use std::io;

fn main() {
    let mut tries = 0;
    let answer = "The letter e";

    println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Erreur lors de la lecture de l'entrée.");

        let guess = guess.trim();

        tries += 1;

        if guess == answer {
            println!("Bonne réponse !");
            break;
        } else {
            println!("mauvaise réponse");
        }
    }

    println!("Nombre d'essais nécessaires : {}", tries);
}
