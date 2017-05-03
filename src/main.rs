use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;

extern crate clap;
use clap::App;
use clap::Arg;

extern crate regex;
use regex::Regex;

struct LinkerSymbol {
    name: String,
    object_file: String,
    size: u32, 
    //adr: u32,
}

impl LinkerSymbol {
    pub fn new() -> LinkerSymbol {
        LinkerSymbol {
            name: String::new(),
            //adr: 0,
            object_file: String::new(),
            size: 0,
        }
    }
}

fn main() {

    let args = App::new("armlink map chartist")
        .version("0.1")
        .author("amomum")
        .about("A simple parser of armlink map file. \
        Primary usage is code-size optimization - you will be able to find out what functions should take the blame.")
        .arg( Arg::with_name("map file")
        .index(1)
        .required(true)
        .help("Input linker map file"))
        .get_matches();

    let file_name = args.value_of("map file").unwrap();
    let f = File::open(file_name).unwrap();

    let reader = BufReader::new(&f);

    // searching for the segment with function sizes
    let mut line_iter = reader.lines();

    line_iter.position( |x| x.unwrap().contains("Symbol Name                              Value     Ov Type        Size  Object(Section)"));

    // parsing this shit

    let mut linker_symbols: Vec<Box<LinkerSymbol>> = Vec::new();

    //let regex_addr = Regex::new(r"\s0[xX][0-9a-fA-F]+").unwrap();
    let regex_func_name = Regex::new(r".*?(?:\s0x)").unwrap();
    let regex_size = Regex::new(r"\s\d+\s").unwrap();
    let regex_obj_file = Regex::new(r"\w+[.][o]").unwrap();

    let line_iter = line_iter.enumerate();

    for (_, line) in line_iter {
        // end of section

        let line = line.unwrap();

        //println!("{} : {}", i, line.as_str());

        if line.contains("==============================================================================") { 
            break;
        }

        if !line.contains("Thumb Code") {
            continue;
        }

        //println!("----- Line accepted");

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

        // let t = regex_addr
        //     .captures(line_str)
        //     .unwrap()
        //     .get(0)
        //     .unwrap()
        //     .as_str()
        //     .trim();

        // symbol.adr = u32::from_str_radix(&t[2..], 16).unwrap();

        linker_symbols.push(symbol);
    }

    // sort results by size - coz that's what I'm mostly interested in
    linker_symbols.sort_by(|a, b| a.size.cmp(&b.size));
    linker_symbols.reverse();

    println!("Code size: {}",
             linker_symbols
                 .iter()
                 .fold(0u32, |sum, symbol| sum + symbol.size));

    // write them to file with TAB as a separator since both comma and space could be inside function names
    let file = File::create("result.txt").unwrap();
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
