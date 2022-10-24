use std::fs::{self, File};
use std::io::Read;
use std::process::Command;

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

        Command::new(route).spawn();
    }
}
