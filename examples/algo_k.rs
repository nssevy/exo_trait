#![allow(unused)]

//Écris une fonction qui reçoit une slice d'entiers et renvoie un Vec<(i32, usize)> : chaque valeur suivie du nombre de fois qu'elle se répète consécutivement.

fn x_fois(tab: &[i32]) -> Vec<(i32, usize)> {
    let mut i: usize = 0;
    let mut tableau = Vec::new();
    let mut a: usize = 1;
    let mut tuple: (i32, usize) = (0,0);

    //let mut tuple: (i32, usize) = (0,0);

    while i+1 < tab.len() {
        let mut courant: i32 = tab[i+1];
        let mut suivant: i32 = tab[i+1];

        if courant == suivant {
            a += 1;
        } else {
            tableau.push(tuple);
            a = 0;
        }

        tuple = (courant, a);

        //tableau.push(tuple);


        dbg!(&tuple);

        i+= 1;
    }

    //tableau.push(tuple);

    tableau
}

fn main() {

    /*let mut tab = Vec::new();
    let tuple: (i32, i32) = (1, 2);

    tab.push((tuple));
    tab.push((5, 3));

    dbg!(&tab); */

    let tab1: Vec<i32> = vec![1, 1, 1, 2, 2, 3];
    println!("{:?} : {:?}", &tab1, x_fois(&tab1));

    /*let tab5: Vec<i32> = vec![];
    dbg!(x_fois(&tab5));*/
}

/*
[1, 1, 1, 2, 2, 3]  : [(1, 3), (2, 2), (3, 1)]
[1, 2, 1]           : [(1, 1), (2, 1), (1, 1)]
[4, 4, 4, 4]        : [(4, 4)]
[7]                 : [(7, 1)]
[]                  : []
*/
