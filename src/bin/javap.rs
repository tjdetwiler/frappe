extern crate frappe;

use std::fs::File;

use frappe::classfile::ClassFile;
use frappe::classfile::attr;
use frappe::classfile::constant_pool;

trait Javap {
    fn pretty_print(&self) -> String;
}

impl Javap for constant_pool::Tag {
    fn pretty_print(&self) -> String {
        match *self {
            constant_pool::Tag::Methodref(constant_pool::MethodrefTag{ref class_index, ref name_and_type_index}) => {
                format!("Methodref\t\t#{}.#{}", class_index, name_and_type_index)
            },
            constant_pool::Tag::Fieldref(constant_pool::FieldrefTag{ref class_index, ref name_and_type_index}) => {
                format!("Fieldref\t\t\t#{}.#{}", class_index, name_and_type_index)
            },
            constant_pool::Tag::String(constant_pool::StringTag{ref string_index}) => {
                format!("String\t\t\t#{}", string_index)
            },
            constant_pool::Tag::Class(constant_pool::ClassTag{ref name_index}) => {
                format!("Class\t\t\t#{}", name_index)
            },
            constant_pool::Tag::Utf8(ref string) => {
                format!("Utf8\t\t\t{}", string)
            },
            constant_pool::Tag::NameAndType(constant_pool::NameAndTypeTag{ref name_index, ref descriptor_index}) => {
                format!("NameAndType\t\t#{}:#{}", name_index, descriptor_index)
            },
            _ => format!("{:?}", self)
        }
    }
}

impl Javap for constant_pool::ConstantPool {
    fn pretty_print(&self) -> String {
        let mut pretty = String::new();
        pretty.push_str(&format!("Constant pool:\n"));
        for (i, tag) in self.iter().enumerate() {
            pretty.push_str(&format!("  #{} = {}\n", i, tag.pretty_print()));
        }
        pretty
    }
}

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
        println!("  major version: {}", class.major_version);
        println!("  access flags: {}", class.access_flags);
        println!("{}", class.constant_pool.pretty_print());
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
