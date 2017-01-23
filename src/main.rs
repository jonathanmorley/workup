#[macro_use]
extern crate clap;

extern crate workup;

use clap::{Arg, ArgMatches};

use std::env;
use std::path::PathBuf;

fn matches<'a>() -> ArgMatches<'a> {
    app_from_crate!()
        .about("Runs workup")
        .arg(Arg::with_name("offline")
            .short("o")
            .long("offline")
            .help("Work offline")
            .takes_value(false))
        .arg(Arg::with_name("policyfile")
            .short("p")
            .long("policyfile")
            .help("Policyfile to use for converge")
            .takes_value(true))
        .arg(Arg::with_name("verify_ssl")
            .long("verify_ssl")
            .takes_value(false))
        .get_matches()
}

fn main() {
    let matches = matches();

    let home_dir = env::home_dir();
    let workup_dir = match home_dir {
        Some(x) => x.join(".workup"),
        None => panic!("No home directory found")
    };

    let default_policyfile = workup_dir.join("Policyfile.rb");

    let policyfile = match matches.value_of("policyfile") {
        Some(x) => PathBuf::from(x),
        None => default_policyfile
    };

    workup::ensure_files(&workup_dir);

    if !(matches.is_present("offline")) {
        workup::chef_update(&policyfile);
        workup::chef_export(&policyfile);
    }

    workup::chef_client(&policyfile);
}
