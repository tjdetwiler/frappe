use std::io;
use std::vec::Vec;
use std::ops::Deref;

use classfile::error::*;
use classfile::constant_pool as cp;
use util::*;

mod code;
pub use self::code::*;
mod source_file;
pub use self::source_file::*;
mod inner_classes;
pub use self::inner_classes::*;
mod enclosing_method;
pub use self::enclosing_method::*;

#[derive(Debug)]
pub struct Attributes {
    attributes: Vec<AttributeInfo>,
}

impl Attributes {
    pub fn read<T: io::Read>(rdr: &mut T,
                             constant_pool: &cp::ConstantPool)
                             -> Result<Attributes> {
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
    Code(Box<CodeAttribute>),
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
            "Code" => {
                let code = try!(CodeAttribute::read(rdr, constant_pool));
                Ok(AttributeInfo::Code(Box::new(code)))
            }
            "InnerClasses" => {
                let inner_classes = try!(InnerClassesAttribute::read(rdr));
                Ok(AttributeInfo::InnerClasses(Box::new(inner_classes)))
            }
            "EnclosingMethod" => {
                let enclosing_method = try!(EnclosingMethodAttribute::read(rdr));
                Ok(AttributeInfo::EnclosingMethod(Box::new(enclosing_method)))
            }
            _ => {
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
