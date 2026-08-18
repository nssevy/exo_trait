#![allow(unused)]

trait Machine {
    fn numero_serie(&self) -> u32;
}

trait Vehicule: Machine {
    fn modele(&self) -> String;
    fn presentation(&self) -> String {
        format!(
            "{} (numero de serie : {})",
            self.modele(),
            self.numero_serie()
        )
    }
}
#[derive(Debug)]
struct Camion {
    modele: String,
    numero_serie: u32,
}

impl Machine for Camion {
    fn numero_serie(&self) -> u32 {
        self.numero_serie
    }
}

impl Vehicule for Camion {
    fn modele(&self) -> String {
        self.modele.clone()
    }
}

fn main() {
    let camion = Camion {
        modele: String::from("Volvo FH"),
        numero_serie: 4021,
    };
    println!("{}", camion.presentation());
    println!("{:?}", camion);
}
