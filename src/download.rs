use crate::{conf::Conf, pg};

use postgres::fallible_iterator::FallibleIterator;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

const NAME_FILE: &str = "names.txt";
const GENERA_FILE: &str = "genera.txt";

pub fn exists(cfg: &Conf) -> bool {
    let files = vec![NAME_FILE, GENERA_FILE];
    let mut num = files.len();
    for f in files {
        if Path::new(&cfg.work_dir).join(f).exists() {
            num -= 1;
        }
    }
    num == 0
}

pub fn download_names(cfg: Conf) {
    let path = Path::new(&cfg.work_dir).join(NAME_FILE);
    let mut f = File::create(path).unwrap();
    let mut db = pg::Db::new(cfg).expect("Cannot connect to DB");
    let mut res = db
        .get_data(
            "SELECT DISTINCT c.name
                FROM canonicals c
                    JOIN name_strings ns
                        ON ns.canonical_id = c.id
                    JOIN name_string_indices nsi
                        ON nsi.name_string_id = ns.id
                    JOIN data_sources ds
                        ON ds.id = nsi.data_source_id
                WHERE ds.is_curated = true",
            // OR ds.is_auto_curated = true",
        )
        .expect("Cannot execute names query");
    while let Some(row) = res.next().unwrap_or(None) {
        let name: &str = row.get(0);
        writeln!(f, "{}", name).unwrap();
    }
    append_ion_names(f);
}

pub fn append_ion_names(mut names_file: File) {
    let path = Path::new("data").join("ion-names.txt");
    let ion = io::BufReader::new(File::open(path).unwrap());
    for name in ion.lines() {
        let name: &str = &name.unwrap();
        writeln!(names_file, "{}", name).unwrap();
    }
}

pub fn download_genera(cfg: Conf) {
    let path = Path::new(&cfg.work_dir).join(GENERA_FILE);
    let mut f = File::create(path).unwrap();
    let mut db = pg::Db::new(cfg).expect("Cannot connect to DB");
    let mut res = db
        .get_data(
            "SELECT DISTINCT c.name
                FROM name_string_indices nsi
                    JOIN name_strings ns on ns.id = nsi.name_string_id
                    JOIN canonicals c on c.id = ns.canonical_id
                WHERE data_source_id = 181 AND RANK = 'genus'",
        )
        .expect("Cannot execute genera query");
    while let Some(row) = res.next().unwrap_or(None) {
        let genus: &str = row.get(0);
        writeln!(f, "{}", genus).unwrap();
    }
}
