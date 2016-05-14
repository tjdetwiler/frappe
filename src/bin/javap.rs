extern crate frappe;

use std::fs::File;

use frappe::classfile::ClassFile;
use frappe::classfile::attr;

fn main() {
    let verbose = true;
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
    let this_class = class.this_class();
    let class_name = class.constant_pool.get_string(this_class.name_index).unwrap();
    if class.access_flags.is_public() {
        print!("public ");
    }
    if class.access_flags.is_interface() {
        print!("interface");
    } else if class.access_flags.is_annotation() {
        print!("@interface");
    } else {
        print!("class");
    }
    print!(" ");
    print!("{}", class_name.replace("/", "."));
    println!(" {{");
    if verbose {
        println!("  minor version: {}", class.minor_version);
        println!("  major version: {}", class.minor_version);
        println!("  flags: TODO");
        println!("Constant pool:");
        for (i, tag) in class.constant_pool.iter().enumerate() {
            println!("  #{} = {:?}", i+1, tag);
        }
    }
    for method in class.methods.iter() {
        print!("  ");
        if method.access_flags.is_public() {
            print!("public ");
        } else if method.access_flags.is_private() {
            print!("private ");
        } else if method.access_flags.is_protected() {
            print!("protected ");
        } else {
            print!("/* package */ ");
        }
        if method.access_flags.is_static() {
            print!("static ");
        }
        let method_name = class.constant_pool.get_string(method.name_index);
        println!("{}", method_name.unwrap());
    }
    println!("}}");
}
