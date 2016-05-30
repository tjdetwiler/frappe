use std::io;
use std::io::{Error, ErrorKind};

use classfile::*;
use bytecode::*;

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
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        try!(write!(fmt.out,
                    "      stack={}, locals={}, args_size={}\n",
                    self.max_stack,
                    self.max_locals,
                    "TODO!"));
        let length = self.code.len();
        let mut pc: usize = 0;
        while pc < length {
            let result = Bytecode::decode(&self.code[pc..], pc);
            try!(write!(fmt.out, "{:>8}: ", pc));
            try!(result.bytecode.pretty_print(fmt, opts));
            try!(write!(fmt.out, "\n"));
            pc += result.consumed;
        }
        Ok(())
    }
}

impl Disassemble for Bytecode {
    fn pretty_print(&self, fmt: &mut Formatter, opts: &Options) -> io::Result<()> {
        let mut arg_string = String::new();
        let mut comment_string = String::new();
        let op_string = match *self {
            Bytecode::nop => format!("nop"),
            Bytecode::aconst_null => format!("aconst_null"),
            Bytecode::iconst_i(i) if i < 0 => format!("iconst_m{}", i),
            Bytecode::iconst_i(i) => format!("iconst_{}", i),
            Bytecode::lconst_l(l) => format!("lconst_{}", l),
            Bytecode::fconst_f(f) => format!("fconst_{}", f),
            Bytecode::dconst_d(d) => format!("dconst_{}", d),
            Bytecode::bipush { .. } => format!("bipush"),
            Bytecode::sipush { .. }=> format!("sipush"),
            Bytecode::ldc { .. }=> format!("ldc"),
            Bytecode::ldc_w { .. }=> format!("ldc_w"),
            Bytecode::ldc2_w { .. }=> format!("ldc2_w"),
            Bytecode::iload { .. } => format!("iload"),
            Bytecode::lload { .. } => format!("lload"),
            Bytecode::fload { .. } => format!("fload"),
            Bytecode::dload { .. } => format!("dload"),
            Bytecode::aload { .. } => format!("aload"),
            Bytecode::iload_n(n) => format!("iload_{}", n),
            Bytecode::lload_n(n) => format!("lload_{}", n),
            Bytecode::fload_n(n) => format!("fload_{}", n),
            Bytecode::dload_n(n) => format!("dload_{}", n),
            Bytecode::aload_n(n) => format!("aload_{}", n),
            Bytecode::iaload => format!("iaload"),
            Bytecode::laload => format!("laload"),
            Bytecode::faload => format!("faload"),
            Bytecode::daload => format!("daload"),
            Bytecode::aaload => format!("aaload"),
            Bytecode::baload => format!("baload"),
            Bytecode::caload => format!("caload"),
            Bytecode::saload => format!("sastore"),
            Bytecode::istore => format!("istore"),
            Bytecode::lstore => format!("lstore"),
            Bytecode::fstore { .. } => format!("fstore"),
            Bytecode::dstore { .. } => format!("dstore"),
            Bytecode::astore { .. } => format!("astore"),
            Bytecode::istore_n(n) => format!("istore_{}", n),
            Bytecode::lstore_n(n) => format!("lstore_{}", n),
            Bytecode::fstore_n(n) => format!("fstore_{}", n),
            Bytecode::dstore_n(n) => format!("dstore_{}", n),
            Bytecode::astore_n(n) => format!("astore_{}", n),
            Bytecode::iastore => format!("iastore"),
            Bytecode::lastore => format!("lastore"),
            Bytecode::fastore => format!("fastore"),
            Bytecode::dastore => format!("dastore"),
            Bytecode::aastore => format!("aastore"),
            Bytecode::bastore => format!("bastore"),
            Bytecode::castore => format!("castore"),
            Bytecode::sastore => format!("sastore"),
            Bytecode::pop => format!("pop"),
            Bytecode::pop2 => format!("pop2"),
            Bytecode::dup => format!("dup"),
            Bytecode::dup_x1 => format!("dup_x1"),
            Bytecode::dup_x2 => format!("dup_x2"),
            Bytecode::dup2 => format!("dup2"),
            Bytecode::dup2_x1 => format!("dup2_x1"),
            Bytecode::dup2_x2 => format!("dup2_x2"),
            Bytecode::swap => format!("swap"),
            Bytecode::iadd => format!("iadd"),
            Bytecode::ladd => format!("ladd"),
            Bytecode::fadd => format!("fadd"),
            Bytecode::dadd => format!("dadd"),
            Bytecode::isub => format!("isub"),
            Bytecode::lsub => format!("lsub"),
            Bytecode::fsub => format!("fsub"),
            Bytecode::dsub => format!("dsub"),
            Bytecode::imul => format!("imul"),
            Bytecode::lmul => format!("lmul"),
            Bytecode::fmul => format!("fmul"),
            Bytecode::dmul => format!("dmul"),
            Bytecode::idiv => format!("idiv"),
            Bytecode::ldiv => format!("ldiv"),
            Bytecode::fdiv => format!("fdiv"),
            Bytecode::ddiv => format!("ddiv"),
            Bytecode::irem => format!("irem"),
            Bytecode::lrem => format!("lrem"),
            Bytecode::frem => format!("frem"),
            Bytecode::drem => format!("drem"),
            Bytecode::ineg => format!("ineg"),
            Bytecode::lneg => format!("lneg"),
            Bytecode::fneg => format!("fneg"),
            Bytecode::dneg => format!("dneg"),
            Bytecode::ishl => format!("ishl"),
            Bytecode::lshl => format!("lshl"),
            Bytecode::ishr => format!("ishr"),
            Bytecode::lshr => format!("lshr"),
            Bytecode::iushr => format!("iushr"),
            Bytecode::lushr => format!("lushr"),
            Bytecode::iand => format!("iand"),
            Bytecode::land => format!("land"),
            Bytecode::ior => format!("ior"),
            Bytecode::lor => format!("lor"),
            Bytecode::ixor => format!("ixor"),
            Bytecode::lxor => format!("lxor"),
            Bytecode::iinc { .. } => format!("iinc"),
            Bytecode::i2l => format!("i2l"),
            Bytecode::i2f => format!("i2f"),
            Bytecode::i2d => format!("i2d"),
            Bytecode::l2i => format!("l2i"),
            Bytecode::l2f => format!("l2f"),
            Bytecode::l2d => format!("l2d"),
            Bytecode::f2i => format!("f2i"),
            Bytecode::f2l => format!("f2l"),
            Bytecode::f2d => format!("f2d"),
            Bytecode::d2i => format!("d2i"),
            Bytecode::d2l => format!("d2l"),
            Bytecode::d2f => format!("d2f"),
            Bytecode::i2b => format!("i2b"),
            Bytecode::i2c => format!("i2c"),
            Bytecode::i2s => format!("i2s"),
            Bytecode::lcmp => format!("lcmp"),
            Bytecode::fcmpl => format!("fcmpl"),
            Bytecode::fcmpg => format!("fcmpg"),
            Bytecode::dcmpl => format!("dcmpl"),
            Bytecode::dcmpg => format!("dcmpg"),
            Bytecode::ifeq { .. } => format!("ifeq"),
            Bytecode::ifne { .. } => format!("ifne"),
            Bytecode::iflt { .. } => format!("iflt"),
            Bytecode::ifge { .. } => format!("ifge"),
            Bytecode::ifgt { .. } => format!("ifgt"),
            Bytecode::ifle { .. } => format!("ifle"),
            Bytecode::if_icmpeq { .. } => format!("if_icmpeq"),
            Bytecode::if_icmpne { .. } => format!("if_icmpne"),
            Bytecode::if_icmplt { .. } => format!("if_icmplt"),
            Bytecode::if_icmpge { .. } => format!("if_icmpge"),
            Bytecode::if_icmpgt { .. } => format!("if_icmpgt"),
            Bytecode::if_icmple { .. } => format!("if_icmple"),
            Bytecode::if_acmpeq { .. } => format!("if_acmpeq"),
            Bytecode::if_acmpne { .. } => format!("if_acmpne"),
            Bytecode::goto { .. } => format!("goto"),
            Bytecode::jsr { .. } => format!("jsr"),
            Bytecode::ret { .. } => format!("ret"),
            Bytecode::tableswitch { .. } => format!("tableswitch"),
            Bytecode::lookupswitch { .. } => format!("lookupswitch"),
            Bytecode::ireturn => format!("ireturn"),
            Bytecode::lreturn => format!("lreturn"),
            Bytecode::freturn => format!("freturn"),
            Bytecode::dreturn => format!("dreturn"),
            Bytecode::areturn => format!("areturn"),
            Bytecode::Return => format!("return"),
            Bytecode::getstatic { .. } => format!("getstatic"),
            Bytecode::putstatic { .. } => format!("putstatic"),
            Bytecode::getfield { .. } => format!("getfield"),
            Bytecode::putfield { .. } => format!("putfield"),
            Bytecode::invokevirtual { .. } => format!("invokevirtual"),
            Bytecode::invokespecial { index } => {
                arg_string = format!("#{}", index); 
                let entity = &match opts.constants[index] {
                    Constant::Methodref(ref entity) => entity,
                    Constant::InterfaceMethodref(ref entity) => entity,
                    _ => {
                        return Err(Error::new(ErrorKind::Other, "Invalid index value"));
                    }
                };
                let descriptor = generate_typed_entity_comment_string(opts.constants, entity);
                comment_string = format!("Method {}", descriptor);
                format!("invokespecial")
            },
            Bytecode::invokestatic { .. } => format!("invokestatic"),
            Bytecode::invokeinterface { .. } => format!("invokeinterface"),
            Bytecode::invokedynamic { .. } => format!("invokedynamic"),
            Bytecode::new { .. } => format!("new"),
            Bytecode::newarray { .. } => format!("newarray"),
            Bytecode::anewarray { .. } => format!("anewarray"),
            Bytecode::arraylength => format!("arraylength"),
            Bytecode::athrow => format!("athrow"),
            Bytecode::checkcast { .. } => format!("checkcast"),
            Bytecode::instanceof { .. } => format!("instanceof"),
            Bytecode::monitorenter => format!("monitorenter"),
            Bytecode::monitorexit => format!("monitorexit"),
            Bytecode::wide_iload { .. } => format!("wide_iload"),
            Bytecode::wide_lload { .. } => format!("wide_lload"),
            Bytecode::wide_fload { .. } => format!("wide_fload"),
            Bytecode::wide_dload { .. } => format!("wide_dload"),
            Bytecode::wide_aload { .. } => format!("wide_aload"),
            Bytecode::wide_istore { .. } => format!("wide_istore"),
            Bytecode::wide_lstore { .. } => format!("wide_lstore"),
            Bytecode::wide_fstore { .. } => format!("wide_fstore"),
            Bytecode::wide_dstore { .. } => format!("wide_dstore"),
            Bytecode::wide_astore { .. } => format!("wide_astore"),
            Bytecode::wide_iinc { .. } => format!("wide_iinc"),
            Bytecode::wide_ret { .. } => format!("wide_ret"),
            Bytecode::multianewarray { .. } => format!("multianewarray"),
            Bytecode::ifnull { .. } => format!("ifnull"),
            Bytecode::ifnonnull { .. } => format!("ifnonnull"),
            Bytecode::goto_w { .. } => format!("goto_w"),
            Bytecode::jsr_w { .. } => format!("jsr_w"),
            Bytecode::invalid(op) => format!("invalid {}", op),
        };
        try!(write!(fmt.out, "{:<14}{:<20}// {}", op_string, arg_string, comment_string));
        Ok(())
    }
}
