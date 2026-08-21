#![allow(unused)]
//Écris une fonction qui reçoit une slice d'entiers et renvoie la deuxième plus grande valeur distincte qu'elle contient.

fn second_max(tab: &[i32]) -> i32 {
    let mut i: usize = 1;
    let mut premier = tab[0];
    let mut second = tab[1];


    while i < tab.len() {

        let x = tab[i];

        if x > premier {
            second = premier;
            premier = x;
        } else if x < premier && x > second { 
            second = x; 
        }

        println!("tour {} | lu={} | grand={} | second={}", i, tab[i], premier, second);

        i+= 1;
    }

    second
}

fn main() {

    let tab: Vec<i32> = vec![4, 12, 7, 12, 3];
    let tab2: Vec<i32> = vec![1, 2, 3, 4, 5];
    let tab3: Vec<i32> = vec![5, 4, 3,];

    dbg!(&second_max(&tab));
    dbg!(&second_max(&tab2));
    dbg!(&second_max(&tab3));
}

/*
[4, 12, 7, 12, 3]  : Some(7)
[1, 2, 3, 4, 5]    : Some(4)
[5, 4, 3, 2, 1]    : Some(4)
[7, 7, 7]          : None
[7]                : None
[]                 : None
*/
