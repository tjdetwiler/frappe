pub mod method;
pub mod field;
pub mod constant_pool;
pub mod attr;
pub mod error;

use std::fmt;
use std::vec::Vec;
use std::io;

use util::*;
use classfile::error::Result;
use classfile::attr::Attributes;
use classfile::constant_pool::{ConstantPool, ClassTag, Tag};
use classfile::field::Fields;
use classfile::method::Methods;

bitflags! {
    pub flags ClassAccessFlags: u16 {
        const ACC_PUBLIC        = 0x0001,
        const ACC_FINAL         = 0x0010,
        const ACC_SUPER         = 0x0020,
        const ACC_INTERFACE     = 0x0200,
        const ACC_ABSTRACT      = 0x0400,
        const ACC_SYNTHETIC     = 0x1000,
        const ACC_ANNOTATION    = 0x2000,
        const ACC_ENUM          = 0x4000

    }
}

impl ClassAccessFlags {
    fn new(access_flags: u16) -> ClassAccessFlags {
        ClassAccessFlags::from_bits_truncate(access_flags)
    }

    pub fn is_public(&self) -> bool {
        self.contains(ACC_PUBLIC)
    }

    pub fn is_final(&self) -> bool {
        self.contains(ACC_FINAL)
    }

    pub fn is_super(&self) -> bool {
        self.contains(ACC_SUPER)
    }

    pub fn is_interface(&self) -> bool {
        self.contains(ACC_INTERFACE)
    }

    pub fn is_abstract(&self) -> bool {
        self.contains(ACC_ABSTRACT)
    }

    pub fn is_synthetic(&self) -> bool {
        self.contains(ACC_SYNTHETIC)
    }

    pub fn is_annotation(&self) -> bool {
        self.contains(ACC_ANNOTATION)
    }

    pub fn is_enum(&self) -> bool {
        self.contains(ACC_ENUM)
    }
}

impl fmt::Display for ClassAccessFlags {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::new();
        if self.is_public() { v.push("ACC_PUBLIC"); }
        if self.is_final() { v.push("ACC_FINAL"); }
        if self.is_super() { v.push("ACC_SUPER"); }
        if self.is_interface() { v.push("ACC_INTERFACE"); }
        if self.is_abstract() { v.push("ACC_ABSTRACT"); }
        if self.is_synthetic() { v.push("ACC_SYNTHETIC"); }
        if self.is_annotation() { v.push("ACC_ANNOTATION"); }
        if self.is_enum() { v.push("ACC_ENUM"); }

        write!(f, "{}", v.join(", "))
    }
}

#[derive(Debug)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: ClassAccessFlags,
    this_class_index: u16,
    super_class_index: u16,
    pub interfaces: Vec<u16>,
    pub fields: Fields,
    pub methods: Methods,
    pub attributes: Attributes
}

impl ClassFile {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<ClassFile> {
        let magic = try!(read_u32(rdr));
        let minor_version = try!(read_u16(rdr));
        let major_version = try!(read_u16(rdr));
        let constant_pool = try!(ConstantPool::read(rdr));
        let access_flags = try!(read_u16(rdr));
        let this_class_index = try!(read_u16(rdr));
        let super_class_index = try!(read_u16(rdr));
        let interfaces_count = try!(read_u16(rdr));
        let mut interfaces: Vec<u16> = vec![];
        for _ in 0..interfaces_count {
            let entry = try!(read_u16(rdr));
            interfaces.push(entry);
        }
        let fields = try!(Fields::read(rdr, &constant_pool));
        let methods = try!(Methods::read(rdr, &constant_pool));
        let attributes = try!(Attributes::read(rdr, &constant_pool));

        Ok(ClassFile {
            magic: magic,
            minor_version: minor_version,
            major_version: major_version,
            constant_pool: constant_pool,
            access_flags: ClassAccessFlags::new(access_flags),
            this_class_index: this_class_index,
            super_class_index: super_class_index,
            interfaces: interfaces,
            fields: fields,
            methods: methods,
            attributes: attributes
        })
    }

    pub fn this_class(&self) -> &ClassTag {
        let tag = &self.constant_pool[self.this_class_index];
        if let Tag::Class(ref class_tag) = *tag {
            class_tag
        } else {
            panic!(format!("ConstantPoolTag entry found is not of type Class: {:?}", tag));
        }
    }

    pub fn super_class(&self) -> Option<&ClassTag> {
        if self.super_class_index == 0 {
            return None;
        }
        let tag = &self.constant_pool[self.super_class_index];
        if let Tag::Class(ref class_tag) = *tag {
            Some(class_tag)
        } else {
            panic!(format!("ConstantPoolTag entry found is not of type Class: {:?}", tag));
        }
    }
}


