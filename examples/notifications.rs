#![allow(unused)]

trait Notifiable {
    fn message(&self) -> String;
    fn nom(&self) -> String;
}
#[derive(Debug)]
struct Email {
    nom: String,
    adresse: String,
}
#[derive(Debug)]
struct Sms {
    nom: String,
    numero: u32,
}

impl Notifiable for Email {
    fn message(&self) -> String {
        self.adresse.clone()
    }
    fn nom(&self) -> String {
        self.nom.clone()
    }
}

impl Notifiable for Sms {
    fn message(&self) -> String {
        //@self.numero as String;
        self.numero.to_string()
    }

    fn nom(&self) -> String {
        self.nom.clone()
    }
}

fn envoyer <T: Notifiable> (notifiable: &T) {
    println!("Envoi : {} vers {}",notifiable.nom(), notifiable.message())
}

fn main () {
    
    let email: Email = Email{nom: String::from("Mail"), adresse: String::from("alice@exemple.fr")};
    envoyer(&email);

    let message: Sms = Sms {nom: String::from("SMS"), numero: 0600000000 };
    envoyer(&message);

    //dbg!(&message.message());
}
