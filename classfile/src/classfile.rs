use std::fmt;
use std::vec::Vec;

use super::{Attributes, ConstantPool, FieldInfo, MethodInfo};

#[derive(Debug)]
pub struct ClassFile {
    /// Java classfile magic number. `0xcafebabe` is the only valid value for
    /// this field.
    pub magic: u32,
    /// The minor version of this classfile.
    pub minor_version: u16,
    /// The major version of this classfile.
    pub major_version: u16,
    /// Classfile constant pool. Contains constant values (integer, long, string, etc)
    /// as well as metadata about classes and types.
    pub constants: ConstantPool,
    /// Access flags for this class.
    pub access_flags: ClassAccessFlags,
    /// Index into the constant pool that resolves to a `Constant::Class` value.
    pub this_class: u16,
    /// Optional index into the constant pool that resolves to a `Constant::Class`
    /// value. If `super_class` is zero then there is no super class for this type.
    pub super_class: u16,
    /// A list of indicies into the constant pool that identify any interfaces
    /// directly applied to this class. These indicies must resolve to
    /// `Constant::Class` entries.
    pub interfaces: Vec<u16>,
    /// A list of field descriptors that identify the fields of this class.
    pub fields: Vec<FieldInfo>,
    /// A list of field descriptors that identify the methods of this class.
    pub methods: Vec<MethodInfo>,
    /// A list of attributes applied to this class.
    pub attrs: Attributes,
}

impl ClassFile {
    /// Resolves the `this_class` member to the UTF8 string in the constant pool
    /// that holds the class name.
    pub fn this_class_name(&self) -> &String {
        let name_index = self.constants[self.this_class].as_class();
        self.constants[name_index].as_utf8()
    }

    /// Resolves the `super_class` member to the UTF8 string in the constant pool
    /// that holds the super class name. If `super_class == 0` then `None` is
    /// returned.
    pub fn super_class_name(&self) -> Option<&String> {
        if self.super_class == 0 {
            return None;
        }
        let name_index = self.constants[self.super_class].as_class();
        Some(self.constants[name_index].as_utf8())
    }

    pub fn find_method(&self, method_name: &str) -> Option<&MethodInfo> {
        for method in self.methods.iter() {
            let name = self.constants[method.name_index].as_utf8();
            if name == method_name {
                return Some(method);
            }
        }
        None
    }

    pub fn find_field(&self, field_name: &str) -> Option<&FieldInfo> {
        for field in self.fields.iter() {
            let name = self.constants[field.name_index].as_utf8();
            if name == field_name {
                return Some(field);
            }
        }
        None
    }
}

bitflags! {
    pub flags ClassAccessFlags: u16 {
        const CLASS_ACC_PUBLIC        = 0x0001,
        const CLASS_ACC_FINAL         = 0x0010,
        const CLASS_ACC_SUPER         = 0x0020,
        const CLASS_ACC_INTERFACE     = 0x0200,
        const CLASS_ACC_ABSTRACT      = 0x0400,
        const CLASS_ACC_SYNTHETIC     = 0x1000,
        const CLASS_ACC_ANNOTATION    = 0x2000,
        const CLASS_ACC_ENUM          = 0x4000
    }
}

impl ClassAccessFlags {
    pub fn is_public(&self) -> bool {
        self.contains(CLASS_ACC_PUBLIC)
    }

    pub fn is_final(&self) -> bool {
        self.contains(CLASS_ACC_FINAL)
    }

    pub fn is_super(&self) -> bool {
        self.contains(CLASS_ACC_SUPER)
    }

    pub fn is_interface(&self) -> bool {
        self.contains(CLASS_ACC_INTERFACE)
    }

    pub fn is_abstract(&self) -> bool {
        self.contains(CLASS_ACC_ABSTRACT)
    }

    pub fn is_synthetic(&self) -> bool {
        self.contains(CLASS_ACC_SYNTHETIC)
    }

    pub fn is_annotation(&self) -> bool {
        self.contains(CLASS_ACC_ANNOTATION)
    }

    pub fn is_enum(&self) -> bool {
        self.contains(CLASS_ACC_ENUM)
    }
}

impl fmt::Display for ClassAccessFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::new();
        if self.is_public() {
            v.push("ACC_PUBLIC");
        }
        if self.is_final() {
            v.push("ACC_FINAL");
        }
        if self.is_super() {
            v.push("ACC_SUPER");
        }
        if self.is_interface() {
            v.push("ACC_INTERFACE");
        }
        if self.is_abstract() {
            v.push("ACC_ABSTRACT");
        }
        if self.is_synthetic() {
            v.push("ACC_SYNTHETIC");
        }
        if self.is_annotation() {
            v.push("ACC_ANNOTATION");
        }
        if self.is_enum() {
            v.push("ACC_ENUM");
        }

        write!(f, "{}", v.join(", "))
    }
}
