extern crate frappe;

use std::fs::File;

use frappe::class::ClassFile;
use frappe::attr;


fn main() {
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
