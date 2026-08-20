//Écris une fonction qui reçoit une slice d'entiers et détermine si elle se lit à l'identique dans les deux sens.

fn is_palindrome(tab: &[i32]) -> bool {

    let mut i: usize = 0;

    while i < tab.len() / 2 {
        if tab[i] != tab[tab.len()-(i+1)] {
            return false
        }
        i+= 1;
    }

    true
}

fn main() {
    let tab_1: [i32; 5] = [1, 2, 3, 2, 1];
    let tab_2: [i32; 4] = [1, 2, 2, 1];
    let tab_3: [i32; 3] = [1, 2, 3];
    let tab_4: [i32; 1] = [7];
    let tab_5: [i32; 0] = [];
    let tab_6: [i32; 4] = [1, 2, 9, 1];

    println!("{:?} : {}",tab_1, is_palindrome(&tab_1));
    println!("{:?} : {}",tab_2, is_palindrome(&tab_2));
    println!("{:?} : {}",tab_3, is_palindrome(&tab_3));
    println!("{:?} : {}",tab_4, is_palindrome(&tab_4));
    println!("{:?} : {}",tab_5, is_palindrome(&tab_5));
    println!("{:?} : {}",tab_6, is_palindrome(&tab_6));

}

/*
[1, 2, 3, 2, 1]  : true
[1, 2, 2, 1]     : true
[1, 2, 3]        : false
[7]              : true
[]               : true

*/
