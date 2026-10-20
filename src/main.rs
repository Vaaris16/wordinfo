use clap::{Parser, parser::ValuesRef};
use reqwest::Client;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(version, about = "Look up words from the terminal")]
struct Commands {
    #[arg(short = 'w', long = "word")]
    word: String,
}

struct Request {
    word: String,
}

#[derive(Deserialize, Debug)]
struct WordInfo {
    word: String,
    entries: Vec<Entry>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Entry {
    part_of_speech: String,
    pronunciations: Vec<Pronunciation>,
    forms: Vec<Forms>,
    senses: Vec<Sense>,
}

#[derive(Deserialize, Debug)]
struct Sense {
    definition: String,
    examples: Vec<String>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Forms {
    word: String,
    tags: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Pronunciation {
    text: String,
    tags: Vec<String>,
}

impl Request {
    fn new(word: String) -> Self {
        Self { word: word }
    }
    fn get_response(&self) -> WordInfo {
        let url = format!(
            "https://freedictionaryapi.com/api/v1/entries/en/{}",
            self.word
        );

        let response = reqwest::blocking::get(&url)
            .map_err(|e| println!("Request error: {e}"))
            .unwrap();

        let word_info: WordInfo = response.json().unwrap();
        word_info
    }
}

fn main() {
    let cli = Commands::parse();

    let reqest = Request::new(cli.word);
    let word_info = reqest.get_response();
    println!("{:#?}", word_info);
}
