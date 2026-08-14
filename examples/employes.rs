#![allow(unused)]
trait Employe {
    fn nom(&self) -> String;
    fn salaire(&self) -> u32 {
        2000
    }
}

struct Stagiaire {
    nom: String,
}

struct Manager {
    nom: String,
}

impl Employe for Stagiaire {
    fn nom(&self) -> String {
        self.nom.clone()
    }
}

impl Employe for Manager {
    fn nom(&self) -> String {
        self.nom.clone()
    }

    fn salaire(&self) -> u32 {
        4500
    }
}

fn affiche_paie<T: Employe>(employe: &T) {
    println!("{} : {} euros", employe.nom(), employe.salaire())
}

fn main() {
    let alice = Stagiaire {nom: String::from("Alice")};
    let bob = Manager {nom: String::from("Bob") };

    affiche_paie(&alice);
    affiche_paie(&bob);
}
