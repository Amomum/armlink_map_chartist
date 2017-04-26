use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let mut file = File::create("bar.txt").unwrap();
    file.write_all(b"Hello world\n").unwrap();
    file.write_all(b"Or not?").unwrap();

    let f = File::open("bar.txt").unwrap();
    let buf = BufReader::new(&f);

    for line in buf.lines() {
        println!("{}", line.unwrap());
    }

}
