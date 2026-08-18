//#![allow(unused)]
struct Voiture {
    nom: String,
    vitesse_maximale: u32,
}

struct Moto {
    cylindree: String,
    nom: String,
    vitesse_maximale: u32,
}

trait InfoVehicule {
    fn get_nom(&self) -> String;
    fn get_max_vitesse(&self) -> u32;
    fn fiche(&self) {
        println!("{} : {} km/h", self.get_nom(), self.get_max_vitesse());
    }
}

impl InfoVehicule for Voiture {
    fn get_nom(&self) -> String {
        self.nom.clone()
    }
    fn get_max_vitesse(&self) -> u32 {
        self.vitesse_maximale
    }
}

impl InfoVehicule for Moto {
    fn get_nom(&self) -> String {
        format!("{} {}", self.nom, self.cylindree)
    }
    fn get_max_vitesse(&self) -> u32 {
        self.vitesse_maximale
    }
}

fn main() {
    let renault: Voiture = Voiture {
        nom: String::from("Twingo"),
        vitesse_maximale: 130,
    };

    let moto: Moto = Moto {
        cylindree: String::from("600cc"),
        nom: String::from("Moto"),
        vitesse_maximale: 180,
    };

    renault.fiche();
    moto.fiche();
}
