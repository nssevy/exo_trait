#![allow(unused)]
/*Écris une fonction qui reçoit deux slices d'entiers, chacune déjà triée par ordre croissant,
et renvoie un Vec<i32> contenant tous leurs éléments, trié lui aussi.

Contrainte : un seul parcours, et sans jamais trier.
Tu exploites le fait que les deux entrées sont déjà ordonnées.*/

fn tab_croissant(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut tableau = Vec::new();
    let mut i: usize = 0;

    while i < a.len() {
        if a[i] < b[i] {
            tableau.push(a[i]);
            tableau.push(b[i]);
        } else {
            tableau.push(b[i]);
            tableau.push(a[i]);
        }
        dbg!(&i);
        i+= 1
    }

    tableau
}

fn main() {
    /*let tab1: Vec<i32> = vec![1, 4, 9];
    let tab2: Vec<i32> = vec![2, 3, 10];*/

    let tab3: Vec<i32> = vec![1, 2, 3];
    let tab4: Vec<i32> = vec![4, 5, 6];

    //println!("{:?}", tab_croissant(&tab1, &tab2));
    println!("{:?}", tab_croissant(&tab3, &tab4));
}

/*
[1, 4, 9]  +  [2, 3, 10]  : [1, 2, 3, 4, 9, 10]
[1, 2, 3]  +  [4, 5, 6]   : [1, 2, 3, 4, 5, 6]
[5]        +  []          : [5]
[]         +  []          : []
[1, 3]     +  [1, 2]      : [1, 1, 2, 3]
*/
