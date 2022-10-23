use std::fs::{self, File};
use std::io::Read;
use std::str::FromStr;
use regex::{Captures, Regex};

pub fn open_files() {
    let mut file = File::open("routes.txt")
        .expect("File not found");

    let mut data = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let num_pair_str = data.split(';').collect::<Vec<_>>();

    let vec1 = num_pair_str.iter();

    for val in vec1 {
        let route = val.replace(";", "").replace("\n", "");


        match open::that(route) {
            Ok(route) => println!("Opened '{}' successfully.", route),
            Err(err) => eprintln!("An error occurred when opening"),
        }
    }
}
