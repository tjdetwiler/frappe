use std::io;
use std::vec::Vec;
use std::ops::Deref;

use util::*;
use classfile::error::*;
use classfile::attr::Attributes;
use classfile::constant_pool as cp;

bitflags! {
/// Holds the
/// [`access_flags`]
/// (https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.5-200-A.1)
/// value from a `FieldInfo` structure.
    pub flags FieldAccessFlags: u16 {
        const ACC_PUBLIC        = 0x0001,
        const ACC_PRIVATE       = 0x0002,
        const ACC_PROTECTED     = 0x0004,
        const ACC_STATIC        = 0x0008,
        const ACC_FINAL         = 0x0010,
        const ACC_VOLATILE      = 0x0040,
        const ACC_TRANSIENT     = 0x0080,
        const ACC_SYNTHETIC     = 0x1000,
        const ACC_ENUM          = 0x4000
    }
}

impl FieldAccessFlags {
    fn new(access_flags: u16) -> FieldAccessFlags {
        FieldAccessFlags::from_bits_truncate(access_flags)
    }

    /// Returns `true` if the `ACC_PUBLIC` flag is set.
    pub fn is_public(&self) -> bool {
        self.contains(ACC_PUBLIC)
    }

    /// Returns `true` if the `ACC_PRIVATE` flag is set.
    pub fn is_private(&self) -> bool {
        self.contains(ACC_PRIVATE)
    }

    /// Returns `true` if the `ACC_PROTECTED` flag is set.
    pub fn is_protected(&self) -> bool {
        self.contains(ACC_PROTECTED)
    }

    /// Returns `true` if the `ACC_STATIC` flag is set.
    pub fn is_static(&self) -> bool {
        self.contains(ACC_STATIC)
    }

    /// Returns `true` if the `ACC_FINAL` flag is set.
    pub fn is_final(&self) -> bool {
        self.contains(ACC_FINAL)
    }

    /// Returns `true` if the `ACC_VOLATILE` flag is set.
    pub fn is_volatile(&self) -> bool {
        self.contains(ACC_VOLATILE)
    }

    /// Returns `true` if the `ACC_TRANSIENT` flag is set.
    pub fn is_transient(&self) -> bool {
        self.contains(ACC_TRANSIENT)
    }

    /// Returns `true` if the `ACC_SYNTHETIC` flag is set.
    pub fn is_synthetic(&self) -> bool {
        self.contains(ACC_SYNTHETIC)
    }

    /// Returns `true` if the `ACC_ENUM` flag is set.
    pub fn is_enum(&self) -> bool {
        self.contains(ACC_ENUM)
    }
}

/// Wrapper around a `Vec<FieldInfo>`.
#[derive(Debug)]
pub struct Fields {
    fields: Vec<FieldInfo>,
}

impl Fields {
    /// Reads in a list of `FieldInfo` structures. The reader should be positioned such that
    /// the next 2 byte define the number of entries, followed immediately by the
    /// `FieldInfo` structures.
    pub fn read<T: io::Read>(rdr: &mut T, constant_pool: &cp::ConstantPool) -> Result<Fields> {
        let fields_count = try!(read_u16(rdr));
        let mut fields: Vec<FieldInfo> = vec![];
        for _ in 0..fields_count {
            let entry = try!(FieldInfo::read(rdr, constant_pool));
            fields.push(entry);
        }
        Ok(Fields { fields: fields })
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
    /// A [field descriptor string]
    /// (https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.3.2).
    pub descriptor_index: u16,
    /// Collection of attributes that are associated with this field.
    pub attributes: Attributes,
}

impl FieldInfo {
    /// Constructs a [`FieldInfo`]
    /// (https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.5)
    /// structure from a byte stream containing classfile data.
    pub fn read<T: io::Read>(rdr: &mut T,
                             constant_pool: &cp::ConstantPool)
                             -> Result<FieldInfo> {
        let access_flags = try!(read_u16(rdr));
        let name_index = try!(read_u16(rdr));
        let descriptor_index = try!(read_u16(rdr));
        let attributes: Attributes = try!(Attributes::read(rdr, constant_pool));
        Ok(FieldInfo {
            access_flags: FieldAccessFlags::new(access_flags),
            name_index: name_index,
            descriptor_index: descriptor_index,
            attributes: attributes,
        })
    }
}
