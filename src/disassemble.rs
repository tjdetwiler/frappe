use classfile::ClassFile;
use classfile::attr;
use classfile::constant_pool as cp;
use classfile::method;

pub struct Options<'a> {
    pub verbose: bool,
    pub constant_pool: &'a cp::ConstantPool
}

pub trait Disassemble {
    fn pretty_print(&self, opts: &Options) -> String;
}

impl Disassemble for ClassFile {
    fn pretty_print(&self, opts: &Options) -> String {
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
        pretty.push_str(&format!(" {} ", class_name.replace("/", ".")));
        if opts.verbose {
            pretty.push_str("\n");
            pretty.push_str(&format!("  minor version: {}\n", self.minor_version));
            pretty.push_str(&format!("  major version: {}\n", self.major_version));
            pretty.push_str(&format!("  access flags: {}\n", self.access_flags));
            pretty.push_str(&format!("{}\n", self.constant_pool.pretty_print(&opts)));
        }
        pretty.push_str(&format!("{{\n"));
        pretty.push_str(&self.methods.pretty_print(opts));
        pretty.push_str("}");
        pretty
    }
}

impl Disassemble for cp::Tag {
    fn pretty_print(&self, _: &Options) -> String {
        match *self {
            cp::Tag::Methodref(cp::MethodrefTag{
                ref class_index, ref name_and_type_index}) => {
                format!("Methodref\t\t#{}.#{}", class_index, name_and_type_index)
            }
            cp::Tag::Fieldref(cp::FieldrefTag{
                ref class_index, ref name_and_type_index}) => {
                format!("Fieldref\t\t\t#{}.#{}", class_index, name_and_type_index)
            }
            cp::Tag::String(cp::StringTag { ref string_index }) => {
                format!("String\t\t\t#{}", string_index)
            }
            cp::Tag::Class(cp::ClassTag { ref name_index }) => {
                format!("Class\t\t\t#{}", name_index)
            }
            cp::Tag::Utf8(ref string) => format!("Utf8\t\t\t{}", string),
            cp::Tag::NameAndType(cp::NameAndTypeTag{
                ref name_index, ref descriptor_index}) => {
                format!("NameAndType\t\t#{}:#{}", name_index, descriptor_index)
            }
            _ => format!("{:?}", self),
        }
    }
}

impl Disassemble for cp::ConstantPool {
    fn pretty_print(&self, opts: &Options) -> String {
        let mut pretty = String::new();
        pretty.push_str(&format!("Constant pool:\n"));
        for (i, tag) in self.iter().enumerate() {
            pretty.push_str(&format!("  #{} = {}\n", i + 1, tag.pretty_print(&opts)));
        }
        pretty
    }
}

impl Disassemble for method::Methods {
    fn pretty_print(&self, opts: &Options) -> String {
        let mut pretty = String::new();
        for method in self.iter() {
            pretty.push_str(&format!("{}\n", method.pretty_print(opts)));
        }
        pretty
    }
}

impl Disassemble for method::MethodInfo {
    fn pretty_print(&self, opts: &Options) -> String {
        let mut pretty = String::new();
        pretty.push_str("  ");
        if self.access_flags.is_public() {
            pretty.push_str("public ");
        } else if self.access_flags.is_private() {
            pretty.push_str("private ");
        } else if self.access_flags.is_protected() {
            pretty.push_str("protected ");
        } else {
            pretty.push_str("/* package */ ");
        }
        if self.access_flags.is_static() {
            pretty.push_str("static ");
        }
        let method_descriptor = opts.constant_pool.get_string(self.descriptor_index);
        let method_name = opts.constant_pool.get_string(self.name_index);
        pretty.push_str(&format!("{};\n", method_name.unwrap()));
        if opts.verbose {
            pretty.push_str(&format!("    descriptor: {}\n", method_descriptor.unwrap()));
            pretty.push_str(&format!("    flags: {:?}\n", self.access_flags));
            if !self.access_flags.contains(method::ACC_ABSTRACT) {
                pretty.push_str("    Code: \n");
                for attr in self.attributes.iter() {
                    if let attr::AttributeInfo::Code(ref code) = *attr {
                        pretty.push_str(&code.pretty_print(opts));
                    }
                }
            }
        }
        pretty
    }
}

impl Disassemble for attr::Code {
    fn pretty_print(&self, opts: &Options) -> String {
        let mut pretty = String::new();
        pretty.push_str(&format!("      stack={}, locals={}, args_size={}\n",
                                 self.max_stack,
                                 self.max_locals,
                                 "TODO!"));
        pretty
    }
}
