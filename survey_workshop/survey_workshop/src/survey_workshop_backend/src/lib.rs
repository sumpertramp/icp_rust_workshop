mod core;

use std::io::{self, Write};

fn main() {
    println!("Akıllı Anket Uygulamasına Hoş Geldiniz!");

    let mut survey = core::Survey::new();

    loop {
        println!("\nLütfen yapmak istediğiniz işlemi seçin:");
        println!("1. Anket Sorusu Ekle");
        println!("2. Anketi Göster");
        println!("3. Anketi Kaydet ve Çık");
        println!("4. Çıkış");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Okuma hatası!");

        match choice.trim().parse::<u32>() {
            Ok(1) => {
                println!("Lütfen yeni anket sorusunu girin:");
                let mut new_question = String::new();
                io::stdin().read_line(&mut new_question).expect("Okuma hatası!");
                survey.add_question(new_question.trim().to_string());
            }
            Ok(2) => {
                println!("Anket Soruları:");
                for (i, question) in survey.questions.iter().enumerate() {
                    println!("{}. {}", i + 1, question);
                }
            }
            Ok(3) => {
                survey.save();
                break;
            }
            Ok(4) => {
                break;
            }
            _ => {
                println!("Geçersiz seçim!");
            }
        }
    }

    println!("Çıkış yapılıyor...");
}