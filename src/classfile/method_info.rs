use classfile::Attributes;

#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: MethodAccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attrs: Attributes,
}

bitflags! {
    pub flags MethodAccessFlags: u16 {
        const METHOD_ACC_PUBLIC        = 0x0001,
        const METHOD_ACC_PRIVATE       = 0x0002,
        const METHOD_ACC_PROTECTED     = 0x0004,
        const METHOD_ACC_STATIC        = 0x0008,
        const METHOD_ACC_FINAL         = 0x0010,
        const METHOD_ACC_SYNCHRONIZED  = 0x0020,
        const METHOD_ACC_BRIDGE        = 0x0040,
        const METHOD_ACC_VARARGS       = 0x0080,
        const METHOD_ACC_NATIVE        = 0x0100,
        const METHOD_ACC_ABSTRACT      = 0x0400,
        const METHOD_ACC_STRICT        = 0x0800,
        const METHOD_ACC_SYNTHETIC     = 0x1000
    }
}

impl MethodAccessFlags {
    pub fn is_public(&self) -> bool {
        self.contains(METHOD_ACC_PUBLIC)
    }

    pub fn is_private(&self) -> bool {
        self.contains(METHOD_ACC_PRIVATE)
    }

    pub fn is_protected(&self) -> bool {
        self.contains(METHOD_ACC_PROTECTED)
    }

    pub fn is_static(&self) -> bool {
        self.contains(METHOD_ACC_STATIC)
    }

    pub fn is_final(&self) -> bool {
        self.contains(METHOD_ACC_FINAL)
    }

    pub fn is_synchronized(&self) -> bool {
        self.contains(METHOD_ACC_SYNCHRONIZED)
    }

    pub fn is_bridge(&self) -> bool {
        self.contains(METHOD_ACC_BRIDGE)
    }

    pub fn is_varargs(&self) -> bool {
        self.contains(METHOD_ACC_VARARGS)
    }

    pub fn is_native(&self) -> bool {
        self.contains(METHOD_ACC_NATIVE)
    }

    pub fn is_abstract(&self) -> bool {
        self.contains(METHOD_ACC_ABSTRACT)
    }

    pub fn is_strict(&self) -> bool {
        self.contains(METHOD_ACC_STRICT)
    }

    pub fn is_synthetic(&self) -> bool {
        self.contains(METHOD_ACC_SYNTHETIC)
    }
}
