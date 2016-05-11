use std::io;
use std::vec::Vec;
use std::ops::Index;

use util::*;
use error::{ClassResult, ClassError};

#[derive(Debug)]
pub struct ConstantPool {
    pool: Vec<ConstantPoolTag>,
}

impl ConstantPool {
    pub fn read<T: io::Read>(rdr: &mut T) -> ClassResult<ConstantPool> {
        let size = try!(read_u16(rdr));
        let mut constant_pool : Vec<ConstantPoolTag> = vec![];
        for _ in 0..(size - 1) {
            let entry = try!(ConstantPoolTag::read(rdr));
            constant_pool.push(entry);
        }
        Ok(ConstantPool {
            pool: constant_pool
        })
    }

    pub fn len(&self) -> u16 {
        self.pool.len() as u16 + 1
    }
}

impl Index<u16> for ConstantPool {
    type Output = ConstantPoolTag;

    fn index<'a>(&'a self, index: u16) -> &'a ConstantPoolTag {
        &self.pool[index as usize - 1]
    }
}


#[derive(Debug, Eq, PartialEq)]
pub enum ConstantPoolTag {
    Class {
        name_index: u16
    },
    Fieldref {
        class_index: u16,
        name_and_type_index: u16
    },
    Methodref {
        class_index: u16,
        name_and_type_index: u16
    },
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16
    },
    String {
        string_index: u16
    },
    Integer {
        bytes: u32
    },
    Float {
        bytes: u32
    },
    Long {
        high_bytes: u32,
        low_bytes: u32
    },
    Double {
        high_bytes: u32,
        low_bytes: u32
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16
    },
    Utf8(String),
    MethodHandle {
        reference_kind: u8,
        reference_index: u16
    },
    MethodType {
        descriptor_index: u16
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16
    }
}

impl ConstantPoolTag {
    pub fn read<T: io::Read>(rdr: &mut T) -> ClassResult<ConstantPoolTag> {
        // https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.4
        const CONSTANT_UTF8: u8                 = 1;
        const CONSTANT_INTEGER: u8              = 3;
        const CONSTANT_FLOAT: u8                = 4;
        const CONSTANT_LONG: u8                 = 5;
        const CONSTANT_DOUBLE: u8               = 6;
        const CONSTANT_CLASS: u8                = 7;
        const CONSTANT_STRING: u8               = 8;
        const CONSTANT_FIELDREF: u8             = 9;
        const CONSTANT_METHODREF: u8            = 10;
        const CONSTANT_INTERFACE_METHODREF: u8  = 11;
        const CONSTANT_NAME_AND_TYPE: u8        = 12;
        const CONSTANT_METHOD_HANDLE: u8        = 15;
        const CONSTANT_METHOD_TYPE: u8          = 16;
        const CONSTANT_INVOKE_DYNAMIC: u8       = 18;

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
                Ok(ConstantPoolTag::Utf8(value))
            }
            CONSTANT_INTEGER => {
                let bytes = try!(read_u32(rdr));
                Ok(ConstantPoolTag::Integer {
                    bytes: bytes
                })
            }
            CONSTANT_FLOAT => {
                let bytes = try!(read_u32(rdr));
                Ok(ConstantPoolTag::Float {
                    bytes: bytes
                })
            }
            CONSTANT_LONG => {
                let high_bytes = try!(read_u32(rdr));
                let low_bytes = try!(read_u32(rdr));
                Ok(ConstantPoolTag::Long {
                    high_bytes: high_bytes,
                    low_bytes: low_bytes
                })
            }
            CONSTANT_DOUBLE => {
                let high_bytes = try!(read_u32(rdr));
                let low_bytes = try!(read_u32(rdr));
                Ok(ConstantPoolTag::Double {
                    high_bytes: high_bytes,
                    low_bytes: low_bytes
                })
            }
            CONSTANT_CLASS => {
                let name_index = try!(read_u16(rdr));
                Ok(ConstantPoolTag::Class {
                    name_index: name_index
                })
            }
            CONSTANT_STRING => {
                let string_index = try!(read_u16(rdr));
                Ok(ConstantPoolTag::String {
                    string_index: string_index
                })
            }
            CONSTANT_FIELDREF => {
                let class_index = try!(read_u16(rdr));
                let name_and_type_index = try!(read_u16(rdr));
                Ok(ConstantPoolTag::Fieldref {
                    class_index: class_index,
                    name_and_type_index: name_and_type_index
                })
            }
            CONSTANT_METHODREF => {
                let class_index = try!(read_u16(rdr));
                let name_and_type_index = try!(read_u16(rdr));
                Ok(ConstantPoolTag::Methodref {
                    class_index: class_index,
                    name_and_type_index: name_and_type_index
                })
            }
            CONSTANT_INTERFACE_METHODREF => {
                let class_index = try!(read_u16(rdr));
                let name_and_type_index = try!(read_u16(rdr));
                Ok(ConstantPoolTag::InterfaceMethodref {
                    class_index: class_index,
                    name_and_type_index: name_and_type_index
                })
            }
            CONSTANT_NAME_AND_TYPE => {
                let name_index = try!(read_u16(rdr));
                let descriptor_index = try!(read_u16(rdr));
                Ok(ConstantPoolTag::NameAndType {
                    name_index: name_index,
                    descriptor_index: descriptor_index
                })
            }
            CONSTANT_METHOD_HANDLE => {
                let reference_kind = try!(read_u8(rdr));
                let reference_index = try!(read_u16(rdr));
                Ok(ConstantPoolTag::MethodHandle {
                    reference_kind: reference_kind,
                    reference_index: reference_index
                })
            }
            CONSTANT_METHOD_TYPE => {
                let descriptor_index = try!(read_u16(rdr));
                Ok(ConstantPoolTag::MethodType {
                    descriptor_index: descriptor_index
                })
            }
            CONSTANT_INVOKE_DYNAMIC => {
                let bootstrap_method_attr_index = try!(read_u16(rdr));
                let name_and_type_index = try!(read_u16(rdr));
                Ok(ConstantPoolTag::InvokeDynamic {
                    bootstrap_method_attr_index: bootstrap_method_attr_index,
                    name_and_type_index: name_and_type_index
                })
            }
            _ => Err(ClassError::InvalidConstantPoolTag(tag))
        }
    }
}

