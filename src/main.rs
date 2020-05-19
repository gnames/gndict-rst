#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
//! Takes data about scientific names and generic names from a database and uses the data to create
//! black, white and grey dictionaries for gnfinder.

mod assets;
mod conf;
mod download;
mod error;
mod pg;

use log::{error, info};
use std::fs::create_dir_all;
use std::process;
use std::thread::spawn;
use stderrlog::{self, Timestamp};

use clap::{App, Arg};

fn main() {
    stderrlog::new()
        .verbosity(2)
        .timestamp(Timestamp::Second)
        .init()
        .unwrap();
    let cfg = match conf::Conf::new() {
        Ok(cnf) => cnf,
        Err(err) => {
            error!("Error: {}", err);
            process::exit(1);
        }
    };
    let mut app = get_app();
    let matches = app.clone().get_matches();

    if std::env::args().collect::<Vec<String>>().len() == 1 {
        app.print_long_help().unwrap();
        println!();
    }

    if matches.is_present("version") {
        println!("Version: {}", clap::crate_version!());
        process::exit(0);
    }

    let mut reload = false;
    if matches.is_present("reload") {
        reload = true;
    }
    if download::exists(&cfg) && !reload {
        info!("Data downloaded already, skipping download.");
    } else {
        info!("Downloading names and genera...");
        create_dir_all(&cfg.work_dir).unwrap();
        let cfg_clone1 = cfg.clone();
        let cfg_clone2 = cfg.clone();
        let mut handles = vec![];
        handles.push(spawn(move || download::download_names(cfg_clone1)));
        handles.push(spawn(move || download::download_genera(cfg_clone2)));
        for h in handles {
            h.join().unwrap();
        }
        info!("Download succeded.");
    }
    let dict = assets::Dict::new(&cfg.work_dir);
    dict.canonicals();
}

fn get_app<'a, 'b>() -> App<'a, 'b> {
    App::new("gndict")
        .about("Creates dictionaries for gnfinder")
        .arg(Arg::with_name("version").short("V").help("Shows version"))
        .arg(
            Arg::with_name("reload")
                .short("r")
                .help("starts anew deleting downloaded data"),
        )
}
