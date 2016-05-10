use std::io;
use std::vec::Vec;

use util::*;

#[derive(Debug)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_name_length: u32,
    pub info: Vec<u8>
}

impl AttributeInfo {
    pub fn read<T: io::Read>(rdr: &mut T) -> io::Result<AttributeInfo> {
        let attribute_name_index = try!(read_u16(rdr));
        let attribute_name_length = try!(read_u32(rdr));
        let mut info: Vec<u8> = vec![];
        for _ in 0..attribute_name_length {
            let byte = try!(read_u8(rdr));
            info.push(byte);
        }
        Ok(AttributeInfo {
            attribute_name_index: attribute_name_index,
            attribute_name_length: attribute_name_length,
            info: info
        })
    }
}
