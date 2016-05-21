use std::io;

use util::*;

#[derive(Debug)]
pub struct SourceFileAttribute {
    pub sourcefile_index: u16,
}

impl SourceFileAttribute {
    pub fn read<T: io::Read>(rdr: &mut T) -> io::Result<SourceFileAttribute> {
        let sourcefile_index = try!(read_u16(rdr));
        Ok(SourceFileAttribute { sourcefile_index: sourcefile_index })
    }
}

