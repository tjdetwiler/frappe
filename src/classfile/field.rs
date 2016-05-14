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

/// Holds the [`access_flags`](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.5-200-A.1) value from a `FieldInfo` structure.
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
        self.read_flag(ACC_PUBLIC)
    }

    pub fn is_private(&self) -> bool {
        self.read_flag(ACC_PRIVATE)
    }

    pub fn is_protected(&self) -> bool {
        self.read_flag(ACC_PROTECTED)
    }

    pub fn is_static(&self) -> bool {
        self.read_flag(ACC_STATIC)
    }

    pub fn is_final(&self) -> bool {
        self.read_flag(ACC_FINAL)
    }

    pub fn is_volatile(&self) -> bool {
        self.read_flag(ACC_VOLATILE)
    }

    pub fn is_transient(&self) -> bool {
        self.read_flag(ACC_TRANSIENT)
    }

    pub fn is_synthetic(&self) -> bool {
        self.read_flag(ACC_SYNTHETIC)
    }

    pub fn is_enum(&self) -> bool {
        self.read_flag(ACC_ENUM)
    }

    fn read_flag(&self, mask: u16) -> bool {
        (self.access_flags & mask) != 0
    }
}

/// Wrapper around a `Vec<FieldInfo>`.
#[derive(Debug)]
pub struct Fields {
    fields: Vec<FieldInfo>
}

impl Fields {
    /// Reads in a list of `FieldInfo` structures. The reader should be positioned such that
    /// the next 2 byte define the number of entries, followed immediately by the
    /// `FieldInfo` structures.
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

/// Metadata about a field in a class file.
#[derive(Debug)]
pub struct FieldInfo {
    /// Metadata about this field.
    pub access_flags: FieldAccessFlags,
    /// The name of this field.
    pub name_index: u16,
    /// A [field descriptor string](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.3.2).
    pub descriptor_index: u16,
    /// Collection of attributes that are associated with this field.
    pub attributes: Attributes
}

impl FieldInfo {
    /// Constructs a [`FieldInfo`](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.5)
    /// structure from a byte stream containing classfile data.
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
