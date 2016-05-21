use std::io;

use util::*;
use classfile::error::Result;

#[derive(Debug)]
pub struct ExceptionsAttribute {
    exception_index_table: Vec<u16>,
}

impl ExceptionsAttribute {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<ExceptionsAttribute> {
        let number_of_exceptions = try!(read_u16(rdr));
        let mut exception_index_table: Vec<u16> = vec![];
        for _ in 0..number_of_exceptions {
            let exception_index = try!(read_u16(rdr));
            exception_index_table.push(exception_index);
        }
        Ok(ExceptionsAttribute { exception_index_table: exception_index_table })
    }
}
