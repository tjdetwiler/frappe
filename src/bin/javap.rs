extern crate frappe;

use std::fs::File;

use frappe::classfile::ClassFile;
use frappe::classfile::attr;
use frappe::classfile::constant_pool;

struct JavapOptions<'a> {
    verbose: bool,
    classfile: &'a ClassFile
}

trait Javap {
    fn pretty_print(&self, opts: &JavapOptions) -> String;
}

impl Javap for ClassFile {
    fn pretty_print(&self, opts: &JavapOptions) -> String {
        let mut pretty = String::new();
        for attribute in self.attributes.iter() {
            if let attr::AttributeInfo::SourceFile(ref sourcefile_info) = *attribute {
                let source_file = &self.constant_pool[sourcefile_info.sourcefile_index];
                pretty.push_str(&format!("Compiled from \"{}\"\n", source_file.as_utf8().unwrap()));
            }
        }
        let this_class = self.this_class();
        let class_name = self.constant_pool.get_string(this_class.name_index).unwrap();
        if self.access_flags.is_public() {
            pretty.push_str(&format!("public "));
        }
        if self.access_flags.is_interface() {
            pretty.push_str("interface");
        } else if self.access_flags.is_annotation() {
            pretty.push_str("@interface");
        } else {
            pretty.push_str("class");
        }
        pretty.push_str(&format!(" {}", class_name.replace("/", ".")));
        pretty.push_str(&format!(" {{\n"));
        if opts.verbose {
            pretty.push_str(&format!("  minor version: {}\n", self.minor_version));
            pretty.push_str(&format!("  major version: {}\n", self.major_version));
            pretty.push_str(&format!("  access flags: {}\n", self.access_flags));
            pretty.push_str(&format!("{}\n", self.constant_pool.pretty_print(&opts)));
        }
        for method in self.methods.iter() {
            pretty.push_str("  ");
            if method.access_flags.is_public() {
                pretty.push_str("public ");
            } else if method.access_flags.is_private() {
                pretty.push_str("private ");
            } else if method.access_flags.is_protected() {
                pretty.push_str("protected ");
            } else {
                pretty.push_str("/* package */ ");
            }
            if method.access_flags.is_static() {
                pretty.push_str("static ");
            }
            let method_name = self.constant_pool.get_string(method.name_index);
            pretty.push_str(&format!("{}\n", method_name.unwrap()));
        }
        pretty.push_str("}");
        pretty
    }
}

impl Javap for constant_pool::Tag {
    fn pretty_print(&self, _: &JavapOptions) -> String {
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
    fn pretty_print(&self, opts: &JavapOptions) -> String {
        let mut pretty = String::new();
        pretty.push_str(&format!("Constant pool:\n"));
        for (i, tag) in self.iter().enumerate() {
            pretty.push_str(&format!("  #{} = {}\n", i, tag.pretty_print(&opts)));
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
    let opts = JavapOptions { verbose: verbose, classfile: &class };
    println!("{}", class.pretty_print(&opts));
}
