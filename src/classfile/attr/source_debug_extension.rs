use std::io;

use util::*;
use classfile::error::Result;

#[derive(Debug)]
pub struct SourceDebugExtensionAttribute {
    pub debug_extension: Vec<u8>,
}

impl SourceDebugExtensionAttribute {
    pub fn read<T: io::Read>(rdr: &mut T,
                             attr_length: u32)
                             -> Result<SourceDebugExtensionAttribute> {
        let mut debug_extension: Vec<u8> = vec![];
        for _ in 0..attr_length {
            let byte = try!(read_u8(rdr));
            debug_extension.push(byte);
        }
        Ok(SourceDebugExtensionAttribute { debug_extension: debug_extension })
    }
}
