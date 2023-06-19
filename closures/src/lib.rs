pub fn first_fifty_even_square() -> Vec<i32> {
    let vec_even=(2..)//initialiser le vecteur a partir de 2...
    .step_by(2)//Iter sur chaque elements de mon vecteur et prend les nombres paires (even)  2,4,6..
    .map(|x| x*x)//map avec une CLOSURE en paramètre qui prend x en parametre et renvoie x au carré
    .take(50)//methode qui limite mon vecteur au 50 premiers element de mon iterator
    .collect();// collect les élements dans mon vec_even
    vec_even
}