use std::io;

use util::*;
use classfile::error::Result;

#[derive(Debug)]
pub struct EnclosingMethodAttribute {
    pub class_index: u16,
    pub method_index: u16,
}

impl EnclosingMethodAttribute {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<EnclosingMethodAttribute> {
        let class_index = try!(read_u16(rdr));
        let method_index = try!(read_u16(rdr));
        Ok(EnclosingMethodAttribute {
            class_index: class_index,
            method_index: method_index,
        })
    }
}
