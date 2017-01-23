extern crate hyper;

use hyper::{Client};
use hyper::client::{Body, RedirectPolicy};
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::status::StatusClass;

use std::path::Path;

use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;

mod configs;

fn write_file(path: &Path, contents: &str) -> () {
    let mut file = File::create(path).unwrap();

    file.write_all(contents.as_bytes()).unwrap();
}

pub fn ensure_files(workup_dir: &Path) -> () {
    println!("Ensuring files at {}", workup_dir.display());

    if !(workup_dir.exists()) {
        println!("{} does not exist, attempting to create", workup_dir.display());
        fs::create_dir_all(workup_dir).unwrap();
    } else if (workup_dir.is_dir()) {
        println!("{} is already a directory.", workup_dir.display());
    } else if !(workup_dir.is_file()) {
        panic!("{} is a file. Workup requires it to be a directory");
    } else {
        panic!("{} is an unknown file type. Workup requires it to be a directory");
    }

    let client_path = workup_dir.join("client.rb");
    if !(client_path.exists()) {
        write_file(&client_path, configs::client().as_str());
    }

    println!("{}", client_path.display());
}

pub fn chef_update(policyfile: &Path) -> () {
    println!("Updating chef at {}", policyfile.display());
}

pub fn chef_export(policyfile: &Path) -> () {
    println!("Exporting chef at {}", policyfile.display());
}

pub fn chef_client(policyfile: &Path) -> () {
    println!("Clienting chef at {}", policyfile.display());
}
