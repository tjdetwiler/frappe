use std::io;
use std::vec::Vec;
use std::ops::{Deref, Index};

use util::*;
use classfile::error::{Result, Error};

#[derive(Debug)]
pub struct ConstantPool {
    pool: Vec<Tag>,
}

impl ConstantPool {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<ConstantPool> {
        let size = try!(read_u16(rdr));
        let mut constant_pool: Vec<Tag> = vec![];
        for _ in 0..(size - 1) {
            let entry = try!(Tag::read(rdr));
            constant_pool.push(entry);
        }
        Ok(ConstantPool { pool: constant_pool })
    }

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
pub struct FieldrefTag {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct MethodrefTag {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct InterfaceMethodrefTag {
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
    Fieldref(FieldrefTag),
    Methodref(MethodrefTag),
    InterfaceMethodref(InterfaceMethodrefTag),
    String(StringTag),
    Integer {
        bytes: u32,
    },
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
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<Tag> {
        // https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.4
        const CONSTANT_UTF8: u8 = 1;
        const CONSTANT_INTEGER: u8 = 3;
        const CONSTANT_FLOAT: u8 = 4;
        const CONSTANT_LONG: u8 = 5;
        const CONSTANT_DOUBLE: u8 = 6;
        const CONSTANT_CLASS: u8 = 7;
        const CONSTANT_STRING: u8 = 8;
        const CONSTANT_FIELDREF: u8 = 9;
        const CONSTANT_METHODREF: u8 = 10;
        const CONSTANT_INTERFACE_METHODREF: u8 = 11;
        const CONSTANT_NAME_AND_TYPE: u8 = 12;
        const CONSTANT_METHOD_HANDLE: u8 = 15;
        const CONSTANT_METHOD_TYPE: u8 = 16;
        const CONSTANT_INVOKE_DYNAMIC: u8 = 18;

        let tag = try!(read_u8(rdr));
        match tag {
            CONSTANT_UTF8 => {
                let length = try!(read_u16(rdr));
                let mut bytes: Vec<u8> = vec![];
                for _ in 0..length {
                    let byte = try!(read_u8(rdr));
                    bytes.push(byte);
                }
                let value = try!(String::from_utf8(bytes));
                Ok(Tag::Utf8(value))
            }
            CONSTANT_INTEGER => {
                let bytes = try!(read_u32(rdr));
                Ok(Tag::Integer { bytes: bytes })
            }
            CONSTANT_FLOAT => {
                let bytes = try!(read_u32(rdr));
                Ok(Tag::Float { bytes: bytes })
            }
            CONSTANT_LONG => {
                let high_bytes = try!(read_u32(rdr));
                let low_bytes = try!(read_u32(rdr));
                Ok(Tag::Long {
                    high_bytes: high_bytes,
                    low_bytes: low_bytes,
                })
            }
            CONSTANT_DOUBLE => {
                let high_bytes = try!(read_u32(rdr));
                let low_bytes = try!(read_u32(rdr));
                Ok(Tag::Double {
                    high_bytes: high_bytes,
                    low_bytes: low_bytes,
                })
            }
            CONSTANT_CLASS => {
                let name_index = try!(read_u16(rdr));
                Ok(Tag::Class(ClassTag { name_index: name_index }))
            }
            CONSTANT_STRING => {
                let string_index = try!(read_u16(rdr));
                Ok(Tag::String(StringTag { string_index: string_index }))
            }
            CONSTANT_FIELDREF => {
                let class_index = try!(read_u16(rdr));
                let name_and_type_index = try!(read_u16(rdr));
                Ok(Tag::Fieldref(FieldrefTag {
                    class_index: class_index,
                    name_and_type_index: name_and_type_index,
                }))
            }
            CONSTANT_METHODREF => {
                let class_index = try!(read_u16(rdr));
                let name_and_type_index = try!(read_u16(rdr));
                Ok(Tag::Methodref(MethodrefTag {
                    class_index: class_index,
                    name_and_type_index: name_and_type_index,
                }))
            }
            CONSTANT_INTERFACE_METHODREF => {
                let class_index = try!(read_u16(rdr));
                let name_and_type_index = try!(read_u16(rdr));
                Ok(Tag::InterfaceMethodref(InterfaceMethodrefTag {
                    class_index: class_index,
                    name_and_type_index: name_and_type_index,
                }))
            }
            CONSTANT_NAME_AND_TYPE => {
                let name_index = try!(read_u16(rdr));
                let descriptor_index = try!(read_u16(rdr));
                Ok(Tag::NameAndType(NameAndTypeTag {
                    name_index: name_index,
                    descriptor_index: descriptor_index,
                }))
            }
            CONSTANT_METHOD_HANDLE => {
                let reference_kind = try!(read_u8(rdr));
                let reference_index = try!(read_u16(rdr));
                Ok(Tag::MethodHandle {
                    reference_kind: reference_kind,
                    reference_index: reference_index,
                })
            }
            CONSTANT_METHOD_TYPE => {
                let descriptor_index = try!(read_u16(rdr));
                Ok(Tag::MethodType { descriptor_index: descriptor_index })
            }
            CONSTANT_INVOKE_DYNAMIC => {
                let bootstrap_method_attr_index = try!(read_u16(rdr));
                let name_and_type_index = try!(read_u16(rdr));
                Ok(Tag::InvokeDynamic {
                    bootstrap_method_attr_index: bootstrap_method_attr_index,
                    name_and_type_index: name_and_type_index,
                })
            }
            _ => Err(Error::InvalidConstantPoolTag(tag)),
        }
    }

    pub fn as_utf8(&self) -> Option<&String> {
        match *self {
            Tag::Utf8(ref value) => Some(value),
            _ => None,
        }
    }
}
