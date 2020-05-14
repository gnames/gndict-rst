#[warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
mod conf;
mod error;

use std::process;

use clap::{App, Arg, ArgMatches, SubCommand};

fn main() {
    let cnf = match conf::Conf::new() {
        Ok(cnf) => cnf,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };
    let mut app = get_app();
    let matches = app.clone().get_matches();

    if std::env::args().collect::<Vec<String>>().len() == 1 {
        app.print_long_help().unwrap();
    }

    if matches.is_present("version") {
        println!("Version: {}", clap::crate_version!());
    }

    if let Some(matches) = matches.subcommand_matches("init") {
        println!("Subcomand init");
        if matches.is_present("reload") {
            println!("reload");
        } else {
            println!("dont reload");
        }
    }
}

fn get_app<'a, 'b>() -> App<'a, 'b> {
    App::new("gndict")
        .author("Dmitry Mozzherin <dmozzherin@gmail.com>")
        .about("Creates dictionaries for gnfinder")
        .arg(Arg::with_name("version").short("V").help("Shows version"))
        .subcommand(
            SubCommand::with_name("init")
                .about("Downloads raw data from gnindex database")
                .arg(
                    Arg::with_name("reload")
                        .short("r")
                        .help("starts anew deleting downloaded data"),
                ),
        )
}

