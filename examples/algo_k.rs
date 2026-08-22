//#![allow(unused)]

//Écris une fonction qui reçoit une slice d'entiers et renvoie un Vec<(i32, usize)> : chaque valeur suivie du nombre de fois qu'elle se répète consécutivement.

fn x_fois(tab: &[i32]) -> Vec<(i32, usize)> {
    let mut i: usize = 0;
    let mut tableau = Vec::new();

    if tab.is_empty() { // si le tableau est vide, renvoie un tableau vide.
        return tableau;
    }

    let mut count: usize = 1;
    let mut tuple: (i32, usize) = (tab[0],1); // tab[0], est pour le cas ou il y a un qu'un seul
    // dernier élément dans le tableau, le if sera instant faut donc pour push l'éléemnt courant.

    while i+1 < tab.len() {
        let courant: i32 = tab[i];
        let suivant: i32 = tab[i+1];

        if courant == suivant {
            count += 1;
        } else {
            tableau.push(tuple);
            count = 1;
        }

        tuple = (suivant, count);

        i+= 1;
    }

    tableau.push(tuple);

    tableau
}

fn main() {

    let tab1: Vec<i32> = vec![1, 1, 1, 2, 2, 3];
    println!("{:?} : {:?}", &tab1, x_fois(&tab1));

    let tab2: Vec<i32> = vec![1, 2, 1];
    println!("{:?} : {:?}", &tab2, x_fois(&tab2));

    let tab3: Vec<i32> = vec![4, 4, 4, 4];
    println!("{:?} : {:?}", &tab3, x_fois(&tab3));

    let tab4: Vec<i32> = vec![7];
    println!("{:?} : {:?}", &tab4, x_fois(&tab4));

    let tab5: Vec<i32> = vec![];
    println!("{:?} : {:?}", &tab5, x_fois(&tab5));

}

/*
[1, 1, 1, 2, 2, 3]  : [(1, 3), (2, 2), (3, 1)]
[1, 2, 1]           : [(1, 1), (2, 1), (1, 1)]
[4, 4, 4, 4]        : [(4, 4)]
[7]                 : [(7, 1)]
[]                  : []
*/
