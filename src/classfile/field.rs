use std::io;
use std::vec::Vec;
use std::ops::Deref;

use util::*;
use classfile::attr::Attributes;
use classfile::constant_pool as cp;

const ACC_PUBLIC: u16 = 0x0001;
const ACC_PRIVATE: u16 = 0x0002;
const ACC_PROTECTED: u16 = 0x0004;
const ACC_STATIC: u16 = 0x0008;
const ACC_FINAL: u16 = 0x0010;
const ACC_VOLATILE: u16 = 0x0040;
const ACC_TRANSIENT: u16 = 0x0080;
const ACC_SYNTHETIC: u16 = 0x1000;
const ACC_ENUM: u16 = 0x4000;

#[derive(Debug)]
pub struct FieldAccessFlags {
    access_flags: u16
}

impl FieldAccessFlags {
    fn new(access_flags: u16) -> FieldAccessFlags {
        FieldAccessFlags {
            access_flags: access_flags
        }
    }

    pub fn is_public(&self) -> bool {
        (self.access_flags & ACC_PUBLIC) != 0
    }

    pub fn is_private(&self) -> bool {
        (self.access_flags & ACC_PRIVATE) != 0
    }

    pub fn is_protected(&self) -> bool {
        (self.access_flags & ACC_PROTECTED) != 0
    }

    pub fn is_static(&self) -> bool {
        (self.access_flags & ACC_STATIC) != 0
    }

    pub fn is_final(&self) -> bool {
        (self.access_flags & ACC_FINAL) != 0
    }

    pub fn is_volatile(&self) -> bool {
        (self.access_flags & ACC_VOLATILE) != 0
    }

    pub fn is_transient(&self) -> bool {
        (self.access_flags & ACC_TRANSIENT) != 0
    }

    pub fn is_synthetic(&self) -> bool {
        (self.access_flags & ACC_SYNTHETIC) != 0
    }

    pub fn is_enum(&self) -> bool {
        (self.access_flags & ACC_ENUM) != 0
    }
}

#[derive(Debug)]
pub struct Fields {
    fields: Vec<FieldInfo>
}

impl Fields {
    pub fn read<T: io::Read>(rdr: &mut T, constant_pool: &cp::ConstantPool) -> io::Result<Fields> {
        let fields_count = try!(read_u16(rdr));
        let mut fields: Vec<FieldInfo> = vec![];
        for _ in 0..fields_count {
            let entry = try!(FieldInfo::read(rdr, constant_pool));
            fields.push(entry);
        }
        Ok(Fields {
            fields: fields
        })
    }
}

impl Deref for Fields {
    type Target = Vec<FieldInfo>;

    fn deref(&self) -> &Vec<FieldInfo> {
        &self.fields
    }
}

#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: FieldAccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Attributes
}

impl FieldInfo {
    pub fn read<T: io::Read>(rdr: &mut T, constant_pool: &cp::ConstantPool) -> io::Result<FieldInfo> {
        let access_flags = try!(read_u16(rdr));
        let name_index = try!(read_u16(rdr));
        let descriptor_index = try!(read_u16(rdr));
        let attributes: Attributes = try!(Attributes::read(rdr, constant_pool));
        Ok(FieldInfo {
            access_flags: FieldAccessFlags::new(access_flags),
            name_index: name_index,
            descriptor_index: descriptor_index,
            attributes: attributes
        })
    }
}
