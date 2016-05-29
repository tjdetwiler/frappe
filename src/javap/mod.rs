use std::io;

use classfile::*;

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
    pub constants: &'a ConstantPool,
}

pub trait Disassemble {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()>;
}

impl Disassemble for ClassFile {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        if let Some(source_file) = self.attrs.source_file(&self.constants) {
            try!(write!(fmt.out, "  Compiled from \"{}\"\n", source_file));
        }
        let class_name = self.this_class_name();
        let class_name = class_name.replace("/", ".");
        let access_mode = if self.access_flags.is_public() {
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
        try!(write!(fmt.out, "{}{} {}", access_mode, class_type, class_name));
        if let Some(super_class_name) = self.super_class_name() {
            if !(super_class_name == "java/lang/Object") {
                let super_class_name = super_class_name.replace("/", ".");
                try!(write!(fmt.out, " extends {} ", super_class_name));
            }
        }
        if opts.verbose {
            try!(write!(fmt.out, "\n"));
            try!(write!(fmt.out, "  minor version: {}\n", self.minor_version));
            try!(write!(fmt.out, "  major version: {}\n", self.major_version));
            try!(write!(fmt.out, "  flags: {}\n", self.access_flags));
            try!(self.constants.pretty_print(fmt, opts));
        }
        try!(write!(fmt.out, "{{\n"));
        try!(self.methods.pretty_print(fmt, opts));
        try!(write!(fmt.out, "}}"));
        Ok(())
    }
}

fn generate_typed_entity_comment_string(cp: &ConstantPool, entity: &TypedEntityConstant) -> String {
    let class_info = cp[entity.class_index].as_class();
    let class_name = cp[class_info].as_utf8();
    let entity_info = cp[entity.name_and_type_index].as_name_and_type();
    let method_name = cp[entity_info.name_index].as_utf8();
    let method_name = match method_name.as_ref() {
        "<init>" | "<clinit>" => format!("\"{}\"", method_name),
        _ => format!("{}", method_name),
    };
    let method_type = cp[entity_info.descriptor_index].as_utf8();
    format!("{}.{}:{}", class_name, method_name, method_type)

}

impl Disassemble for Constant {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        let mut tag_string = "";
        let mut arg_string = String::new();
        let mut comment_string: Option<String> = None;
        match *self {
            Constant::Methodref(ref method_tag) => {
                tag_string = "Methodref";
                arg_string = format!("#{}.#{}",
                                     method_tag.class_index,
                                     method_tag.name_and_type_index);
                comment_string = Some(generate_typed_entity_comment_string(opts.constants,
                                                                           method_tag));
            }
            Constant::Fieldref(ref field_tag) => {
                tag_string = "Fieldref";
                arg_string = format!("#{}.#{}",
                                     field_tag.class_index,
                                     field_tag.name_and_type_index);
                comment_string = Some(generate_typed_entity_comment_string(opts.constants,
                                                                           field_tag));
            }
            Constant::InterfaceMethodref(ref method_tag) => {
                tag_string = "InterfaceMethodref";
                arg_string = format!("#{}.#{}",
                                     method_tag.class_index,
                                     method_tag.name_and_type_index);
                comment_string = Some(generate_typed_entity_comment_string(opts.constants,
                                                                           method_tag));
            }
            Constant::String(string_index) => {
                tag_string = "String";
                arg_string = format!("#{}", string_index);
                let string = opts.constants[string_index].as_utf8();
                comment_string = Some(format!("{}", string));
            }
            Constant::Class(name_index) => {
                tag_string = "Class";
                arg_string = format!("#{}", name_index);
                let class_name = opts.constants[name_index].as_utf8();
                comment_string = Some(format!("{}", class_name));
            }
            Constant::Utf8(ref string) => {
                tag_string = "Utf8";
                arg_string = format!("{}", string);
            }
            Constant::NameAndType(NameAndTypeConstant { name_index, descriptor_index }) => {
                tag_string = "NameAndType";
                arg_string = format!("#{}:#{}", name_index, descriptor_index);
                let method_name = opts.constants[name_index].as_utf8();
                let method_type = opts.constants[descriptor_index].as_utf8();
                comment_string = Some(format!("{}:{}", method_name, method_type));
            }
            Constant::Integer(val) => {
                tag_string = "Integer";
                arg_string = format!("{}", val);
            }
            Constant::Long(val) => {
                tag_string = "Long";
                arg_string = format!("{}l", val);
            }
            Constant::Float(val) => {
                tag_string = "Float";
                let sign_string = if val.is_sign_negative() {
                    "-"
                } else {
                    ""
                };
                if val.is_nan() {
                    arg_string = format!("{}NaN", sign_string);
                } else if val.is_infinite() {
                    arg_string = format!("{}Infinity", sign_string);
                } else {
                    arg_string = format!("{:.8E}", val);
                }
                arg_string = format!("{}f", arg_string);
            }
            Constant::Double(val) => {
                tag_string = "Double";
                let sign_string = if val.is_sign_negative() {
                    "-"
                } else {
                    ""
                };
                if val.is_nan() {
                    arg_string = format!("{}NaN", sign_string);
                } else if val.is_infinite() {
                    arg_string = format!("{}Infinity", sign_string);
                } else {
                    arg_string = format!("{:.16E}", val);
                }
                arg_string = format!("{}d", arg_string);
            }
            _ => {}
        }
        let comment_string = comment_string.map_or(String::new(), |s| format!("// {}", s));
        try!(write!(fmt.out,
                    "{:<19}{:<15}{}",
                    tag_string,
                    arg_string,
                    comment_string));
        Ok(())
    }
}

