use std::io;

use util::*;
use classfile::error::Result;

#[derive(Debug)]
pub struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

impl LineNumberTableEntry {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<LineNumberTableEntry> {
        let start_pc = try!(read_u16(rdr));
        let line_number = try!(read_u16(rdr));
        Ok(LineNumberTableEntry {
            start_pc: start_pc,
            line_number: line_number,
        })
    }
}

#[derive(Debug)]
pub struct LineNumberTableAttribute {
    line_number_table: Vec<LineNumberTableEntry>,
}

impl LineNumberTableAttribute {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<LineNumberTableAttribute> {
        let line_number_table_length = try!(read_u16(rdr));
        let mut line_number_table: Vec<LineNumberTableEntry> = vec![];
        for _ in 0..line_number_table_length {
            let line_number_table_entry = try!(LineNumberTableEntry::read(rdr));
            line_number_table.push(line_number_table_entry);
        }
        Ok(LineNumberTableAttribute { line_number_table: line_number_table })
    }
}
