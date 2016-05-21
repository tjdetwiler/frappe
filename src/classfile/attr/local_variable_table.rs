use std::io;

use util::*;
use classfile::error::Result;

#[derive(Debug)]
pub struct LocalVariableTableEntry {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

impl LocalVariableTableEntry {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<LocalVariableTableEntry> {
        let start_pc = try!(read_u16(rdr));
        let length = try!(read_u16(rdr));
        let name_index = try!(read_u16(rdr));
        let descriptor_index = try!(read_u16(rdr));
        let index = try!(read_u16(rdr));
        Ok(LocalVariableTableEntry {
            start_pc: start_pc,
            length: length,
            name_index: name_index,
            descriptor_index: descriptor_index,
            index: index,
        })
    }
}

#[derive(Debug)]
pub struct LocalVariableTableAttribute {
    local_variable_table: Vec<LocalVariableTableEntry>,
}

impl LocalVariableTableAttribute {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<LocalVariableTableAttribute> {
        let local_variable_table_length = try!(read_u16(rdr));
        let mut local_variable_table: Vec<LocalVariableTableEntry> = vec![];
        for _ in 0..local_variable_table_length {
            let local_variable_table_entry = try!(LocalVariableTableEntry::read(rdr));
            local_variable_table.push(local_variable_table_entry);
        }
        Ok(LocalVariableTableAttribute { local_variable_table: local_variable_table })
    }
}
