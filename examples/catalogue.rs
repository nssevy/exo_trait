#![allow(unused)]
trait Descriptible {
    fn description(&self) -> String;
}

trait Achetable {
    fn prix(&self) -> u32;
}

struct Livre {
    titre: String,
    prix: u32,
}

struct Jouet {
    nom: String,
    prix: u32,
}

impl Descriptible for Livre {
    fn description(&self) -> String {
        self.titre.clone()
    }
}

impl Achetable for Livre {
    fn prix(&self) -> u32 {
        self.prix
    }
}

impl Descriptible for Jouet {
    fn description(&self) -> String {
        self.nom.clone()
    }
}

impl Achetable for Jouet {
    fn prix(&self) -> u32 {
        self.prix
    }
}

fn etiquette<T: Descriptible + Achetable>(article: &T) {
    println!("{} : {} euros", article.description(), article.prix());
}

fn main() {
    let livre = Livre {
        titre: String::from("Rust en action"),
        prix: 39,
    };
    let jouet = Jouet {
        nom: String::from("Robot"),
        prix: 25,
    };
    etiquette(&livre);
    etiquette(&jouet);
}
