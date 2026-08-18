#![allow(unused)]

trait Forme {
    fn aire(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    largeur: f64,
    hauteur: f64,
}

impl Forme for Rectangle {
    fn aire(&self) -> f64 {
        self.largeur * self.hauteur
    }
}

impl Rectangle {
    //Constructeur
    fn new(l: f64, h: f64) -> Rectangle {
        Rectangle {
            largeur: l,
            hauteur: h,
        }
    }
}

fn aire_totale(formes: &[Rectangle]) -> f64 {
    let mut total: f64 = 0.0;

    for forme in formes {
        total += forme.aire();
    }

    total
}

fn main() {
    let mut rectangle_un: Rectangle = Rectangle::new(6.0, 3.0);
    let mut rectangle_deux: Rectangle = Rectangle::new(10.0, 5.0);
    let mut rectangle_trois: Rectangle = Rectangle::new(14.0, 6.0);

    let tab_de_rectangle: Vec<Rectangle> = vec![rectangle_un, rectangle_deux, rectangle_trois];

    let result = aire_totale(&tab_de_rectangle);

    dbg!(result);
}
