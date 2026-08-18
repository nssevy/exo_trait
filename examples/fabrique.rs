trait Dessinable {
    fn dessiner(&self) -> String;
}

#[derive(Debug)]
struct Carre {
    coter: u32,
}

#[derive(Debug)]
struct Triangle {
    base: u32,
    hauteur: u32,
}

impl Dessinable for Carre {
    fn dessiner(&self) -> String {
        format!("Carre de côté {}", self.coter)
    }
}

impl Dessinable for Triangle {
    fn dessiner(&self) -> String {
        format!("Triangle {}x{}", self.base, self.hauteur)
    }
}

fn fabrique_carrer(coter: u32) -> impl Dessinable {
    Carre { coter }
}

fn fabrique_triangle(base: u32, hauteur: u32) -> impl Dessinable {
    Triangle { base, hauteur }
}

fn main() {
    let carre = fabrique_carrer(4);
    println!("{}", carre.dessiner());

    let triangle = fabrique_triangle(3, 6);
    println!("{}", triangle.dessiner());
}
