#![allow(unused)]

trait Nommable {
    fn nom(&self) -> String;
}

trait Forme: Nommable {
    fn aire(&self) -> f64;

    fn resume(&self) {
        println!("{} a une aire de {}", self.nom(), self.aire());
    }
}

struct Cercle {
    nom: String,
    rayon: f64,
}

impl Nommable for Cercle {
    fn nom(&self) -> String {
        self.nom.clone()
    }
}

impl Forme for Cercle {
    fn aire(&self) -> f64 {
        std::f64::consts::PI * self.rayon * self.rayon
    }
}

fn main() {
    let cercle: Cercle = Cercle {
        nom: String::from("C1"),
        rayon: 2.0,
    };
    cercle.resume();
}
