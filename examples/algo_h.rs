#![allow(unused)]

//Écris une fonction qui reçoit une slice d'entiers et renvoie la plus petite et la plus grande valeur qu'elle contient, en un seul parcours.

fn min_and_max(tab: &[i32]) -> Option<(i32, i32)> {

    if tab.is_empty() {
        return None;
    }

    let mut i: usize = 0;
    let mut small: i32 = tab[0];
    let mut big: i32 = tab[0];

    while i < tab.len() {

        if small > tab[i] { small = tab[i];}

        if big < tab[i] { big = tab[i];}

        i += 1;
    }

    Some((small, big))

}

fn main() {

    let tab: Vec<i32> = vec![4, 12, 7, 12, 3];
    let tab_negatif: Vec<i32> = vec![-8, -3, -50];
    let doublon: Vec<i32> = vec![7];
    let tab_vide: Vec<i32> = vec![];

    println!("{:?}", min_and_max(&tab));
    println!("{:?}", min_and_max(&tab_negatif));
    println!("{:?}", min_and_max(&doublon));
    println!("{:?}", min_and_max(&tab_vide));

}

/*
[4, 12, 7, 12, 3]  : Some((3, 12))
[-8, -3, -50]      : Some((-50, -3))
[7]                : Some((7, 7))
[]                 : None*/
