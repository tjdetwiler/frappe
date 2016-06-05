use super::Attributes;

bitflags! {
/// Holds the
/// [`access_flags`]
/// (https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.5-200-A.1)
/// value from a `FieldInfo` structure.
    pub flags FieldAccessFlags: u16 {
        const FIELD_ACC_PUBLIC        = 0x0001,
        const FIELD_ACC_PRIVATE       = 0x0002,
        const FIELD_ACC_PROTECTED     = 0x0004,
        const FIELD_ACC_STATIC        = 0x0008,
        const FIELD_ACC_FINAL         = 0x0010,
        const FIELD_ACC_VOLATILE      = 0x0040,
        const FIELD_ACC_TRANSIENT     = 0x0080,
        const FIELD_ACC_SYNTHETIC     = 0x1000,
        const FIELD_ACC_ENUM          = 0x4000
    }
}

impl FieldAccessFlags {
    pub fn is_public(&self) -> bool {
        self.contains(FIELD_ACC_PUBLIC)
    }

    pub fn is_private(&self) -> bool {
        self.contains(FIELD_ACC_PRIVATE)
    }

    pub fn is_protected(&self) -> bool {
        self.contains(FIELD_ACC_PROTECTED)
    }

    pub fn is_static(&self) -> bool {
        self.contains(FIELD_ACC_STATIC)
    }

    pub fn is_final(&self) -> bool {
        self.contains(FIELD_ACC_FINAL)
    }

    pub fn is_volatile(&self) -> bool {
        self.contains(FIELD_ACC_VOLATILE)
    }

    pub fn is_transient(&self) -> bool {
        self.contains(FIELD_ACC_TRANSIENT)
    }

    pub fn is_synthetic(&self) -> bool {
        self.contains(FIELD_ACC_SYNTHETIC)
    }

    pub fn is_enum(&self) -> bool {
        self.contains(FIELD_ACC_ENUM)
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
    pub attrs: Attributes,
}
