extern crate classfile;

use std::io;
use std::fmt::Display;

use classfile::*;

pub struct Formatter {
    out: Box<io::Write>,
}

impl Formatter {
    pub fn new() -> Formatter {
        Formatter {
            out: Box::new(io::stdout()),
        }
    }

    pub fn with_output<W: 'static + io::Write>(write: W) -> Formatter {
        Formatter {
            out: Box::new(write),
        }
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
                let method_name = match method_name.as_ref() {
                    "<init>" | "<clinit" => format!("\"{}\"", method_name),
                    _ => format!("{}", method_name),
                };
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
        let line = format!("{:<19}{:<15}{}",
                    tag_string,
                    arg_string,
                    comment_string);
        try!(write!(fmt.out, "{}", line.trim()));
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
                try!(write!(fmt.out, "    Code:\n"));
                try!(code.pretty_print(fmt, opts));
            }
            AttributeInfo::LineNumberTable(ref table) => {
                try!(write!(fmt.out, "      LineNumberTable:\n"));
                try!(table.pretty_print(fmt, opts));
            }
            _ => {
                try!(write!(fmt.out, "Other"));
            }
        }
        Ok(())
    }
}

impl Disassemble for Vec<LineNumberTableEntry> {
    fn pretty_print(&self, fmt: &mut Formatter, _: &Options) -> io::Result<()> {
        for entry in self.iter() {
            let line_number = format!("        line {}", entry.line_number);
            try!(write!(fmt.out, "{}: {}\n", line_number, entry.start_pc));
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
            try!(write!(fmt.out, "    flags: "));
            try!(self.access_flags.pretty_print(fmt, opts));
            try!(write!(fmt.out, "\n"));
            for attr in self.attrs.iter() {
                try!(attr.pretty_print(fmt, opts));
            }
        }
        Ok(())
    }
}

impl Disassemble for MethodAccessFlags {
    fn pretty_print(&self, fmt: &mut Formatter, _: &Options) -> io::Result<()> {
        let mut flags: Vec<&str> = vec![];
        if self.is_public() {
            flags.push("ACC_PUBLIC");
        }
        if self.is_private() {
            flags.push("ACC_PRIVATE");
        }
        if self.is_protected() {
            flags.push("ACC_PROTECTED");
        }
        if self.is_static() {
            flags.push("ACC_STATIC");
        }
        if self.is_final() {
            flags.push("ACC_FINAL");
        }
        if self.is_synchronized() {
            flags.push("ACC_SYNCHRONIZED");
        }
        if self.is_bridge() {
            flags.push("ACC_BRIDGE");
        }
        if self.is_varargs() {
            flags.push("ACC_VARARGS");
        }
        if self.is_native() {
            flags.push("ACC_NATIVE");
        }
        if self.is_abstract() {
            flags.push("ACC_ABSTRACT");
        }
        if self.is_strict() {
            flags.push("ACC_STRICT");
        }
        if self.is_synthetic() {
            flags.push("ACC_SYNTHETIC");
        }
        write!(fmt.out, "{}", flags.join(", "))
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
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        try!(write!(fmt.out,
                    "      stack={}, locals={}, args_size={}\n",
                    self.max_stack,
                    self.max_locals,
                    "TODO!"));
        let length = self.code.len();
        let mut pc: usize = 0;
        while pc < length {
            let result = Bytecode::decode(&self.code, pc);
            try!(write!(fmt.out, "{:>10}: ", pc));
            try!(result.bytecode.pretty_print(fmt, opts));
            try!(write!(fmt.out, "\n"));
            pc = result.newpc;
        }
        for attr in self.attrs.iter() {
            try!(attr.pretty_print(fmt, opts));
        }
        Ok(())
    }
}

macro_rules! no_arg_bytecode {
    ($op:expr) => {
        BytecodeFormat {
            op: format!($op),
            arg: None,
            detail: None,
        }
    };
    ($op:ident @ $n:expr) => {
        BytecodeFormat {
            op: format!("{}{}", stringify!($op), $n),
            arg: None,
            detail: None,
        }
    };
}

fn simple_arg<T: Display>(op: &str, value: T) -> BytecodeFormat {
    BytecodeFormat {
        op: format!("{}", op),
        arg: Some(format!("{}", value)),
        detail: None,
    }
}

