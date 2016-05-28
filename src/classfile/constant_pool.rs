use std::ops::{Deref, Index};

#[derive(Debug)]
pub struct ConstantPool {
    pub pool: Vec<Constant>,
}

impl ConstantPool {
    pub fn len(&self) -> u16 {
        self.pool.len() as u16 + 1
    }

    pub fn get_string(&self, index: u16) -> Option<&String> {
        self[index].as_utf8()
    }

    pub fn get_class(&self, index: u16) -> Option<&ClassConstant> {
        if let Constant::Class(ref class_tag) = self[index] {
            Some(class_tag)
        } else {
            None
        }
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

#[derive(Debug, Eq, PartialEq)]
pub struct ClassConstant {
    pub name_index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TypedEntityConstant {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct StringConstant {
    pub string_index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct NameAndTypeConstant {
    pub name_index: u16,
    pub descriptor_index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Constant {
    Class(ClassConstant),
    Fieldref(TypedEntityConstant),
    Methodref(TypedEntityConstant),
    InterfaceMethodref(TypedEntityConstant),
    String(StringConstant),
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
    pub fn as_class(&self) -> Option<&ClassConstant> {
        match *self {
            Constant::Class(ref class) => Some(class),
            _ => None,
        }
    }

    pub fn as_name_and_type(&self) -> Option<&NameAndTypeConstant> {
        match *self {
            Constant::NameAndType(ref name_and_type) => Some(name_and_type),
            _ => None,
        }
    }

    pub fn as_utf8(&self) -> Option<&String> {
        match *self {
            Constant::Utf8(ref value) => Some(value),
            _ => None,
        }
    }
}
