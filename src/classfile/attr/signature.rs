use std::io;

use util::*;
use classfile::error::Result;

#[derive(Debug)]
pub struct SignatureAttribute {
    signature_index: u16,
}

impl SignatureAttribute {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<SignatureAttribute> {
        let signature_index = try!(read_u16(rdr));
        Ok(SignatureAttribute { signature_index: signature_index })
    }
}
