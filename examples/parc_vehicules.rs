#![allow(unused)]
trait Vehicule {
    fn nom(&self) -> String;
    fn vitesse_max(&self) -> u32;
}

struct Voiture {
    modele: String,
    vitesse_max: u32,
}

struct Avion {
    vol: String,
    vitesse_max: u32,
}

impl Vehicule for Voiture {
    fn nom(&self) -> String {
        self.modele.clone()
    }
    fn vitesse_max(&self) -> u32 {
        self.vitesse_max
    }
}

impl Vehicule for Avion {
    fn nom(&self) -> String {
        self.vol.clone()
    }
    fn vitesse_max(&self) -> u32 {
        self.vitesse_max
    }
}

fn annoncer<T: Vehicule>(vehicule: &T) {
    println!("Vehicule : {}, vitesse max : {:?} km/h", vehicule.nom(), vehicule.vitesse_max());
}

fn main() {
    let voiture: Voiture = Voiture {modele: String::from("Clio"), vitesse_max: 180 };
    let avion: Avion =  Avion {vol: String::from("Vol AF447"), vitesse_max: 900 };

    annoncer(&voiture);
    annoncer(&avion)
}

/*
Vehicule : Clio, vitesse max : 180 km/h
Vehicule : Vol AF447, vitesse max : 900 km/h
*/
