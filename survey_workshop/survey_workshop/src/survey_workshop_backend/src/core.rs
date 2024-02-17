use std::fs::File;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Survey {
    pub questions: Vec<String>,
}

impl Survey {
    pub fn new() -> Survey {
        Survey { questions: Vec::new() }
    }

    pub fn add_question(&mut self, question: String) {
        self.questions.push(question);
    }

    pub fn save(&self) {
        let json = serde_json::to_string(&self).expect("JSON dönüşüm hatası!");
        let mut file = File::create("survey.json").expect("Dosya oluşturma hatası!");
        file.write_all(json.as_bytes()).expect("Yazma hatası!");
        println!("Anket başarıyla kaydedildi.");
    }
}