fn constant_arg_detail(index: u16, opts: &Options) -> Option<String> {
    let cp = opts.constants;
    match cp[index] {
        Constant::Integer(int) => Some(format!("int {}", int)),
        Constant::Long(long) => Some(format!("long {}", long)),
        Constant::Float(float) => Some(format!("float {}", float)),
        Constant::Double(double) => Some(format!("double {}", double)),
        Constant::Class(name_index) => {
            let name = cp[name_index].as_utf8();
            Some(format!("class {}", name))
        },
        Constant::String(string_index) => {
            let name = cp[string_index].as_utf8();
            Some(format!("String {}", name))
        },
        Constant::Fieldref(ref entity) => {
            let entity_info = cp[entity.name_and_type_index].as_name_and_type();
            let field_name = cp[entity_info.name_index].as_utf8();
            let field_type = cp[entity_info.descriptor_index].as_utf8();
            Some(format!("Field {}:{}", field_name, field_type))
        },
        Constant::Methodref(ref entity) => {
            let detail = generate_typed_entity_comment_string(cp, entity);
            Some(format!("Method {}", detail))
        },
        Constant::InterfaceMethodref(ref entity) => {
            let detail = generate_typed_entity_comment_string(cp, entity);
            Some(format!("InterfaceMethod {}", detail))
        },
        Constant::InvokeDynamic { bootstrap_method_attr_index, name_and_type_index } => {
            let entity_info = cp[name_and_type_index].as_name_and_type();
            let method_name = cp[entity_info.name_index].as_utf8();
            let method_name = match method_name.as_ref() {
                "<init>" | "<clinit>" => format!("\"{}\"", method_name),
                _ => format!("{}", method_name),
            };
            let method_type = cp[entity_info.descriptor_index].as_utf8();
            Some(format!("InvokeDynamic #{}:{}:{}",
                         bootstrap_method_attr_index,
                         method_name,
                         method_type))
        },
        ref constant @ _ => panic!(format!("Unimplemented constant {:#?}", constant)),
    }
}

fn constant_arg(op: &str, index: u16, opts: &Options) -> BytecodeFormat {
    BytecodeFormat {
        op: op.into(),
        arg: Some(format!("#{}", index)),
        detail: constant_arg_detail(index, opts),
    }
}

struct BytecodeFormat {
    op: String,
    arg: Option<String>,
    detail: Option<String>,
}

