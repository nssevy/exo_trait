#![allow(unused)]
trait Abonnement {
    fn nom(&self) -> String;
    fn prix_mensuel(&self) -> u32 {
        10
    }
}

#[derive(Debug)]
struct Gratuit {
    nom_client: String,
}

struct Prenium {
    nom_client: String,
}

impl Abonnement for Gratuit {
    fn nom(&self) -> String {
        self.nom_client.clone()
    }
    fn prix_mensuel(&self) -> u32 {
        0
    }
}

impl Abonnement for Prenium {
    fn nom(&self) -> String {
        self.nom_client.clone()
    }
}

fn afficher_facture<T: Abonnement>(a: &T) {
    println!("{} : {} euros/mois", a.nom(), a.prix_mensuel());
}

fn main() {
    let alice: Gratuit = Gratuit {
        nom_client: String::from("Alice"),
    };
    afficher_facture(&alice);

    let bob: Prenium = Prenium {
        nom_client: String::from("Bob"),
    };
    afficher_facture(&bob);
}
