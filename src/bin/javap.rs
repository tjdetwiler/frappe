extern crate frappe;
extern crate getopts;

use std::env;
use std::fs::File;

use getopts::Options;
use frappe::class::ClassFile;
use frappe::constant_pool as cp;
use frappe::attr;


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut options = Options::new();
    options
        .optflag("h", "help", "Print this usage message")
        .optflag("", "version", "Version information")
        .optflag("v", "verbose", "Print additional information")
        .optflag("l", "", "Print line number and local variable tables");

    let matches = options.parse(&args[1..]).unwrap();
    println!("{:?}", args);

    let classname = "io.hcf.frappe.HelloWorld";
    let mut class_filename = classname.replace('.', "/");
    class_filename.push_str(".class");
    let mut class_file = File::open(class_filename).unwrap();
    let class = ClassFile::read(&mut class_file).unwrap();

    for attribute in class.attributes.iter() {
        if let attr::AttributeInfo::SourceFile(ref sourcefile_info) = *attribute {
            let source_file = &class.constant_pool[sourcefile_info.sourcefile_index];
            println!("Compiled from \"{}\"", source_file.as_utf8().unwrap());
        }
    }
    println!("{:?}", class);
}