impl Disassemble for ConstantPool {
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
        try!(write!(fmt.out, "Constant pool:\n"));
        for (i, tag) in self.iter().enumerate() {
            if let Constant::Skip = *tag {
                continue;
            }
            let index = format!("#{}", i + 1);
            try!(write!(fmt.out, "  {:>1$} = ", index, magnitude + 1));
            try!(tag.pretty_print(fmt, opts));
            try!(write!(fmt.out, "\n"));
        }
        Ok(())
    }
}

impl Disassemble for Vec<MethodInfo> {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        for method in self.iter() {
            try!(method.pretty_print(fmt, opts));
            try!(write!(fmt.out, "\n"));
        }
        Ok(())
    }
}

impl Disassemble for Attributes {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        for attr in self.iter() {
            try!(attr.pretty_print(fmt, opts));
            try!(write!(fmt.out, "\n"));
        }
        Ok(())
    }
}

impl Disassemble for AttributeInfo {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        match *self {
            AttributeInfo::SourceFile(_) => {
                try!(write!(fmt.out, "SourceFile"));
            }
            AttributeInfo::AnnotationDefault(ref element_value) => {
                try!(write!(fmt.out, "AnnotationDefault:\n"));
                try!(write!(fmt.out, "  default_value: "));
                try!(element_value.pretty_print(fmt, opts));
                try!(write!(fmt.out, "\n"));
            }
            AttributeInfo::Code(ref code) => {
                try!(code.pretty_print(fmt, opts));
            }
            _ => {
                try!(write!(fmt.out, "Other"));
            }
        }
        Ok(())
    }
}

impl Disassemble for MethodInfo {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        try!(write!(fmt.out, "  "));
        let access_mode = if self.access_flags.is_public() {
            "public"
        } else if self.access_flags.is_private() {
            "private"
        } else if self.access_flags.is_protected() {
            "protected"
        } else {
            // package access
            ""
        };
        let scope = if self.access_flags.is_static() {
            " static"
        } else {
            ""
        };
        let method_name = opts.constants[self.name_index].as_utf8();
        try!(write!(fmt.out, "{}{} {};\n", access_mode, scope, method_name));
        let method_descriptor = opts.constants[self.descriptor_index].as_utf8();
        if opts.verbose {
            try!(write!(fmt.out, "    descriptor: {}\n", method_descriptor));
            try!(write!(fmt.out, "    flags: {:?}\n", self.access_flags));
            for attr in self.attrs.iter() {
                try!(attr.pretty_print(fmt, opts));
            }
        }
        Ok(())
    }
}

impl Disassemble for ElementValue {
    fn pretty_print(&self, fmt: &mut Formatter, _: &Options) -> io::Result<()> {
        match *self {
            ElementValue::ConstantValue(ref constant_value) => {
                try!(write!(fmt.out,
                            "{}#{}",
                            constant_value.tag as char,
                            constant_value.const_value_index));
            }
            _ => {
                try!(write!(fmt.out, "Unsupported ElementValue!"));
            }
        }
        Ok(())
    }
}

impl Disassemble for CodeAttribute {
    fn pretty_print(&self, fmt: &mut Formatter, _: &Options) -> io::Result<()> {
        try!(write!(fmt.out,
                    "      stack={}, locals={}, args_size={}\n",
                    self.max_stack,
                    self.max_locals,
                    "TODO!"));
        Ok(())
    }
}
