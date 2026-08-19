#![allow(unused)]

//Écris une fonction qui reçoit une slice d'entiers et une valeur, et renvoie le nombre de fois que cette valeur apparaît dans la slice.

fn occurence(tab: &[i32], valeur: i32) -> usize {
    let mut i: usize = 0;
    let mut apparition: usize = 0;

    while i < tab.len() {
        if tab[i] == valeur {
            apparition += 1;
        }
        i+= 1;
    }

    apparition
}

fn main() {
    let tab_1: [i32; 5] = [4, 12, 7, 12, 3];
    let tab_2: [i32; 0] = [];

    let v_1: i32 = 12;
    let v_2: i32 = 99;
    let v_3: i32 = 5;

    println!("Occurences de {} dans {:?} : {}", v_1, tab_1, occurence(&tab_1, v_1));
    println!("Occurences de {} dans {:?} : {}", v_2, tab_1, occurence(&tab_1, v_2));
    println!("Occurences de {} dans {:?} : {}", v_3, tab_2, occurence(&tab_2, v_3));
}
