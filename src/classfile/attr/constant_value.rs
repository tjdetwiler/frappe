use std::io;

use util::*;
use classfile::error::Result;

#[derive(Debug)]
pub struct ConstantValueAttribute {
    constantvalue_index: u16,
}

impl ConstantValueAttribute {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<ConstantValueAttribute> {
        let constantvalue_index = try!(read_u16(rdr));
        Ok(ConstantValueAttribute { constantvalue_index: constantvalue_index })
    }
}
