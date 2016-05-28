use std::ops::{Deref, Index};

#[derive(Debug)]
pub struct ConstantPool {
    pub pool: Vec<Constant>,
}

impl ConstantPool {
    pub fn len(&self) -> u16 {
        self.pool.len() as u16 + 1
    }
}

impl Index<u16> for ConstantPool {
    type Output = Constant;

    fn index<'a>(&'a self, index: u16) -> &'a Constant {
        &self.pool[index as usize - 1]
    }
}

impl Deref for ConstantPool {
    type Target = Vec<Constant>;

    fn deref(&self) -> &Vec<Constant> {
        &self.pool
    }
}

/// Represents an entity in the constant pool has an associated class, as well
/// as a name and type.
///
/// This applies to `Fieldref`, `Methodref`, and `InterfaceMethodref` constants.
#[derive(Debug, Eq, PartialEq)]
pub struct TypedEntityConstant {
    /// An index into the constant pool that is of type `Constant::Class`.
    pub class_index: u16,
    /// An index into the constant pool that is of type `Constant::NameAndType`.
    pub name_and_type_index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct NameAndTypeConstant {
    /// An index into the constant pool that is of type `Constant::Utf8`.
    pub name_index: u16,
    /// An index into the constant pool that is of type `Constant::Utf8`.
    ///
    /// This string is either a method or field descriptor (depending on context)
    /// that identifies the specific type of the entity.
    pub descriptor_index: u16,
}

/// Represents a single entry in the constant pool.
#[derive(Debug, Eq, PartialEq)]
pub enum Constant {
    Class(u16),
    Fieldref(TypedEntityConstant),
    Methodref(TypedEntityConstant),
    InterfaceMethodref(TypedEntityConstant),
    String(u16),
    Integer(i32),
    Float {
        bytes: u32,
    },
    Long {
        high_bytes: u32,
        low_bytes: u32,
    },
    Double {
        high_bytes: u32,
        low_bytes: u32,
    },
    NameAndType(NameAndTypeConstant),
    Utf8(String),
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    MethodType {
        descriptor_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
}

impl Constant {
    fn name(&self) -> &str {
        match *self {
            Constant::Class(_) => "Class",
            Constant::Fieldref(_) => "Fieldref",
            Constant::Methodref(_) => "Methodref",
            Constant::InterfaceMethodref(_) => "InterfaceMethodref",
            Constant::String(_) => "String",
            Constant::Integer(_) => "Integer",
            Constant::Float {..} => "Float",
            Constant::Long {..} => "Long",
            Constant::Double {..} => "Double",
            Constant::NameAndType(_) => "NameAndType",
            Constant::Utf8(_) => "Utf8",
            Constant::MethodHandle {..} => "MethodHandle",
            Constant::MethodType {..} => "MethodType",
            Constant::InvokeDynamic {..} => "InvokeDynamic",

        }
    }

    pub fn as_class(&self) -> u16 {
        match *self {
            Constant::Class(class) => class,
            _ => panic!("Constant is of incorrect type! Expected Class but was {}", self.name()) 
        }
    }

    pub fn as_fieldref(&self) -> &TypedEntityConstant {
        match *self {
            Constant::Fieldref(ref entity) => entity,
            _ => panic!("Constant is of incorrect type! Expected Fieldref but was {}", self.name()) 
        }
    }

    pub fn as_methodref(&self) -> &TypedEntityConstant {
        match *self {
            Constant::Methodref(ref entity) => entity,
            _ => panic!("Constant is of incorrect type! Expected Methodref but was {}", self.name()) 
        }
    }

    pub fn as_interface_methodref(&self) -> &TypedEntityConstant {
        match *self {
            Constant::InterfaceMethodref(ref entity) => entity,
            _ => panic!("Constant is of incorrect type! Expected InterfaceMethodref but was {}", self.name()) 
        }
    }

    pub fn as_string(&self) -> u16 {
        match *self {
            Constant::String(name_index) => name_index,
            _ => panic!("Constant is of incorrect type! Expected String but was {}", self.name()) 
        }
    }

    pub fn as_utf8(&self) -> &String {
        match *self {
            Constant::Utf8(ref value) => value,
            _ => panic!("Constant is of incorrect type! Expected Utf8 but was {}", self.name()) 
        }
    }

    pub fn as_name_and_type(&self) -> &NameAndTypeConstant {
        match *self {
            Constant::NameAndType(ref name_and_type) => name_and_type,
            _ => panic!("Constant is of incorrect type! Expected NameAndType but was {}", self.name()) 
        }
    }
}
