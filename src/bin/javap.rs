extern crate frappe;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::fs::File;
use std::env;

use frappe::classfile::reader::ClassReader;
use frappe::javap::{Disassemble, Formatter, Options};

fn main() {
    env_logger::init().unwrap();
    let verbose = true;
    let class_filename = env::args().nth(1).expect("usage: class_reader <class file>");
    let class_file = File::open(class_filename).unwrap();
    let mut fmt = Formatter::new();
    let class = ClassReader::new(class_file).read_class().unwrap();
    let opts = Options {
        verbose: verbose,
        constants: &class.constants,
    };
    class.pretty_print(&mut fmt, &opts).unwrap();
}
