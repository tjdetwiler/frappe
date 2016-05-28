use classfile::attr::Attributes;

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
