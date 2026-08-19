#![allow(unused)]

//Écris une fonction qui reçoit une slice d'entiers et renvoie la plus grande valeur qu'elle contient.

fn get_most_value(elements: &[i32]) -> Option<i32> {

    if elements.is_empty() {
        return None;
    }

    let mut i: usize = 0;
    let mut most: i32 = elements[i]; //4

    while i < elements.len() {
        if most <= elements[i] {
            most = elements[i];
        }
        i += 1;
    }

    Some(most)
}

fn main () {
    let tab_1: [i32; 5] = [4, 12, 7, 12, 3];
    let tab_2: [i32; 3] = [-8, -3, -50];
    let tab_3: [i32; 0] = [];

    println!("Max de {:?} : {:?}", tab_1,get_most_value(&tab_1));
    println!("Max de {:?} : {:?}", tab_2,get_most_value(&tab_2));
    println!("Max de {:?} : {:?}", tab_3,get_most_value(&tab_3));

    //dbg!(&tab_1);
}
