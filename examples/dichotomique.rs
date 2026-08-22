#![allow(unused)]

fn dichotomique(tableau: &[i32], valeur: i32) -> i32 {
    let mut i: usize = 0;
    let mut debut: usize = 0;
    let mut fin: usize = tableau.len()-1;
    let mut position: i32 = 0;
    
    let mut milieu: usize = tableau.len() / 2 - 1; // Pour l'indice
    
    while fin >= tableau.len() {
        if tableau[milieu] == valeur {
            position = tableau[milieu];
            return position
        } else if tableau[milieu] < valeur {
            fin = milieu;
        } else {
            debut = milieu;
        }
        dbg!(&fin);
    }
    
    position

}

fn main() {
    let tab: Vec<i32> = vec![5, 7, 12, 14, 23, 27, 35, 40, 41, 45]; // 10 éléments
    let value: i32 = 35; 
    println!("Dans {:?}, {} apparait à la {} ème position", &tab, &value, &dichotomique(&tab, value));
}
