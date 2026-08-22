#![allow(unused)]

//Modéliser une réponse d'une API.
#[derive(Debug)]
struct Utilisateur {
    id: u32,
    pseudo: String,
    actif: bool
}

impl Utilisateur {
    fn new(id: u32, pseudo: String, actif: bool) -> Utilisateur {
        Utilisateur { id, pseudo, actif }
    }
}

enum Reponse {
    Ok(Utilisateur),
    Redirection {url: String, permanente: bool},
    Erreur {code: u32, message: String},
    Timeout,
}

fn traiter(reponse: &Reponse) {

    match reponse {
        Reponse::Ok(Utilisateur {id, pseudo, actif: true}) => { println!("200 - Utilisateur {} ({})", pseudo, id)},
        Reponse::Ok(Utilisateur {id, pseudo, actif: false}) => { println!("403 - Compte désactivé : {}", pseudo)},
        Reponse::Redirection {url, permanente: false} => { println!("302 - Redirection vers {}", url)},
        Reponse::Erreur {code, message} => { println!("{} - {}", code, message)},
        Reponse::Timeout => println!("504 - Aucune reponse du serveur"),
        _ => println!("courage"),
    }
}

fn main() {

    let utilisateur: Utilisateur = Utilisateur::new(7, "Yves".into(), true);
    traiter(&Reponse::Ok(utilisateur));

    let utilisateur2: Utilisateur = Utilisateur::new(12, "Bernie".into(), false);
    traiter(&Reponse::Ok(utilisateur2));

    traiter(&Reponse::Redirection{url: "/connexion".into(), permanente: false });
    traiter(&Reponse::Erreur {code: 404, message: "Page introuvable".into() });
    traiter(&Reponse::Timeout);

}

/*
Dans main, construis cinq réponses et appelle traiter sur chacune :

un Ok avec l'utilisateur 7 / sevy / actif
un Ok avec l'utilisateur 12 / bernie / inactif
une Redirection vers /connexion, non permanente
une Erreur 404 / Page introuvable
un Timeout

200 - utilisateur 7 (sevy)
403 - compte desactive : bernie
302 - redirection vers /connexion
404 - Page introuvable
504 - aucune reponse du serveur

*/
