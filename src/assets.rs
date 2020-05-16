use log::info;
use rust_embed::RustEmbed;
use std::collections::HashSet;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Asset;

#[derive(Debug)]
pub struct Dict {
    pub common_words: HashSet<String>,
    pub species_black: HashSet<String>,
    pub uninomials_black: HashSet<String>,
}

impl Dict {
    pub fn new() -> Self {
        info!("Building temp dictionaries");
        let mut dict = Dict {
            common_words: HashSet::new(),
            species_black: HashSet::new(),
            uninomials_black: HashSet::new(),
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

        dict
    }
}
