use std::io;
use std::vec::Vec;
use std::ops::Deref;

use classfile::constant_pool as cp;
use util::*;

#[derive(Debug)]
pub struct SourceFile {
    pub sourcefile_index: u16,
}

impl SourceFile {
    fn read<T: io::Read>(rdr: &mut T) -> io::Result<SourceFile> {
        let sourcefile_index = try!(read_u16(rdr));
        Ok(SourceFile { sourcefile_index: sourcefile_index })
    }
}

/// Defines an entry in the `exception_table` of a `Code` attribute.
#[derive(Debug)]
pub struct ExceptionInfo {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl ExceptionInfo {
    fn read<T: io::Read>(rdr: &mut T) -> io::Result<ExceptionInfo> {
        let start_pc = try!(read_u16(rdr));
        let end_pc = try!(read_u16(rdr));
        let handler_pc = try!(read_u16(rdr));
        let catch_type = try!(read_u16(rdr));
        Ok(ExceptionInfo {
            start_pc: start_pc,
            end_pc: end_pc,
            handler_pc: handler_pc,
            catch_type: catch_type,
        })
    }
}

#[derive(Debug)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionInfo>,
    pub attributes: Attributes,
}

impl Code {
    fn read<T: io::Read>(rdr: &mut T, constant_pool: &cp::ConstantPool) -> io::Result<Code> {
        let max_stack = try!(read_u16(rdr));
        let max_locals = try!(read_u16(rdr));
        let code_length = try!(read_u32(rdr));
        let mut code: Vec<u8> = vec![];
        for _ in 0..code_length {
            let byte = try!(read_u8(rdr));
            code.push(byte);
        }
        let exception_table_length = try!(read_u16(rdr));
        let mut exception_table: Vec<ExceptionInfo> = vec![];
        for _ in 0..exception_table_length {
            let exception_info = try!(ExceptionInfo::read(rdr));
            exception_table.push(exception_info);
        }
        let attributes = try!(Attributes::read(rdr, constant_pool));
        Ok(Code {
            max_stack: max_stack,
            max_locals: max_locals,
            code: code,
            exception_table: exception_table,
            attributes: attributes,
        })
    }
}

#[derive(Debug)]
pub struct Attributes {
    attributes: Vec<AttributeInfo>,
}

impl Attributes {
    pub fn read<T: io::Read>(rdr: &mut T,
                             constant_pool: &cp::ConstantPool)
                             -> io::Result<Attributes> {
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
    SourceFile(Box<SourceFile>),
    Code(Box<Code>),
    Raw(Box<Vec<u8>>),
}

impl AttributeInfo {
    pub fn read<T: io::Read>(rdr: &mut T,
                             constant_pool: &cp::ConstantPool)
                             -> io::Result<AttributeInfo> {
        let name_index = try!(read_u16(rdr));
        if let cp::Tag::Utf8(ref attribute_name) = constant_pool[name_index] {
            AttributeInfo::read_by_name(rdr, attribute_name, constant_pool)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "whoops"))
        }
    }

    fn read_by_name<T: io::Read>(rdr: &mut T,
                                 name: &str,
                                 constant_pool: &cp::ConstantPool)
                                 -> io::Result<AttributeInfo> {
        let attribute_length = try!(read_u32(rdr));
        match name {
            "SourceFile" => {
                let source_file = try!(SourceFile::read(rdr));
                Ok(AttributeInfo::SourceFile(Box::new(source_file)))
            }
            "Code" => {
                let code = try!(Code::read(rdr, constant_pool));
                Ok(AttributeInfo::Code(Box::new(code)))
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