impl Disassemble for Bytecode {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        let format = match *self {
            Bytecode::nop => no_arg_bytecode!("nop"),
            Bytecode::aconst_null => no_arg_bytecode!("aconst_null"),
            Bytecode::iconst_i(i) if i < 0 => no_arg_bytecode!(iconst_m @ -i),
            Bytecode::iconst_i(i) => no_arg_bytecode!(iconst_m @ i),
            Bytecode::lconst_l(l) => no_arg_bytecode!(lconst_ @ l),
            Bytecode::fconst_f(f) => no_arg_bytecode!(fconst_ @ f),
            Bytecode::dconst_d(d) => no_arg_bytecode!(dconst_ @ d),
            Bytecode::bipush { byte } => simple_arg("bipush", byte),
            Bytecode::sipush { short } => simple_arg("sipush", short),
            Bytecode::ldc { index } => constant_arg("ldc", index as u16, opts),
            Bytecode::ldc_w { index } => constant_arg("ldc_w", index, opts),
            Bytecode::ldc2_w { index  }=> constant_arg("ldc2_w", index, opts),
            Bytecode::iload { index } => simple_arg("iload", index),
            Bytecode::lload { index } => simple_arg("lload", index),
            Bytecode::fload { index } => simple_arg("fload", index),
            Bytecode::dload { byte } => simple_arg("dload", byte),
            Bytecode::aload { index } => simple_arg("aload", index),
            Bytecode::iload_n(n) => no_arg_bytecode!(iload_ @ n),
            Bytecode::lload_n(n) => no_arg_bytecode!(lload_ @ n),
            Bytecode::fload_n(n) => no_arg_bytecode!(fload_ @ n),
            Bytecode::dload_n(n) => no_arg_bytecode!(dload_ @ n),
            Bytecode::aload_n(n) => no_arg_bytecode!(aload_ @ n),
            Bytecode::iaload => no_arg_bytecode!("iaload"),
            Bytecode::laload => no_arg_bytecode!("laload"),
            Bytecode::faload => no_arg_bytecode!("faload"),
            Bytecode::daload => no_arg_bytecode!("daload"),
            Bytecode::aaload => no_arg_bytecode!("aaload"),
            Bytecode::baload => no_arg_bytecode!("baload"),
            Bytecode::caload => no_arg_bytecode!("caload"),
            Bytecode::saload => no_arg_bytecode!("sastore"),
            Bytecode::istore { index } => simple_arg("istore", index),
            Bytecode::lstore { index } => simple_arg("lstore", index),
            Bytecode::fstore { index } => simple_arg("fstore", index),
            Bytecode::dstore { index } => simple_arg("dstore", index),
            Bytecode::astore { index } => simple_arg("astore", index),
            Bytecode::istore_n(n) => no_arg_bytecode!(istore_ @ n),
            Bytecode::lstore_n(n) => no_arg_bytecode!(lstore_ @ n),
            Bytecode::fstore_n(n) => no_arg_bytecode!(fstore_ @ n),
            Bytecode::dstore_n(n) => no_arg_bytecode!(dstore_ @ n),
            Bytecode::astore_n(n) => no_arg_bytecode!(astore_ @ n),
            Bytecode::iastore => no_arg_bytecode!("iastore"),
            Bytecode::lastore => no_arg_bytecode!("lastore"),
            Bytecode::fastore => no_arg_bytecode!("fastore"),
            Bytecode::dastore => no_arg_bytecode!("dastore"),
            Bytecode::aastore => no_arg_bytecode!("aastore"),
            Bytecode::bastore => no_arg_bytecode!("bastore"),
            Bytecode::castore => no_arg_bytecode!("castore"),
            Bytecode::sastore => no_arg_bytecode!("sastore"),
            Bytecode::pop => no_arg_bytecode!("pop"),
            Bytecode::pop2 => no_arg_bytecode!("pop2"),
            Bytecode::dup => no_arg_bytecode!("dup"),
            Bytecode::dup_x1 => no_arg_bytecode!("dup_x1"),
            Bytecode::dup_x2 => no_arg_bytecode!("dup_x2"),
            Bytecode::dup2 => no_arg_bytecode!("dup2"),
            Bytecode::dup2_x1 => no_arg_bytecode!("dup2_x1"),
            Bytecode::dup2_x2 => no_arg_bytecode!("dup2_x2"),
            Bytecode::swap => no_arg_bytecode!("swap"),
            Bytecode::iadd => no_arg_bytecode!("iadd"),
            Bytecode::ladd => no_arg_bytecode!("ladd"),
            Bytecode::fadd => no_arg_bytecode!("fadd"),
            Bytecode::dadd => no_arg_bytecode!("dadd"),
            Bytecode::isub => no_arg_bytecode!("isub"),
            Bytecode::lsub => no_arg_bytecode!("lsub"),
            Bytecode::fsub => no_arg_bytecode!("fsub"),
            Bytecode::dsub => no_arg_bytecode!("dsub"),
            Bytecode::imul => no_arg_bytecode!("imul"),
            Bytecode::lmul => no_arg_bytecode!("lmul"),
            Bytecode::fmul => no_arg_bytecode!("fmul"),
            Bytecode::dmul => no_arg_bytecode!("dmul"),
            Bytecode::idiv => no_arg_bytecode!("idiv"),
            Bytecode::ldiv => no_arg_bytecode!("ldiv"),
            Bytecode::fdiv => no_arg_bytecode!("fdiv"),
            Bytecode::ddiv => no_arg_bytecode!("ddiv"),
            Bytecode::irem => no_arg_bytecode!("irem"),
            Bytecode::lrem => no_arg_bytecode!("lrem"),
            Bytecode::frem => no_arg_bytecode!("frem"),
            Bytecode::drem => no_arg_bytecode!("drem"),
            Bytecode::ineg => no_arg_bytecode!("ineg"),
            Bytecode::lneg => no_arg_bytecode!("lneg"),
            Bytecode::fneg => no_arg_bytecode!("fneg"),
            Bytecode::dneg => no_arg_bytecode!("dneg"),
            Bytecode::ishl => no_arg_bytecode!("ishl"),
            Bytecode::lshl => no_arg_bytecode!("lshl"),
            Bytecode::ishr => no_arg_bytecode!("ishr"),
            Bytecode::lshr => no_arg_bytecode!("lshr"),
            Bytecode::iushr => no_arg_bytecode!("iushr"),
            Bytecode::lushr => no_arg_bytecode!("lushr"),
            Bytecode::iand => no_arg_bytecode!("iand"),
            Bytecode::land => no_arg_bytecode!("land"),
            Bytecode::ior => no_arg_bytecode!("ior"),
            Bytecode::lor => no_arg_bytecode!("lor"),
            Bytecode::ixor => no_arg_bytecode!("ixor"),
            Bytecode::lxor => no_arg_bytecode!("lxor"),
            Bytecode::iinc { index, constant } => {
                BytecodeFormat {
                    op: format!("iinc"),
                    arg: Some(format!("{}, {}", index, constant)),
                    detail: None,
                }
            }
            Bytecode::i2l => no_arg_bytecode!("i2l"),
            Bytecode::i2f => no_arg_bytecode!("i2f"),
            Bytecode::i2d => no_arg_bytecode!("i2d"),
            Bytecode::l2i => no_arg_bytecode!("l2i"),
            Bytecode::l2f => no_arg_bytecode!("l2f"),
            Bytecode::l2d => no_arg_bytecode!("l2d"),
            Bytecode::f2i => no_arg_bytecode!("f2i"),
            Bytecode::f2l => no_arg_bytecode!("f2l"),
            Bytecode::f2d => no_arg_bytecode!("f2d"),
            Bytecode::d2i => no_arg_bytecode!("d2i"),
            Bytecode::d2l => no_arg_bytecode!("d2l"),
            Bytecode::d2f => no_arg_bytecode!("d2f"),
            Bytecode::i2b => no_arg_bytecode!("i2b"),
            Bytecode::i2c => no_arg_bytecode!("i2c"),
            Bytecode::i2s => no_arg_bytecode!("i2s"),
            Bytecode::lcmp => no_arg_bytecode!("lcmp"),
            Bytecode::fcmpl => no_arg_bytecode!("fcmpl"),
            Bytecode::fcmpg => no_arg_bytecode!("fcmpg"),
            Bytecode::dcmpl => no_arg_bytecode!("dcmpl"),
            Bytecode::dcmpg => no_arg_bytecode!("dcmpg"),
            Bytecode::ifeq { branchoffset } => simple_arg("ifeq", branchoffset),
            Bytecode::ifne { branchoffset } => simple_arg("ifne", branchoffset),
            Bytecode::iflt { branchoffset } => simple_arg("iflt", branchoffset),
            Bytecode::ifge { branchoffset } => simple_arg("ifge", branchoffset),
            Bytecode::ifgt { branchoffset } => simple_arg("ifgt", branchoffset),
            Bytecode::ifle { branchoffset } => simple_arg("ifle", branchoffset),
            Bytecode::if_icmpeq { branchoffset } => simple_arg("if_icmpeq", branchoffset),
            Bytecode::if_icmpne { branchoffset } => simple_arg("if_icmpne", branchoffset),
            Bytecode::if_icmplt { branchoffset } => simple_arg("if_icmplt", branchoffset),
            Bytecode::if_icmpge { branchoffset } => simple_arg("if_icmpge", branchoffset),
            Bytecode::if_icmpgt { branchoffset } => simple_arg("if_icmpgt", branchoffset),
            Bytecode::if_icmple { branchoffset } => simple_arg("if_icmple", branchoffset),
            Bytecode::if_acmpeq { branchoffset } => simple_arg("if_acmpeq", branchoffset),
            Bytecode::if_acmpne { branchoffset } => simple_arg("if_acmpne", branchoffset),
            Bytecode::goto { branchoffset } => simple_arg("goto", branchoffset),
            Bytecode::jsr { branchoffset } => simple_arg("jsr", branchoffset),
            Bytecode::ret { index } => simple_arg("ret", index),
            Bytecode::tableswitch { .. } => no_arg_bytecode!("tableswitch"),
            Bytecode::lookupswitch { .. } => no_arg_bytecode!("lookupswitch"),
            Bytecode::ireturn => no_arg_bytecode!("ireturn"),
            Bytecode::lreturn => no_arg_bytecode!("lreturn"),
            Bytecode::freturn => no_arg_bytecode!("freturn"),
            Bytecode::dreturn => no_arg_bytecode!("dreturn"),
            Bytecode::areturn => no_arg_bytecode!("areturn"),
            Bytecode::Return => no_arg_bytecode!("return"),
            Bytecode::getstatic { index } => constant_arg("getstatic", index, opts),
            Bytecode::putstatic { index } => constant_arg("putstatic", index, opts),
            Bytecode::getfield { index } => constant_arg("getfield", index, opts),
            Bytecode::putfield { index } => constant_arg("putfield", index, opts),
            Bytecode::invokevirtual { index } => constant_arg("invokevirtual", index, opts),
            Bytecode::invokespecial { index } => constant_arg("invokespecial", index, opts),
            Bytecode::invokestatic { index } => constant_arg("invokestatic", index, opts),
            Bytecode::invokeinterface { index, count } => {
                BytecodeFormat {
                    op: "invokeinterface".into(),
                    arg: Some(format!("#{}, {}", index, count)),
                    detail: constant_arg_detail(index, opts),
                }
            },
            Bytecode::invokedynamic { index } => {
                BytecodeFormat {
                    op: "invokedynamic".into(),
                    arg: Some(format!("#{},  {}", index, 0)),
                    detail: constant_arg_detail(index, opts),
                }
            },
            Bytecode::new { index } => constant_arg("new", index, opts),
            Bytecode::newarray { atype } => simple_arg("newarray", atype),
            Bytecode::anewarray { index } => constant_arg("anewarray", index, opts),
            Bytecode::arraylength => no_arg_bytecode!("arraylength"),
            Bytecode::athrow => no_arg_bytecode!("athrow"),
            Bytecode::checkcast { index } => constant_arg("checkcast", index, opts),
            Bytecode::instanceof { index } => constant_arg("instanceof", index, opts),
            Bytecode::monitorenter => no_arg_bytecode!("monitorenter"),
            Bytecode::monitorexit => no_arg_bytecode!("monitorexit"),
            Bytecode::wide_iload { index } => simple_arg("wide_iload", index),
            Bytecode::wide_lload { index } => simple_arg("wide_lload", index),
            Bytecode::wide_fload { index } => simple_arg("wide_fload", index),
            Bytecode::wide_dload { index } => simple_arg("wide_dload", index),
            Bytecode::wide_aload { index } => simple_arg("wide_aload", index),
            Bytecode::wide_istore { index } => simple_arg("wide_istore", index),
            Bytecode::wide_lstore { index } => simple_arg("wide_lstore", index),
            Bytecode::wide_fstore { index } => simple_arg("wide_fstore", index),
            Bytecode::wide_dstore { index } => simple_arg("wide_dstore", index),
            Bytecode::wide_astore { index } => simple_arg("wide_astore", index),
            Bytecode::wide_iinc { .. } => no_arg_bytecode!("wide_iinc"),
            Bytecode::wide_ret { .. } => no_arg_bytecode!("wide_ret"),
            Bytecode::multianewarray { .. } => no_arg_bytecode!("multianewarray"),
            Bytecode::ifnull { .. } => no_arg_bytecode!("ifnull"),
            Bytecode::ifnonnull { .. } => no_arg_bytecode!("ifnonnull"),
            Bytecode::goto_w { .. } => no_arg_bytecode!("goto_w"),
            Bytecode::jsr_w { .. } => no_arg_bytecode!("jsr_w"),
            Bytecode::invalid(op) => no_arg_bytecode!(invalid_ @ op),
        };
        let arg_string = format.arg.unwrap_or(String::new());
        let comment_string = format.detail.map_or(String::new(), |s| format!("// {}", s));
        let line = format!("{:<14}{:<20}{}", format.op, arg_string, comment_string);
        try!(write!(fmt.out, "{}", line.trim()));
        Ok(())
    }
}
