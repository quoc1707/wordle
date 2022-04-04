use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn dictionary_words(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file!");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line."))
        .collect()
}
