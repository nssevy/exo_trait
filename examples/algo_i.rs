//#![allow(unused)]
//Écris une fonction qui reçoit une slice d'entiers et détermine si elle est triée par ordre croissant, au sens large deux valeurs égales côte à côte ne cassent pas l'ordre.

fn is_sorted(tab: &[i32]) -> bool {

    let mut i: usize = 0;

    while i + 1 < tab.len() {
        let courant: i32 = tab[i];
        let suivant: i32 = tab[i+1];

        if courant > suivant {
            return false;
        }

        //println!("courant : {}, suivant : {}", courant, suivant);
        i+= 1;
    }
    true
}

fn main() {
    let tab: Vec<i32> = vec![1, 2, 3, 4, 5];
    dbg!(&is_croissant(&tab));

    let tab_1: Vec<i32> = vec![1, 2, 2, 3];
    dbg!(&is_croissant(&tab_1));

    let tab_2: Vec<i32> = vec![1, 3, 2];
    dbg!(&is_croissant(&tab_2));

    let tab_3: Vec<i32> = vec![5, 4, 3];
    dbg!(&is_croissant(&tab_3));


    let tab_4: Vec<i32> = vec![7];
    dbg!(&is_croissant(&tab_4));

    let tab_5: Vec<i32> = vec![];
    dbg!(&is_croissant(&tab_5));
}

/*
[1, 2, 3, 4, 5]  : true
[1, 2, 2, 3]     : true
[1, 3, 2]        : false
[5, 4, 3]        : false
[7]              : true
[]               : true
*/
