#![allow(unused)]
trait Boisson {
    fn servir(&self) -> String;
}

struct Cafe {
    //name: String,
    sucre: u8,
}

struct The {
    //name: String,
    temps_infusion: u32, // Temps d'infusion en minutes
}

impl Boisson for Cafe {
    fn servir(&self) -> String {
        if self.sucre > 1 {
            format!("Café avec {} sucres", self.sucre)
        } else {
            format!("Café avec {} sucre", self.sucre)
        }
    }
}

impl Boisson for The {
    fn servir(&self) -> String {
        format!("Thé infusé {} min", self.temps_infusion)
    }
}

fn prepare_cafe(a: u8) -> impl Boisson {
    Cafe { sucre: a }
}

fn prepare_the(a: u32) -> impl Boisson {
    The { temps_infusion: a }
}

fn main() {
    let cafe = prepare_cafe(2);
    let the = prepare_the(3);

    println!("{}", cafe.servir());
    println!("{}", the.servir());
}
