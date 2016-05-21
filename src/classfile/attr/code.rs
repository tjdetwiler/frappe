use std::io;
use std::vec::Vec;

use classfile::constant_pool as cp;
use util::*;

use super::Attributes;

/// Defines an entry in the `exception_table` of a `CodeAttribute`.
#[derive(Debug)]
pub struct ExceptionInfo {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl ExceptionInfo {
    pub fn read<T: io::Read>(rdr: &mut T) -> io::Result<ExceptionInfo> {
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
pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionInfo>,
    pub attributes: Attributes,
}

impl CodeAttribute {
    pub fn read<T: io::Read>(rdr: &mut T,
                             constant_pool: &cp::ConstantPool)
                             -> io::Result<CodeAttribute> {
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
        Ok(CodeAttribute {
            max_stack: max_stack,
            max_locals: max_locals,
            code: code,
            exception_table: exception_table,
            attributes: attributes,
        })
    }
}
