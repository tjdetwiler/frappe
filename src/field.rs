use std::io;
use std::vec::Vec;

use util::*;
use attr::Attributes;

#[derive(Debug)]
pub struct Fields {
    fields: Vec<FieldInfo>
}

impl Fields {
    pub fn read<T: io::Read>(rdr: &mut T) -> io::Result<Fields> {
        let fields_count = try!(read_u16(rdr));
        let mut fields: Vec<FieldInfo> = vec![];
        for _ in 0..fields_count {
            let entry = try!(FieldInfo::read(rdr));
            fields.push(entry);
        }
        Ok(Fields {
            fields: fields
        })
    }
}

#[derive(Debug)]
pub struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Attributes
}

impl FieldInfo {
    pub fn read<T: io::Read>(rdr: &mut T) -> io::Result<FieldInfo> {
        let access_flags = try!(read_u16(rdr));
        let name_index = try!(read_u16(rdr));
        let descriptor_index = try!(read_u16(rdr));
        let mut attributes: Attributes = try!(Attributes::read(rdr));
        Ok(FieldInfo {
            access_flags: access_flags,
            name_index: name_index,
            descriptor_index: descriptor_index,
            attributes: attributes
        })
    }
}

