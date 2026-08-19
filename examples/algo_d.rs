#[allow(unused)]

//Écris une fonction qui reçoit une slice d'entiers et renvoie la somme des nombres pairs qu'elle contient.

fn somme_pairs(tab: &[i32]) -> i32 {
    let mut i: usize = 0;
    let mut nbr_pairs: i32 = 0;

    while i < tab.len() {
        if tab[i] % 2 == 0 {
            nbr_pairs += tab[i];
        }

        i += 1;
    }
    nbr_pairs
}

fn main() {
    let tab_1: [i32; 5] = [4, 12, 7, 12, 3];
    let tab_2: [i32; 3] = [1, 3, 5];
    let tab_3: [i32; 0] = [];
    let tab_4: [i32; 3] = [-4, -3, -8];

    println!("Somme des pairs de {:?} : {}", tab_1, somme_pairs(&tab_1));
    println!("Somme des pairs de {:?} : {}", tab_2, somme_pairs(&tab_2));
    println!("Somme des pairs de {:?} : {}", tab_3, somme_pairs(&tab_3));
    println!("Somme des pairs de {:?} : {}", tab_4, somme_pairs(&tab_4));
}
