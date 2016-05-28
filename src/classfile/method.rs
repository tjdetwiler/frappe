use classfile::attr::Attributes;

bitflags! {
    pub flags MethodAccessFlags: u16 {
        const ACC_PUBLIC        = 0x0001,
        const ACC_PRIVATE       = 0x0002,
        const ACC_PROTECTED     = 0x0004,
        const ACC_STATIC        = 0x0008,
        const ACC_FINAL         = 0x0010,
        const ACC_SYNCHRONIZED  = 0x0020,
        const ACC_BRIDGE        = 0x0040,
        const ACC_VARARGS       = 0x0080,
        const ACC_NATIVE        = 0x0100,
        const ACC_ABSTRACT      = 0x0400,
        const ACC_STRICT        = 0x0800,
        const ACC_SYNTHETIC     = 0x1000
    }
}

impl MethodAccessFlags {
    pub fn is_public(&self) -> bool {
        self.contains(ACC_PUBLIC)
    }

    pub fn is_private(&self) -> bool {
        self.contains(ACC_PRIVATE)
    }

    pub fn is_protected(&self) -> bool {
        self.contains(ACC_PROTECTED)
    }

    pub fn is_static(&self) -> bool {
        self.contains(ACC_STATIC)
    }

    pub fn is_final(&self) -> bool {
        self.contains(ACC_FINAL)
    }

    pub fn is_synchronized(&self) -> bool {
        self.contains(ACC_SYNCHRONIZED)
    }

    pub fn is_bridge(&self) -> bool {
        self.contains(ACC_BRIDGE)
    }

    pub fn is_varargs(&self) -> bool {
        self.contains(ACC_VARARGS)
    }

    pub fn is_native(&self) -> bool {
        self.contains(ACC_NATIVE)
    }

    pub fn is_abstract(&self) -> bool {
        self.contains(ACC_ABSTRACT)
    }

    pub fn is_strict(&self) -> bool {
        self.contains(ACC_STRICT)
    }

    pub fn is_synthetic(&self) -> bool {
        self.contains(ACC_SYNTHETIC)
    }
}

#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: MethodAccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Attributes,
}
