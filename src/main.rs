use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;

extern crate regex;
use regex::Regex;

extern crate csv;
use csv::Writer;

struct LinkerSymbol {
    name: String,
    object_file: String,
    size: u32,
}

impl LinkerSymbol {
    pub fn new() -> LinkerSymbol {
        LinkerSymbol {
            name: String::new(),
            object_file: String::new(),
            size: 0,
        }
    }
}

fn main() {

    let f = File::open("bootloader.map").unwrap();
    let mut reader = BufReader::new(&f);

    // searching for the segment with function sizes
    let mut line_iter = reader.lines();

    let start_pos = line_iter.position( |x| x.unwrap().contains("Symbol Name                              Value     Ov Type        Size  Object(Section)"));

    // parsing this shit

    let mut linker_symbols: Vec<Box<LinkerSymbol>> = Vec::new();

    let regex_addr = Regex::new(r"\s0[xX][0-9a-fA-F]+").unwrap();
    let regex_func_name = Regex::new(r".*?(?:0x)").unwrap();
    let regex_size = Regex::new(r"\s\d+\s").unwrap();
    let regex_obj_file = Regex::new(r"\w+.o").unwrap();

    while line_iter.next().is_some() {
        // end of section
        let line = line_iter.next().unwrap().unwrap();
        if line.contains("==============================================================================") { 
            break;
        }

        if !line.contains("Thumb Code") {
            continue;
        }

        let mut symbol = Box::new(LinkerSymbol::new());
        let line_str = line.as_str();
        // unfortunately I couldn't craft ideal regexp so I have to remove the traling "0x" like that
        symbol.name = regex_func_name
            .captures(line_str)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .trim_right_matches("0x")
            .to_string();

        symbol.size = regex_size
            .captures(line_str)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .trim()
            .parse::<u32>()
            .unwrap();

        symbol.object_file = regex_obj_file
            .captures(line_str)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .to_string();

        linker_symbols.push(symbol);
    }

    // sort results by size - coz that's what I'm mostly interested in
    linker_symbols.sort_by(|a, b| a.size.cmp(&b.size));
    linker_symbols.reverse();


    // write them to file with TAB as a separator since both comma and space could be inside function names
    let mut file = File::create("result.txt").unwrap();
    let mut writer = BufWriter::new(&file);

    for symbol in linker_symbols {
        writeln!(&mut writer,
                 "{}\t{}\t{}",
                 symbol.name,
                 symbol.size,
                 symbol.object_file)
                .unwrap();
    }
}
