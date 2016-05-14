use std::io;
use std::vec::Vec;
use std::ops::Deref;

use util::*;
use classfile::attr::AttributeInfo;
use classfile::constant_pool as cp;

#[derive(Debug)]
pub struct Methods {
    methods: Vec<MethodInfo>
}

impl Methods {
    pub fn read<T: io::Read>(rdr: &mut T, constant_pool: &cp::ConstantPool) -> io::Result<Methods> {
        let methods_count = try!(read_u16(rdr));
        let mut methods: Vec<MethodInfo> = vec![];
        for _ in 0..methods_count {
            let entry = try!(MethodInfo::read(rdr, constant_pool));
            methods.push(entry);
        }
        Ok(Methods {
            methods: methods
        })
    }
}

impl Deref for Methods {
    type Target = Vec<MethodInfo>;

    fn deref(&self) -> &Vec<MethodInfo> {
        &self.methods
    }
}

const ACC_PUBLIC: u16 = 0x0001;
const ACC_PRIVATE: u16 = 0x0002;
const ACC_PROTECTED: u16 = 0x0004;
const ACC_STATIC: u16 = 0x0008;
const ACC_FINAL: u16 = 0x0010;
const ACC_SYNCHRONIZED: u16 = 0x0020;
const ACC_BRIDGE: u16 = 0x0040;
const ACC_VARARGS: u16 = 0x0080;
const ACC_NATIVE: u16 = 0x0100;
const ACC_ABSTRACT: u16 = 0x0400;
const ACC_STRICT: u16 = 0x0800;
const ACC_SYNTHETIC: u16 = 0x1000;

#[derive(Debug)]
pub struct MethodAccessFlags {
    access_flags: u16
}

impl MethodAccessFlags {
    fn new(access_flags: u16) -> MethodAccessFlags {
        MethodAccessFlags {
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

    pub fn is_syncronized(&self) -> bool {
        (self.access_flags & ACC_SYNCHRONIZED) != 0
    }

    pub fn is_bridge(&self) -> bool {
        (self.access_flags & ACC_BRIDGE) != 0
    }

    pub fn is_varargs(&self) -> bool {
        (self.access_flags & ACC_VARARGS) != 0
    }

    pub fn is_native(&self) -> bool {
        (self.access_flags & ACC_NATIVE) != 0
    }

    pub fn is_abstract(&self) -> bool {
        (self.access_flags & ACC_ABSTRACT) != 0
    }

    pub fn is_strict(&self) -> bool {
        (self.access_flags & ACC_STRICT) != 0
    }

    pub fn is_synthetic(&self) -> bool {
        (self.access_flags & ACC_SYNTHETIC) != 0
    }
}

#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: MethodAccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attribute_info: Vec<AttributeInfo>
}

impl MethodInfo {
    pub fn read<T: io::Read>(rdr: &mut T, constant_pool: &cp::ConstantPool) -> io::Result<MethodInfo> {
        let access_flags = try!(read_u16(rdr));
        let name_index = try!(read_u16(rdr));
        let descriptor_index = try!(read_u16(rdr));
        let attributes_count = try!(read_u16(rdr));
        let mut attribute_info: Vec<AttributeInfo> = vec![];
        for _ in 0..attributes_count {
            let attribute = try!(AttributeInfo::read(rdr, constant_pool));
            attribute_info.push(attribute);
        }
        Ok(MethodInfo {
            access_flags: MethodAccessFlags::new(access_flags),
            name_index: name_index,
            descriptor_index: descriptor_index,
            attributes_count: attributes_count,
            attribute_info: attribute_info
        })
    }
}

