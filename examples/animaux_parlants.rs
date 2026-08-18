#![allow(unused)]

trait Animal {
    fn cri(&self) -> String;

    fn parler(&self) {
        println!("Je fais : {}", self.cri())
    }
}

#[derive(Debug)]
struct Chien {
    nom: String,
}

#[derive(Debug)]
struct Chat {
    nom: String,
}

impl Animal for Chien {
    fn cri(&self) -> String {
        self.nom.clone()
    }
}

impl Animal for Chat {
    fn cri(&self) -> String {
        self.nom.clone()
    }
}

fn main() {
    let chien: Chien = Chien {
        nom: String::from("Wouf"),
    };
    chien.parler();

    let chat: Chat = Chat {
        nom: String::from("Miaou"),
    };
    chat.parler();
}
