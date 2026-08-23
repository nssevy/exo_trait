//#![allow(unused)]

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

impl Reponse {

    fn ok(utilisateur: Utilisateur) -> Self {
        Reponse::Ok(utilisateur)
    }

    fn redirect(url: String) -> Self {
        Reponse::Redirection { url, permanente: false }
    }

    #[allow(unused)] // pas utiliser
    fn permanent_redirect(url: String) -> Self {
        Reponse::Redirection { url, permanente: true }
    }

    fn erreur(code: u32, message: String) -> Self {
        Reponse::Erreur { code, message }
    }

    fn timeout() -> Self {
        Reponse::Timeout
    }

    fn traiter(&self) {
        match self {
            Reponse::Ok(Utilisateur {id, pseudo, actif: true}) => { println!("200 - Utilisateur {} ({})", id, pseudo)},
            Reponse::Ok(Utilisateur {pseudo, actif: false, ..}) => { println!("403 - Compte désactivé : {}", pseudo)},
            Reponse::Redirection {url, permanente: false} => { println!("302 - Redirection vers {}", url)},
            Reponse::Erreur {code, message} => { println!("{} - {}", code, message)},
            Reponse::Timeout => println!("504 - Aucune reponse du serveur"),
            _ => unreachable!(),
        }
    }

}

fn main() {

    // Initialisation des users
    let yves: Utilisateur = Utilisateur::new(7, "Yves".into(), true);
    let bernie: Utilisateur = Utilisateur::new(12, "Bernie".into(), false);

    //Initialisation des reponses
    let yves_ok = Reponse::ok(yves);
    let bernie_ok = Reponse::ok(bernie);
    let redirection = Reponse::redirect("/connexion".into());
    let erreur = Reponse::erreur(404, "Page introuvable".into());
    let reponse_api = Reponse::timeout();

    //Appels de la fonction taiter
    yves_ok.traiter();
    bernie_ok.traiter();
    redirection.traiter();
    erreur.traiter();
    reponse_api.traiter();

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
