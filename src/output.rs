use crate::assets::Dict;
use crate::conf::Conf;
use log::info;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub struct Output<'a> {
    path: String,
    dict: &'a Dict,
}

impl<'a> Output<'a> {
    pub fn new(cnf: &Conf, dict: &'a Dict) -> Self {
        let o = Output {
            path: cnf.work_dir.to_owned(),
            dict,
        };
        o
    }

    pub fn process(&self) {
        self.prepare_uninomials();
        self.prepare_genera();
        self.prepare_species();
    }

    fn prepare_uninomials(&self) {
        info!("Writing down uninomials final dictionaries");
        let mut out_white: Vec<String> = Vec::new();
        let mut out_grey: Vec<String> = Vec::new();
        let mut white = File::create(
            Path::new(&self.path)
                .join("dict")
                .join("white")
                .join("uninomials.csv"),
        )
        .unwrap();
        let mut grey = File::create(
            Path::new(&self.path)
                .join("dict")
                .join("grey")
                .join("uninomials.csv"),
        )
        .unwrap();
        let species = File::open(Path::new(&self.path).join("uninomials.csv")).unwrap();
        let r = BufReader::new(species);

        for line in r.lines() {
            let line = line.unwrap();
            let word = line.splitn(2, ",").next().unwrap();
            if self.uninomial_problems(word) {
                continue;
            };
            if self.is_grey_word(word) {
                out_grey.push(line);
            } else {
                out_white.push(line);
            }
        }

        out_grey.sort();
        for line in out_grey {
            writeln!(grey, "{}", line).unwrap();
        }

        out_white.sort();
        for line in out_white {
            writeln!(white, "{}", line).unwrap();
        }
    }

    fn prepare_genera(&self) {
        info!("Writing down genera final dictionaries");
        let mut out_white: Vec<String> = Vec::new();
        let mut out_grey: Vec<String> = Vec::new();
        let mut out_grey_species: Vec<String> = Vec::new();
        let mut white = File::create(
            Path::new(&self.path)
                .join("dict")
                .join("white")
                .join("genera.csv"),
        )
        .unwrap();
        let mut grey = File::create(
            Path::new(&self.path)
                .join("dict")
                .join("grey")
                .join("genera.csv"),
        )
        .unwrap();
        let mut grey_species = File::create(
            Path::new(&self.path)
                .join("dict")
                .join("grey")
                .join("genera_species.csv"),
        )
        .unwrap();
        let species = File::open(Path::new(&self.path).join("genera.csv")).unwrap();
        let r = BufReader::new(species);

        for line in r.lines() {
            let line = line.unwrap();
            let word = line.splitn(2, ",").next().unwrap();
            if self.generic_problems(word) {
                continue;
            };
            if self.is_grey_word(word) {
                out_grey.push(line.to_owned());
                if let Some((_, names)) = self.dict.canonicals.get_key_value(word) {
                    for name in names {
                        out_grey_species.push(name.to_owned());
                    }
                }
            } else {
                out_white.push(line);
            }
        }
        out_white.sort();
        for line in out_white {
            writeln!(white, "{}", line).unwrap();
        }
        out_grey.sort();
        for line in out_grey {
            writeln!(grey, "{}", line).unwrap();
        }
        let out_grey_species = self.prepare_grey_species(out_grey_species);
        for line in out_grey_species {
            writeln!(grey_species, "{}", line).unwrap();
        }
    }

    fn prepare_grey_species(&self, names: Vec<String>) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        for name in names {
            res.push(name.to_owned());
            let words: Vec<&str> = name.split_whitespace().collect();
            if words.len() == 3 {
                res.push(format!("{} {}", words[0], words[1]));
                res.push(format!("{} {}", words[0], words[2]));
            }
        }
        res.sort_unstable();
        res.dedup();
        res
    }

    fn prepare_species(&self) {
        info!("Writing down species final dictionaries");
        let mut out_white: Vec<String> = Vec::new();
        let mut out_grey: Vec<String> = Vec::new();
        let mut white = File::create(
            Path::new(&self.path)
                .join("dict")
                .join("white")
                .join("species.csv"),
        )
        .unwrap();
        let mut grey = File::create(
            Path::new(&self.path)
                .join("dict")
                .join("grey")
                .join("species.csv"),
        )
        .unwrap();
        let species = File::open(Path::new(&self.path).join("species.csv")).unwrap();
        let r = BufReader::new(species);
        for line in r.lines() {
            let line = line.unwrap();
            let word = line.splitn(2, ",").next().unwrap();
            if self.species_problems(word) {
                continue;
            };
            if self.is_grey_word(word) {
                out_grey.push(line);
            } else {
                out_white.push(line);
            }
        }
        out_white.sort();
        for line in out_white {
            writeln!(white, "{}", line).unwrap();
        }
        out_grey.sort();
        for line in out_grey {
            writeln!(grey, "{}", line).unwrap();
        }
    }

    fn species_problems(&self, sp: &str) -> bool {
        if sp.len() < 2
            || self.dict.species_black.contains(&sp.to_lowercase())
            || sp.find(char::is_numeric).is_some()
            || sp.contains(".")
        {
            return true;
        }
        false
    }

    fn generic_problems(&self, word: &str) -> bool {
        if self.dict.uninomials_black.contains(&word.to_lowercase()) || word.contains(".") {
            return true;
        }
        false
    }

    fn uninomial_problems(&self, word: &str) -> bool {
        if self.dict.uninomials_black.contains(&word.to_lowercase()) || word.contains(".") {
            return true;
        }
        false
    }

    fn is_grey_word(&self, word: &str) -> bool {
        if word.len() < 4 || self.dict.common_words.contains(&word.to_lowercase()) {
            return true;
        }
        false
    }
}
