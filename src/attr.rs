use std::io;
use std::vec::Vec;
use std::ops::Deref;

use constant_pool::{ConstantPool, ConstantPoolTag};
use util::*;

pub struct SourceFile {
    pub sourcefile_index: u16
}

/// Defines an entry in the `exception_table` of a `Code` attribute.
pub struct ExceptionInfo {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16
}

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
    pub fn read<T: io::Read>(rdr: &mut T) -> io::Result<Attributes> {
        let attributes_count = try!(read_u16(rdr));
        let mut attributes: Vec<AttributeInfo> = vec![];
        for _ in 0..attributes_count {
            let entry = try!(AttributeInfo::read(rdr));
            attributes.push(entry);
        }
        Ok(Attributes {
            attributes: attributes
        })
    }

    pub fn with_name<'a>(&'a self, name: &str, constant_pool: &ConstantPool) -> Vec<&AttributeInfo> {
        let expected_tag = ConstantPoolTag::Utf8(name.into());
        self.attributes
            .iter()
            .filter(
                |&attr| constant_pool[attr.attribute_name_index] == expected_tag)
            .collect()
    }
}

impl Deref for Attributes {
    type Target = Vec<AttributeInfo>;

    fn deref(&self) -> &Vec<AttributeInfo> {
        &self.attributes
    }
}

#[derive(Debug)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub info: Vec<u8>
}

impl AttributeInfo {
    pub fn read<T: io::Read>(rdr: &mut T) -> io::Result<AttributeInfo> {
        let attribute_name_index = try!(read_u16(rdr));
        let attribute_length = try!(read_u32(rdr));
        let mut info: Vec<u8> = vec![];
        for _ in 0..attribute_length {
            let byte = try!(read_u8(rdr));
            info.push(byte);
        }
        Ok(AttributeInfo {
            attribute_name_index: attribute_name_index,
            attribute_length: attribute_length,
            info: info
        })
    }
}

