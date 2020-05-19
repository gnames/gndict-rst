use log::info;
use rust_embed::RustEmbed;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Asset;

#[derive(Debug)]
pub struct Dict {
    pub path: String,
    pub common_words: HashSet<String>,
    pub species_black: HashSet<String>,
    pub uninomials_black: HashSet<String>,
    pub genera: HashSet<String>,
}

impl Dict {
    pub fn new(path: &str) -> Self {
        info!("Building temp dictionaries");
        let mut dict = Dict {
            path: path.to_owned(),
            common_words: HashSet::new(),
            species_black: HashSet::new(),
            uninomials_black: HashSet::new(),
            genera: HashSet::new(),
        };

        let common_words =
            Asset::get("common-eu-words.txt").expect("Cannot find file common-eu-words.txt");
        let common_words_str =
            std::str::from_utf8(common_words.as_ref()).expect("Cannot read common-eu-words.txt");
        for word in common_words_str.lines() {
            dict.common_words.insert(word.trim().to_owned());
        }

        let uninomials_black =
            Asset::get("uninomials-black.txt").expect("Cannot find file uninomials_black.txt");
        let uninomials_black_str = std::str::from_utf8(uninomials_black.as_ref())
            .expect("Cannot read uninomials_black.txt");
        for word in uninomials_black_str.lines() {
            dict.uninomials_black.insert(word.trim().to_owned());
        }

        let species_black =
            Asset::get("species-black.txt").expect("Cannot find file species_black.txt");
        let species_black_str =
            std::str::from_utf8(species_black.as_ref()).expect("Cannot read species_black.txt");
        for word in species_black_str.lines() {
            dict.species_black.insert(word.trim().to_owned());
        }

        let f = File::open(Path::new(path).join("genera.txt")).unwrap();
        let r = BufReader::new(f);
        for line in r.lines() {
            let line = line.unwrap().to_owned();
            dict.genera.insert(line.trim().to_owned());
        }

        dict
    }

    pub fn canonicals(&self) {
        info!("Creating interim csv files.");
        let mut uninomials: HashMap<String, u32> = HashMap::new();
        let mut genera: HashMap<String, u32> = HashMap::new();
        let mut species: HashMap<String, u32> = HashMap::new();
        let f = File::open(Path::new(&self.path).join("names.txt")).unwrap();
        let r = BufReader::new(f);
        for line in r.lines() {
            let line: String = line.unwrap().to_owned();
            if line.find("×").is_some() {
                continue;
            }
            let words: Vec<&str> = line.trim().split(" ").collect();
            if words.len() == 1 {
                let uni = words[0];
                if self.genera.get(uni).is_none() {
                    let entry = uninomials.entry(words[0].to_owned()).or_default();
                    *entry += 1;
                } else {
                    let entry = genera.entry(words[0].to_owned()).or_default();
                    *entry += 1;
                }
            } else {
                let entry = genera.entry(words[0].to_owned()).or_default();
                *entry += 1;
                for word in &words[1..] {
                    let word = word.to_owned();
                    let entry = species.entry(word.to_owned()).or_default();
                    *entry += 1;
                }
            }
        }
        let mut genera_keys: Vec<String> = Vec::new();
        for (k, v) in &uninomials {
            if genera.contains_key(k) {
                let entry = genera.entry(k.clone()).or_default();
                *entry += *v;
                genera_keys.push(k.to_owned());
            }
        }
        for k in genera_keys {
            uninomials.remove(&k);
        }
        self.build_csv("uninomials.csv", uninomials);
        self.build_csv("genera.csv", genera);
        self.build_csv("species.csv", species);
    }
    fn build_csv(&self, name: &str, data: HashMap<String, u32>) {
        let mut f = File::create(Path::new(&self.path).join(name)).unwrap();
        for (k, v) in data {
            writeln!(f, "{},{}", k, v).unwrap();
        }
    }
}
