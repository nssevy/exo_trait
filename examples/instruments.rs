#![allow(unused)]
trait Instrument {
    fn jouer(&self) -> String;
}

#[derive(Debug)]
struct Guitare {
    nombre_de_corde: u32,
}

#[derive(Debug)]
struct Piano {
    nombre_de_touche: u32,
}

impl Instrument for Guitare {
    fn jouer(&self) -> String {
        format!("La Guitare à {} cordes résonne", self.nombre_de_corde)
    }
}

impl Instrument for Piano {
    fn jouer(&self) -> String {
       format!("Le piano à {} touches retentit", self.nombre_de_touche) 
    }
}


fn main() {
    let guitare: Guitare = Guitare { nombre_de_corde: 6 };
    let piano: Piano = Piano { nombre_de_touche: 88 };

    println!("{}", guitare.jouer());
    println!("{}", piano.jouer());
}
