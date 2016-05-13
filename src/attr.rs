use std::io;
use std::vec::Vec;
use std::ops::Deref;

use constant_pool as cp;
use util::*;

#[derive(Debug)]
pub struct SourceFile {
    pub sourcefile_index: u16
}

/// Defines an entry in the `exception_table` of a `Code` attribute.
#[derive(Debug)]
pub struct ExceptionInfo {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16
}

#[derive(Debug)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionInfo>,
    pub attribute_info: Attributes
}

#[derive(Debug)]
pub struct Attributes {
    attributes: Vec<AttributeInfo>
}

impl Attributes {
    pub fn read<T: io::Read>(rdr: &mut T, constant_pool: &cp::ConstantPool) -> io::Result<Attributes> {
        let attributes_count = try!(read_u16(rdr));
        let mut attributes: Vec<AttributeInfo> = vec![];
        for _ in 0..attributes_count {
            let entry = try!(AttributeInfo::read(rdr, constant_pool));
            attributes.push(entry);
        }
        Ok(Attributes {
            attributes: attributes
        })
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
    SourceFile(Box<SourceFile>),
    Code(Box<Code>),
    Raw(Box<Vec<u8>>)
}

impl AttributeInfo {
    pub fn read<T: io::Read>(rdr: &mut T, constant_pool: &cp::ConstantPool) -> io::Result<AttributeInfo> {
        let name_index = try!(read_u16(rdr));
        if let cp::Tag::Utf8(ref attribute_name) = constant_pool[name_index] {
            AttributeInfo::read_by_name(rdr, attribute_name)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "whoops"))
        }
    }

    fn read_by_name<T: io::Read>(rdr: &mut T, name: &str) -> io::Result<AttributeInfo> {
        let attribute_length = try!(read_u32(rdr));
        match name {
            "SourceFile" => {
                let sourcefile_index = try!(read_u16(rdr));
                Ok(AttributeInfo::SourceFile(Box::new(SourceFile{
                    sourcefile_index: sourcefile_index
                })))
            },
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

