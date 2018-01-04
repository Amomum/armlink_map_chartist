use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;

#[macro_use]
extern crate clap;
use clap::App;
use clap::Arg;

extern crate regex;
use regex::Regex;

struct LinkerSymbol {
    name: String,
    object_file: String,
    size: u32,
    orig: String
}

impl LinkerSymbol {
    pub fn new() -> LinkerSymbol {
        LinkerSymbol {
            name: String::new(),
            object_file: String::new(),
            size: 0,
            orig: String::new()
        }
    }
}

fn main() {

    let args = App::new("armlink map chartist")
        .version(crate_version!())
        .about("A simple parser of armlink map file. \
        Primary usage is code-size optimization - you will be able to find out what code should take the blame. \
        Output is TAB-separated file because comma and space can be in the symbol names. \
        \nBEWARE: It's not very accurate! The total numbers do not always add up with armlink total info.")
        .arg( Arg::with_name("map file")
            .index(1)
            .required(true)
            .help("Input linker map file") )
        .arg( Arg::with_name("output file name")
            .index(2)
            .required( false )
            .default_value("result.txt")
            .help( "Output text file name")
        )
        .get_matches();

    let map_file_name = args.value_of("map file").unwrap();
    let f = File::open(map_file_name).unwrap();

    let reader = BufReader::new(&f);

    // searching for the segment with function sizes
    let mut line_iter = reader.lines();

    line_iter.position( |x| x.unwrap().contains("Symbol Name                              Value     Ov Type        Size  Object(Section)"));

    // parsing this shit

    let mut linker_symbols: Vec<Box<LinkerSymbol>> = Vec::new();

    let regex_func_name = Regex::new(r".*?(?:\s0x)").unwrap();
    let regex_size = Regex::new(r"\s\d+\s").unwrap();
    let regex_obj_file = Regex::new(r"\w+[.][o]").unwrap();

    let line_iter = line_iter.enumerate();

    let mut code_size = 0;
    let mut const_data_size = 0;

    for (_, line) in line_iter {
        // end of section

        let line = line.unwrap();

        if line.contains("==============================================================================") {
            break;
        }

        if !line.contains("Thumb Code") && !line.contains( ".constdata"){
            continue;
        }

        let mut symbol = Box::new(LinkerSymbol::new());
        let line_str = line.as_str();

        symbol.orig = line_str.to_string();

        // unfortunately I couldn't craft ideal regexp so I have to remove the trailing "0x" like that
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

        if line.contains("Thumb Code" ) {
            code_size += symbol.size;
        }
        else {
            const_data_size += symbol.size;
        }

        linker_symbols.push(symbol);
    }

    // sort results by size - coz that's what I'm mostly interested in
    linker_symbols.sort_by(|a, b| a.size.cmp(&b.size));
    linker_symbols.reverse();

    println!("Code size: {}, const data size {}", code_size, const_data_size );

    // write them to file with TAB as a separator since both comma and space could be inside function names

    let output_filename = args.value_of("output file name").unwrap();

    let file = File::create(output_filename).unwrap();
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
