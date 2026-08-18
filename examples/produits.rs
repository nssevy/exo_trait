#![allow(unused)]

trait Nomme {
    fn nom(&self) -> String;
}

trait Taxable {
    fn prix_ht(&self) -> u32; //en euros
}

#[derive(Debug)]
struct Ordinateur {
    nom: String,
    prix: u32,
}

#[derive(Debug)]
struct Livre {
    nom: String,
    prix: u32,
}

impl Nomme for Ordinateur {
    fn nom(&self) -> String {
        self.nom.clone()
    }
}

impl Nomme for Livre {
    fn nom(&self) -> String {
        self.nom.clone()
    }
}

impl Taxable for Ordinateur {
    fn prix_ht(&self) -> u32 {
        self.prix
    }
}

impl Taxable for Livre {
    fn prix_ht(&self) -> u32 {
        self.prix
    }
}

fn ticket<T: Nomme + Taxable>(article: &T) {
    println!("{} : {} euros HT", article.nom(), article.prix_ht());
}

fn main() {
    let ordi: Ordinateur = Ordinateur {
        nom: String::from("Ordinateur portable"),
        prix: 800,
    };
    let livre: Livre = Livre {
        nom: String::from("Rust en action"),
        prix: 39,
    };

    ticket(&ordi);
    ticket(&livre);
}
