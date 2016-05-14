extern crate frappe;

use std::fs::File;

use frappe::class::ClassFile;
use frappe::attr;
use frappe::constant_pool as cp;

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
    let this_class = &class.constant_pool[class.this_class];
    if let cp::Tag::Class{name_index} = *this_class {
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
        let class_name = class.constant_pool[name_index].as_utf8().unwrap();
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
}
