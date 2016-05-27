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
        let class_name = class_name.replace("/", ".");
        let access_mode =  if self.access_flags.is_public() {
            "public "
        } else {
            ""
        };
        let class_type = if self.access_flags.is_interface() {
            "interface"
        } else if self.access_flags.is_annotation() {
            "@interface"
        } else {
            "class"
        };
        write!(fmt.out, "{}{} {}", access_mode, class_type, class_name); 
        if let Some(super_class) = self.super_class() {
            let super_class_name = self.constant_pool.get_string(super_class.name_index).unwrap();
            let super_class_name = super_class_name.replace("/", ".");
            write!(fmt.out, " extends {} ", super_class_name);
        }
        if opts.verbose {
            write!(fmt.out, "\n");
            write!(fmt.out, "  minor version: {}\n", self.minor_version);
            write!(fmt.out, "  major version: {}\n", self.major_version);
            write!(fmt.out, "  flags: {}\n", self.access_flags);
            self.constant_pool.pretty_print(fmt, opts);
        }
        write!(fmt.out, "{{\n");
        self.methods.pretty_print(fmt, opts);
        write!(fmt.out, "}}");
        Ok(())
    }
}

impl Disassemble for cp::Tag {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        let mut tag_string = "";
        let mut arg_string = String::new();
        let mut comment_string: Option<String> = None;
        match *self {
            cp::Tag::Methodref(ref method_tag) => {
                tag_string = "Methodref";
                arg_string = format!("#{}.#{}", method_tag.class_index, method_tag.name_and_type_index);
                let class_info = opts.constant_pool[method_tag.class_index]
                    .as_class().unwrap();
                let method_info = opts.constant_pool[method_tag.name_and_type_index]
                    .as_name_and_type().unwrap();
                let class_name = opts.constant_pool[class_info.name_index]
                    .as_utf8().unwrap();
                let method_name = opts.constant_pool[method_info.name_index]
                    .as_utf8().unwrap();
                let method_type = opts.constant_pool[method_info.descriptor_index]
                    .as_utf8().unwrap();
                comment_string = Some(format!("{}.{}:{}", class_name, method_name, method_type));
            }
            cp::Tag::Fieldref(ref field_tag) => {
                tag_string = "Fieldref";
                arg_string = format!("#{}.#{}", field_tag.class_index, field_tag.name_and_type_index);
                let class_info = opts.constant_pool[field_tag.class_index]
                    .as_class().unwrap();
                let method_info = opts.constant_pool[field_tag.name_and_type_index]
                    .as_name_and_type().unwrap();
                let class_name = opts.constant_pool[class_info.name_index]
                    .as_utf8().unwrap();
                let method_name = opts.constant_pool[method_info.name_index]
                    .as_utf8().unwrap();
                let method_type = opts.constant_pool[method_info.descriptor_index]
                    .as_utf8().unwrap();
                comment_string = Some(format!("{}.{}:{}", class_name, method_name, method_type));
            }
            cp::Tag::String(ref string_tag) => {
                tag_string = "String";
                arg_string = format!("#{}", string_tag.string_index);
                let string = opts.constant_pool[string_tag.string_index].as_utf8().unwrap();
                comment_string = Some(format!("{}", string));
            }
            cp::Tag::Class(ref class_tag) => {
                tag_string = "Class";
                arg_string = format!("#{}", class_tag.name_index);
                let class_name = opts.constant_pool[class_tag.name_index]
                    .as_utf8().unwrap();
                comment_string = Some(format!("{}", class_name));
            }
            cp::Tag::Utf8(ref string) => {
                tag_string = "Utf8";
                arg_string = format!("{}", string);
            }
            cp::Tag::NameAndType(cp::NameAndTypeTag { name_index, descriptor_index }) => {
                tag_string = "NameAndType";
                arg_string = format!("#{}:#{}", name_index, descriptor_index);
                let method_name = opts.constant_pool[name_index]
                    .as_utf8().unwrap();
                let method_type = opts.constant_pool[descriptor_index]
                    .as_utf8().unwrap();
                comment_string = Some(format!("{}:{}", method_name, method_type));
            }
            _ => { }
        }
        let comment_string = comment_string.map_or(String::new(), |s| format!("// {}", s));
        write!(fmt.out, "{:<19}{:<15}{}", tag_string, arg_string, comment_string);
        Ok(())
    }
}

impl Disassemble for cp::ConstantPool {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        let mut magnitude = 1;
        let mut entries = self.len();
        loop {
            entries = entries / 10;
            if entries == 0 {
                break;
            }
            magnitude = magnitude + 1;
        }
        write!(fmt.out, "Constant pool:\n");
        for (i, tag) in self.iter().enumerate() {
            let index = format!("#{}", i + 1);
            write!(fmt.out, "  {:>1$} = ", index, magnitude + 1);
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
