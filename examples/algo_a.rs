//#![allow(unused)]
//Écris une fonction qui cherche une valeur dans une slice d'entiers et renvoie la position de la première occurrence, ou l'absence de résultat si la valeur n'y est pas.

/*fn get_position_value(slice: &[i32], value: i32) -> Option<usize> {
    slice.iter().position(|&x| x == value)
}*/

fn get_position_value(slice: &[i32], value: i32) -> Option<usize> {
    let mut i: usize = 0;

    while i < slice.len() {

        if slice[i] == value {
            return Some(i);
        }

        i += 1;

    }

    None

}

fn main() {
    let nombres: Vec<i32> = vec![4, 12, 7, 12, 3];
    let vide: Vec<i32> = vec![];

    let valeur_1: i32 = 7;
    let valeur_2: i32 = 12;
    let valeur_3: i32 = 99;

    println!("Recherche de {} : {:?}", &valeur_1, get_position_value(&nombres, valeur_1));
    println!("Recherche de {} : {:?}", &valeur_2, get_position_value(&nombres, valeur_2));
    println!("Recherche de {} : {:?}", &valeur_3, get_position_value(&nombres, valeur_3));

    println!("Recherche de {} : {:?}", &valeur_3, get_position_value(&vide, valeur_3));

}
