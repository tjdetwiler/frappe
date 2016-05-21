use std::io;

use util::*;
use classfile::error::Result;

bitflags! {
    pub flags InnerClassAccessFlags: u16 {
        const IC_ACC_PUBLIC        = 0x0001,
        const IC_ACC_PRIVATE       = 0x0002,
        const IC_ACC_PROTECTED     = 0x0004,
        const IC_ACC_STATIC        = 0x0008,
        const IC_ACC_FINAL         = 0x0010,
        const IC_ACC_INTERFACE     = 0x0200,
        const IC_ACC_ABSTRACT      = 0x0400,
        const IC_ACC_SYNTHETIC     = 0x1000,
        const IC_ACC_ANNOTATION    = 0x2000,
        const IC_ACC_ENUM          = 0x4000
    }
}

#[derive(Debug)]
pub struct InnerClassInfo {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: InnerClassAccessFlags,
}

impl InnerClassInfo {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<InnerClassInfo> {
        let inner_class_info_index = try!(read_u16(rdr));
        let outer_class_info_index = try!(read_u16(rdr));
        let inner_name_index = try!(read_u16(rdr));
        let inner_class_access_flags = try!(read_u16(rdr));
        Ok(InnerClassInfo {
            inner_class_info_index: inner_class_info_index,
            outer_class_info_index: outer_class_info_index,
            inner_name_index: inner_name_index,
            inner_class_access_flags:
                InnerClassAccessFlags::from_bits_truncate(inner_class_access_flags),
        })
    }
}

#[derive(Debug)]
pub struct InnerClassesAttribute {
    pub classes: Vec<InnerClassInfo>,
}

impl InnerClassesAttribute {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<InnerClassesAttribute> {
        let number_of_classes = try!(read_u16(rdr));
        let mut inner_classes: Vec<InnerClassInfo> = vec![];
        for _ in 0..number_of_classes {
            let inner_class_info = try!(InnerClassInfo::read(rdr));
            inner_classes.push(inner_class_info);
        }
        Ok(InnerClassesAttribute { classes: inner_classes })
    }
}
