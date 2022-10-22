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

    let mut num_pair_str = data.split(';').collect::<Vec<_>>();


    let vec1 = num_pair_str.iter();


    for val in vec1 {
        let re = Regex::new(r"[A-Za-z]").unwrap();

        let result = re.replace_all(val, |cap: &Captures| {
            match &cap[0] {
                ";" => "",
                "\n" => "",
                _ => panic!("We should never get here"),
            }.to_string()
        });

    }

    println!("{:?}", num_pair_str)
}
