extern crate frappe;

use std::fs::File;

use frappe::classfile::ClassFile;
use frappe::javap::{Disassemble, Formatter, Options};

fn main() {
    let verbose = true;
    let classname = "io.hcf.frappe.HelloWorld";
    let mut class_filename = classname.replace('.', "/");
    class_filename.push_str(".class");
    let mut class_file = File::open(class_filename).unwrap();
    let mut fmt = Formatter::new();
    let class = ClassFile::read(&mut class_file).unwrap();
    let opts = Options {
        verbose: verbose,
        constant_pool: &class.constant_pool,
    };
    println!("{}", class.pretty_print(&mut fmt, &opts));
}
