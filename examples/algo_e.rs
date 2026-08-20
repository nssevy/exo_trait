//Écris une fonction qui reçoit une slice d'entiers et détermine si elle contient au moins un doublon, c'est-à-dire une valeur qui apparaît plus d'une fois.

fn at_least_a_deplicate(tab: &[i32]) -> bool {

    let mut i: usize = 0;

    while i < tab.len() {

        let mut j: usize = i+1;

        while j < tab.len() {
            if tab[i] == tab[j] {
                return true
            }
            j += 1;
        }
        i += 1;
    }
    
    false
}

fn main() {
    let tab_1: [i32; 5] = [4, 12, 7, 12, 3];
    let tab_2: [i32; 4] = [22, 33, 44, 55];
    let tab_3: [i32; 0] = [];
    let tab_4: [i32; 1] = [5];
    let tab_5: [i32; 2] = [5, 5];

    println!("{:?} : {:?}", tab_1, at_least_a_deplicate(&tab_1));
    println!("{:?} : {:?}", tab_2, at_least_a_deplicate(&tab_2));
    println!("{:?} : {:?}", tab_3, at_least_a_deplicate(&tab_3));
    println!("{:?} : {:?}", tab_4, at_least_a_deplicate(&tab_4));
    println!("{:?} : {:?}", tab_5, at_least_a_deplicate(&tab_5));
}

/*

[4, 12, 7, 12, 3]  : true
[4, 12, 7, 3]      : false
[]                 : false
[5]                : false
[5, 5]             : true

*/
