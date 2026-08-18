//#[allow(dead_code)]
trait Vehicule {
    fn nom(&self) -> String;
    fn vitesse(&self) -> u32;
}

#[derive(Debug)]
struct Voiture {
    nom: String,
    vitesse: u32,
}

impl Voiture {
    fn new(n: String, v: u32) -> Voiture {
        Voiture { nom: n, vitesse: v }
    }
}

impl Vehicule for Voiture {
    fn nom(&self) -> String {
        self.nom.clone()
    }

    fn vitesse(&self) -> u32 {
        self.vitesse
    }
}

fn plus_rapide(voitures: &Vec<Voiture>) -> Option<String> {
    if voitures.is_empty() {
        return None;
    } else {
        let mut comparant = voitures[0].vitesse();
        let mut vr: &Voiture = &voitures[0]; //vr pour voiture rapide

        for voiture in voitures {
            if voiture.vitesse() > comparant {
                vr = voiture;
                comparant = voiture.vitesse();
            }
        }
        Some(vr.nom())
    }
}

fn print_vehicule(v: &Option<String>) {
    match v {
        Some(valeur) => println!("Le plus rapide : {}", valeur),
        None => println!("Aucun véhicule"),
    }
}



fn main() {
    //Les voitures initialiser
    let voiture_n1: Voiture = Voiture::new(String::from("Bolide"), 23_000);
    let voiture_n2: Voiture = Voiture::new(String::from("Berline"), 10_000);
    let voiture_n3: Voiture = Voiture::new(String::from("Citadine"), 5_000);

    //Les tableaux, un remplis de voiture, un autre vide.
    let tab_de_voiture: Vec<Voiture> = vec![voiture_n1, voiture_n2, voiture_n3];
    let tab_vide: Vec<Voiture> = vec![];

    //Recupere la voiture la plus rapide du vec.
    let pr = plus_rapide(&tab_de_voiture);
    let v = plus_rapide(&tab_vide);

    //Affiche le resultat.
    print_vehicule(&pr);
    print_vehicule(&v);
}
