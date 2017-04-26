use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;

extern crate regex;
use regex::Regex;

extern crate csv;
use csv::Writer;

fn main() {
    // let mut file = File::create("bar.txt").unwrap();
    // file.write_all(b"Hello world\n").unwrap();
    // file.write_all(b"Or not?").unwrap();

    // let f = File::open("bar.txt").unwrap();
    // let buf = BufReader::new(&f);

    // for line in buf.lines() {
    //     println!("{}", line.unwrap());
    // }

    //let exmpl = "TIM_SetCompare4                          0x08004fb1   Thumb Code    58  stm32f4xx_tim.o(i.TIM_SetCompare4)";
    let exmpl = "tasks::JackRecorderTask_t::getMaxTriple(tasks::JackRecorderTask_t::Triple&, const short*, unsigned) 0x080057dd   Thumb Code   122  jack_recorder.o(i._ZN5tasks18JackRecorderTask_t12getMaxTripleERNS0_6TripleEPKsj)";

    // mathes all the text before the first hex number - that's supposed to be a function prototype
    let re = Regex::new(r".*?(?:0x)").unwrap();
    let caps = re.captures(exmpl).unwrap();

    // unfortunately I couldn't craft ideal regexp so I have to remove the traling "0x" like that
    let funcname = caps.get(0).unwrap().as_str().trim_right_matches("0x");

    println!("{}", funcname);

    let address = Regex::new(r"\s0[xX][0-9a-fA-F]+").unwrap().captures(exmpl).unwrap().get(0).unwrap().as_str().trim();

    println!("{}", address);

    let size = Regex::new(r"\s\d+\s").unwrap().captures(exmpl).unwrap().get(0).unwrap().as_str().trim();

    println!("{}", size);

    let mut writer = csv::Writer::from_file("test.cvs").unwrap();
    let res = writer.encode( (funcname, size, address) );

    assert!(res.is_ok());

}
