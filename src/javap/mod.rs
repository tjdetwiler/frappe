use std::io;

use classfile::ClassFile;
use classfile::attr;
use classfile::attr::AttributeInfo;
use classfile::attr::annotation;
use classfile::constant_pool as cp;
use classfile::method;

pub struct Formatter {
    indent: usize,
    out: Box<io::Write>,
}

impl Formatter {
    pub fn new() -> Formatter {
        Formatter {
            indent: 0,
            out: Box::new(io::stdout()),
        }
    }

    pub fn indent(&mut self) {
        self.indent = self.indent + 1
    }

    pub fn unindent(&mut self) {
        if self.indent <= 0 {
            panic!("Attempting to negatively indent");
        }
        self.indent = self.indent - 1
    }

    pub fn println(&mut self, line: &str) -> io::Result<()> {
        for _ in 0..self.indent {
            try!(write!(self.out, "  "));
        }
        try!(write!(self.out, "{}", line));
        Ok(())
    }
}

pub struct Options<'a> {
    pub verbose: bool,
    pub constant_pool: &'a cp::ConstantPool,
}

pub trait Disassemble {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()>;
}

impl Disassemble for ClassFile {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        for attribute in self.attributes.iter() {
            if let attr::AttributeInfo::SourceFile(ref sourcefile_info) = *attribute {
                let source_file = &self.constant_pool[sourcefile_info.sourcefile_index];
                write!(fmt.out,
                       "Compiled from \"{}\"\n", source_file.as_utf8().unwrap());
            }
        }
        let this_class = self.this_class();
        let class_name = self.constant_pool.get_string(this_class.name_index).unwrap();
        if self.access_flags.is_public() {
            write!(fmt.out, "public ");
        }
        if self.access_flags.is_interface() {
            write!(fmt.out, "interface");
        } else if self.access_flags.is_annotation() {
            write!(fmt.out, "@interface");
        } else {
            write!(fmt.out, "class");
        }
        write!(fmt.out, " {} ", class_name.replace("/", "."));
        if let Some(super_class) = self.super_class() {
            let super_class_name = self.constant_pool.get_string(super_class.name_index).unwrap();
            write!(fmt.out, "extends {} ", super_class_name.replace("/", "."));
        }
        if opts.verbose {
            write!(fmt.out, "\n");
            write!(fmt.out, "  minor version: {}\n", self.minor_version);
            write!(fmt.out, "  major version: {}\n", self.major_version);
            write!(fmt.out, "  access flags: {}\n", self.access_flags);
            self.constant_pool.pretty_print(fmt, opts);
        }
        write!(fmt.out, "{{\n");
        self.methods.pretty_print(fmt, opts);
        write!(fmt.out, "}}");
        Ok(())
    }
}

impl Disassemble for cp::Tag {
    fn pretty_print(&self, fmt: &mut Formatter, _: &Options) -> io::Result<()> {
        match *self {
            cp::Tag::Methodref(cp::MethodrefTag { ref class_index, ref name_and_type_index }) => {
                write!(fmt.out,
                       "Methodref\t\t#{}.#{}", class_index, name_and_type_index);
            }
            cp::Tag::Fieldref(cp::FieldrefTag { ref class_index, ref name_and_type_index }) => {
                write!(fmt.out,
                       "Fieldref\t\t\t#{}.#{}", class_index, name_and_type_index);
            }
            cp::Tag::String(cp::StringTag { ref string_index }) => {
                write!(fmt.out,
                       "String\t\t\t#{}", string_index);
            }
            cp::Tag::Class(cp::ClassTag { ref name_index }) => {
                write!(fmt.out,
                       "Class\t\t\t#{}", name_index);
            }
            cp::Tag::Utf8(ref string) => {
                write!(fmt.out, "Utf8\t\t\t{}", string);
            }
            cp::Tag::NameAndType(cp::NameAndTypeTag { ref name_index, ref descriptor_index }) => {
                write!(fmt.out,
                       "NameAndType\t\t#{}:#{}", name_index, descriptor_index);
            }
            _ => {
                write!(fmt.out, "{:?}", self);
            }
        }
        Ok(())
    }
}

impl Disassemble for cp::ConstantPool {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        write!(fmt.out, "Constant pool:\n");
        for (i, tag) in self.iter().enumerate() {
            write!(fmt.out, "  #{} = ", i + 1);
            tag.pretty_print(fmt, opts);
            write!(fmt.out, "\n");
        }
        Ok(())
    }
}

impl Disassemble for method::Methods {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        for method in self.iter() {
            method.pretty_print(fmt, opts);
            write!(fmt.out, "\n");
        }
        Ok(())
    }
}

impl Disassemble for attr::Attributes {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        for attr in self.iter() {
            attr.pretty_print(fmt, opts);
            write!(fmt.out, "\n");
        }
        Ok(())
    }
}

impl Disassemble for AttributeInfo {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        match *self {
            AttributeInfo::SourceFile(ref source_file) => {
                write!(fmt.out, "SourceFile");
            }
            AttributeInfo::AnnotationDefault(ref annotation_default) => {
                annotation_default.pretty_print(fmt, opts);
            }
            AttributeInfo::Code(ref code) => { code.pretty_print(fmt, opts); }
            _ => {
                write!(fmt.out, "Other");
            }
        }
        Ok(())
    }
}

impl Disassemble for method::MethodInfo {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        write!(fmt.out, "  ");
        if self.access_flags.is_public() {
            write!(fmt.out, "public ");
        } else if self.access_flags.is_private() {
            write!(fmt.out, "private ");
        } else if self.access_flags.is_protected() {
            write!(fmt.out, "protected ");
        } else {
            write!(fmt.out, "/* package */ ");
        }
        if self.access_flags.is_static() {
            write!(fmt.out, "static ");
        }
        let method_descriptor = opts.constant_pool.get_string(self.descriptor_index);
        let method_name = opts.constant_pool.get_string(self.name_index);
        write!(fmt.out, "{};\n", method_name.unwrap());
        if opts.verbose {
            write!(fmt.out, "    descriptor: {}\n", method_descriptor.unwrap());
            write!(fmt.out, "    flags: {:?}\n", self.access_flags);
            for attr in self.attributes.iter() {
                attr.pretty_print(fmt, opts);
            }
        }
        Ok(())
    }
}

impl Disassemble for annotation::ElementValue {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        match *self {
            annotation::ElementValue::ConstantValue(ref constant_value) => {
                write!(fmt.out, "{}#{}", constant_value.tag as char, constant_value.const_value_index);
            }
            _ => { write!(fmt.out, "Unsupported ElementValue!"); }
        }
        Ok(())
    }
}

impl Disassemble for attr::AnnotationDefaultAttribute {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        write!(fmt.out, "AnnotationDefault:\n");
        write!(fmt.out, "  default_value: ");
        self.default_value.pretty_print(fmt, opts);
        write!(fmt.out, "\n");
        Ok(())
    }
}

impl Disassemble for attr::CodeAttribute {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        write!(fmt.out, "      stack={}, locals={}, args_size={}\n",
                                 self.max_stack,
                                 self.max_locals,
                                 "TODO!");
        Ok(())
    }
}
