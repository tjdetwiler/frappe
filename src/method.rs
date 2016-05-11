use std::io;
use std::vec::Vec;

use util::*;
use attr::AttributeInfo;

#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attribute_info: Vec<AttributeInfo>
}

impl MethodInfo {
    pub fn read<T: io::Read>(rdr: &mut T) -> io::Result<MethodInfo> {
        let access_flags = try!(read_u16(rdr));
        let name_index = try!(read_u16(rdr));
        let descriptor_index = try!(read_u16(rdr));
        let attributes_count = try!(read_u16(rdr));
        let mut attribute_info: Vec<AttributeInfo> = vec![];
        for _ in 0..attributes_count {
            let attribute = try!(AttributeInfo::read(rdr));
            attribute_info.push(attribute);
        }
        Ok(MethodInfo {
            access_flags: access_flags,
            name_index: name_index,
            descriptor_index: descriptor_index,
            attributes_count: attributes_count,
            attribute_info: attribute_info
        })
    }
}

