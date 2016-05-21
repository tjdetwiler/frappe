use std::io;

use util::*;
use classfile::error::Result;

#[derive(Debug)]
pub struct LocalVariableTypeTableAttribute {
    pub local_variable_type_table: Vec<LocalVariableTypeTableEntry>,
}

impl LocalVariableTypeTableAttribute {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<LocalVariableTypeTableAttribute> {
        let local_variable_type_table_length = try!(read_u16(rdr));
        let mut local_variable_type_table: Vec<LocalVariableTypeTableEntry> = vec![];
        for _ in 0..local_variable_type_table_length {
            let entry = try!(LocalVariableTypeTableEntry::read(rdr));
            local_variable_type_table.push(entry);
        }
        Ok(LocalVariableTypeTableAttribute { local_variable_type_table: local_variable_type_table })
    }
}

#[derive(Debug)]
pub struct LocalVariableTypeTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

impl LocalVariableTypeTableEntry {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<LocalVariableTypeTableEntry> {
        let start_pc = try!(read_u16(rdr));
        let length = try!(read_u16(rdr));
        let name_index = try!(read_u16(rdr));
        let signature_index = try!(read_u16(rdr));
        let index = try!(read_u16(rdr));
        Ok(LocalVariableTypeTableEntry {
            start_pc: start_pc,
            length: length,
            name_index: name_index,
            signature_index: signature_index,
            index: index,
        })
    }
}
