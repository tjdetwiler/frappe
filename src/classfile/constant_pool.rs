use std::ops::{Deref, Index};

#[derive(Debug)]
pub struct ConstantPool {
    pub pool: Vec<Tag>,
}

impl ConstantPool {
    pub fn len(&self) -> u16 {
        self.pool.len() as u16 + 1
    }

    pub fn get_string(&self, index: u16) -> Option<&String> {
        self[index].as_utf8()
    }

    pub fn get_class(&self, index: u16) -> Option<&ClassTag> {
        if let Tag::Class(ref class_tag) = self[index] {
            Some(class_tag)
        } else {
            None
        }
    }
}

impl Index<u16> for ConstantPool {
    type Output = Tag;

    fn index<'a>(&'a self, index: u16) -> &'a Tag {
        &self.pool[index as usize - 1]
    }
}

impl Deref for ConstantPool {
    type Target = Vec<Tag>;

    fn deref(&self) -> &Vec<Tag> {
        &self.pool
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ClassTag {
    pub name_index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TypedEntityTag {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct StringTag {
    pub string_index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct NameAndTypeTag {
    pub name_index: u16,
    pub descriptor_index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Tag {
    Class(ClassTag),
    Fieldref(TypedEntityTag),
    Methodref(TypedEntityTag),
    InterfaceMethodref(TypedEntityTag),
    String(StringTag),
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
    NameAndType(NameAndTypeTag),
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

impl Tag {
    pub fn as_class(&self) -> Option<&ClassTag> {
        match *self {
            Tag::Class(ref class) => Some(class),
            _ => None,
        }
    }

    pub fn as_name_and_type(&self) -> Option<&NameAndTypeTag> {
        match *self {
            Tag::NameAndType(ref name_and_type) => Some(name_and_type),
            _ => None,
        }
    }

    pub fn as_utf8(&self) -> Option<&String> {
        match *self {
            Tag::Utf8(ref value) => Some(value),
            _ => None,
        }
    }
}
