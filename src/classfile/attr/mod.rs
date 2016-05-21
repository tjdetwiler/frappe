use std::io;
use std::vec::Vec;
use std::ops::Deref;

use classfile::error::*;
use classfile::constant_pool as cp;
use util::*;

pub mod annotation;

mod code;
pub use self::code::*;
mod source_file;
pub use self::source_file::*;
mod inner_classes;
pub use self::inner_classes::*;
mod enclosing_method;
pub use self::enclosing_method::*;
mod source_debug_extension;
pub use self::source_debug_extension::*;
mod bootstrap_methods;
pub use self::bootstrap_methods::*;
mod constant_value;
pub use self::constant_value::*;
mod exceptions;
pub use self::exceptions::*;
mod line_number_table;
pub use self::line_number_table::*;
mod local_variable_table;
pub use self::local_variable_table::*;
mod stack_map_table;
pub use self::stack_map_table::*;
mod signature;
pub use self::signature::*;

#[derive(Debug)]
pub struct Attributes {
    attributes: Vec<AttributeInfo>,
}

impl Attributes {
    pub fn read<T: io::Read>(rdr: &mut T, constant_pool: &cp::ConstantPool) -> Result<Attributes> {
        let attributes_count = try!(read_u16(rdr));
        let mut attributes: Vec<AttributeInfo> = vec![];
        for _ in 0..attributes_count {
            let entry = try!(AttributeInfo::read(rdr, constant_pool));
            attributes.push(entry);
        }
        Ok(Attributes { attributes: attributes })
    }
}

impl Deref for Attributes {
    type Target = Vec<AttributeInfo>;

    fn deref(&self) -> &Vec<AttributeInfo> {
        &self.attributes
    }
}

#[derive(Debug)]
pub enum AttributeInfo {
    SourceFile(Box<SourceFileAttribute>),
    InnerClasses(Box<InnerClassesAttribute>),
    EnclosingMethod(Box<EnclosingMethodAttribute>),
    SourceDebugExtension(Box<SourceDebugExtensionAttribute>),
    BootstrapMethods(Box<BootstrapMethodsAttribute>),
    ConstantValue(Box<ConstantValueAttribute>),
    Code(Box<CodeAttribute>),
    Exceptions(Box<ExceptionsAttribute>),
    LineNumberTable(Box<LineNumberTableAttribute>),
    LocalVariableTable(Box<LocalVariableTableAttribute>),
    StackMapTable(Box<StackMapTableAttribute>),
    Synthetic,
    Signature(Box<SignatureAttribute>),
    Raw(Box<Vec<u8>>),
}

impl AttributeInfo {
    pub fn read<T: io::Read>(rdr: &mut T,
                             constant_pool: &cp::ConstantPool)
                             -> Result<AttributeInfo> {
        let name_index = try!(read_u16(rdr));
        if let cp::Tag::Utf8(ref attribute_name) = constant_pool[name_index] {
            AttributeInfo::read_by_name(rdr, attribute_name, constant_pool)
        } else {
            Err(Error::IOError)
        }
    }

    fn read_by_name<T: io::Read>(rdr: &mut T,
                                 name: &str,
                                 constant_pool: &cp::ConstantPool)
                                 -> Result<AttributeInfo> {
        let attribute_length = try!(read_u32(rdr));
        match name {
            "SourceFile" => {
                let source_file = try!(SourceFileAttribute::read(rdr));
                Ok(AttributeInfo::SourceFile(Box::new(source_file)))
            }
            "InnerClasses" => {
                let inner_classes = try!(InnerClassesAttribute::read(rdr));
                Ok(AttributeInfo::InnerClasses(Box::new(inner_classes)))
            }
            "EnclosingMethod" => {
                let enclosing_method = try!(EnclosingMethodAttribute::read(rdr));
                Ok(AttributeInfo::EnclosingMethod(Box::new(enclosing_method)))
            }
            "SourceDebugExtension" => {
                let source_debug_extension =
                    try!(SourceDebugExtensionAttribute::read(rdr, attribute_length));
                Ok(AttributeInfo::SourceDebugExtension(Box::new(source_debug_extension)))
            }
            "BootstrapMethods" => {
                let bootstrap_methods = try!(BootstrapMethodsAttribute::read(rdr));
                Ok(AttributeInfo::BootstrapMethods(Box::new(bootstrap_methods)))
            }
            "ConstantValue" => {
                let constant_value = try!(ConstantValueAttribute::read(rdr));
                Ok(AttributeInfo::ConstantValue(Box::new(constant_value)))
            }
            "Code" => {
                let code = try!(CodeAttribute::read(rdr, constant_pool));
                Ok(AttributeInfo::Code(Box::new(code)))
            }
            "Exceptions" => {
                let exceptions = try!(ExceptionsAttribute::read(rdr));
                Ok(AttributeInfo::Exceptions(Box::new(exceptions)))
            }
            "LineNumberTable" => {
                let line_number_table = try!(LineNumberTableAttribute::read(rdr));
                Ok(AttributeInfo::LineNumberTable(Box::new(line_number_table)))
            }
            "LocalVariableTable" => {
                let local_variable_table = try!(LocalVariableTableAttribute::read(rdr));
                Ok(AttributeInfo::LocalVariableTable(Box::new(local_variable_table)))
            }
            "StackMapTable" => {
                let stack_map_table = try!(StackMapTableAttribute::read(rdr));
                Ok(AttributeInfo::StackMapTable(Box::new(stack_map_table)))
            }
            "Synthetic" => Ok(AttributeInfo::Synthetic),
            "Signature" => {
                let signature = try!(SignatureAttribute::read(rdr));
                Ok(AttributeInfo::Signature(Box::new(signature)))
            }
            attr_name => {
                println!("UNKNOWN ATTRIBUTE {}", attr_name);
                let mut info: Vec<u8> = vec![];
                for _ in 0..attribute_length {
                    let byte = try!(read_u8(rdr));
                    info.push(byte);
                }
                Ok(AttributeInfo::Raw(Box::new(info)))
            }
        }
    }
}
