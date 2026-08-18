//#![allow(unused)]

trait EtreVivant {
    fn nom(&self) -> String;
}

trait Creature: EtreVivant {
    fn points_de_vie(&self) -> u32;

    fn etat(&self) -> String {
        format!("{} a {} PV", self.nom(), self.points_de_vie())
    }
}

#[derive(Debug)]
struct Dragon {
    nom: String,
    points_de_vie: u32,
}

#[derive(Debug)]
struct Gobelin {
    nom: String,
    points_de_vie: u32,
}

impl EtreVivant for Dragon {
    fn nom(&self) -> String {
        self.nom.clone()
    }
}

impl Creature for Dragon {
    fn points_de_vie(&self) -> u32 {
        self.points_de_vie
    }

    fn etat(&self) -> String {
        format!(
            "{} a {} PV et crache du feu",
            self.nom(),
            self.points_de_vie()
        )
    }
}

impl EtreVivant for Gobelin {
    fn nom(&self) -> String {
        self.nom.clone()
    }
}

impl Creature for Gobelin {
    fn points_de_vie(&self) -> u32 {
        self.points_de_vie
    }
}

fn inspecter<T: Creature>(c: &T) {
    println!("{}", c.etat());
}

fn main() {
    let dragon: Dragon = Dragon {
        nom: String::from("Smaug"),
        points_de_vie: 500,
    };
    let gobelin: Gobelin = Gobelin {
        nom: String::from("Grok"),
        points_de_vie: 30,
    };

    inspecter(&dragon);
    inspecter(&gobelin);

    println!("{:?}", dragon);
}
