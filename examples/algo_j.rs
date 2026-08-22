//#![allow(unused)]
//Écris une fonction qui reçoit une slice d'entiers et renvoie la longueur de la plus longue suite d'éléments consécutifs strictement croissants.

fn verif_consecutif(tab: &[i32]) -> usize {
    
    if tab.is_empty() {
        return 0;
    }

    let mut i: usize = 0;
    let mut grand: usize = 0;
    let mut valeur: usize = 1;

    while i+1 < tab.len() {
        let courant: i32 = tab[i];
        let suivant: i32 = tab[i+1];

        if courant < suivant {
            valeur += 1;
        } else {
            valeur = 1;
        }

         if valeur > grand {
            grand = valeur;
        }

        i+= 1;
    }

    grand
}

fn main() {

    let tab_1: Vec<i32> = vec![1, 2, 5, 3, 4, 8, 9]; // 4
    dbg!(&verif_consecutif(&tab_1));

    let tab_2: Vec<i32> = vec![1, 2, 3, 4, 5]; // 5
    dbg!(&verif_consecutif(&tab_2));

    let tab_3: Vec<i32> = vec![5, 4, 3, 2, 1]; // 1
    dbg!(&verif_consecutif(&tab_3));

    let tab_4: Vec<i32> = vec![1, 3, 2, 4]; // 2
    dbg!(&verif_consecutif(&tab_4));

    let tab_5: Vec<i32> = vec![7];
    dbg!(&verif_consecutif(&tab_5)); // 1

    let tab_6: Vec<i32> = vec![];
    dbg!(&verif_consecutif(&tab_6)); // 0

    let tab_7: Vec<i32> = vec![1, 2, 0, 1, 2, 3]; //4
    dbg!(&verif_consecutif(&tab_7));

    let tab_8: Vec<i32> = vec![12, 13, 14, 0, 1, 0]; //3
    dbg!(&verif_consecutif(&tab_8));

    let tab_9: Vec<i32> = vec![4, 66, 356, 2, 0, 1, 2, 3, 6]; // 5
    dbg!(&verif_consecutif(&tab_9));

}

/*
[1, 2, 5, 3, 4, 8, 9]  : 4
[1, 2, 3, 4, 5]        : 5
[5, 4, 3, 2, 1]        : 1
[1, 3, 2, 4]           : 2
[7]                    : 1
[]                     : 0
*/
