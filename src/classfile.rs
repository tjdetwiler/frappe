use byteorder::{BigEndian, ReadBytesExt};
use std::vec::Vec;
use std::fmt;
use std::io;
use std::ops::Index;
use std::result;
use std::error;
use std::convert::From;

#[derive(Debug)]
pub enum Error {
    IOError,
    InvalidConstantPoolTag(u8)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BOOM")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "BOOM"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Error {
        Error::IOError
    }
}

pub type Result<T> = result::Result<T, Error>;

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
    Utf8 {
        length: u16,
        bytes: Vec<u8>
    },
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

#[derive(Debug)]
pub struct ConstantPool {
    pool: Vec<ConstantPoolTag>,
}

fn read_u8<T: io::Read>(rdr: &mut T) -> Result<u8> {
    match rdr.read_u8() {
        Ok(val) => Ok(val),
        Err(_) => Err(Error::IOError)
    }
}

fn read_u16<T: io::Read>(rdr: &mut T) -> Result<u16> {
    match rdr.read_u16::<BigEndian>() {
        Ok(val) => Ok(val),
        Err(_) => Err(Error::IOError)
    }
}

fn read_u32<T: io::Read>(rdr: &mut T) -> Result<u32> {
    match rdr.read_u32::<BigEndian>() {
        Ok(val) => Ok(val),
        Err(_) => Err(Error::IOError)
    }
}

impl ConstantPool {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<ConstantPool> {
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

pub const ACC_PUBLIC: u16       = 0x0001;
pub const ACC_PRIVATE: u16      = 0x0002;
pub const ACC_PROTECTED: u16    = 0x0004;
pub const ACC_STATIC: u16       = 0x0008;
pub const ACC_FINAL: u16        = 0x0010;
pub const ACC_SUPER: u16        = 0x0020;
pub const ACC_VOLATILE: u16     = 0x0040;
pub const ACC_TRANSIENT: u16    = 0x0080;
pub const ACC_INTERFACE: u16    = 0x0200;
pub const ACC_ABSTRACT: u16     = 0x0400;
pub const ACC_SYNTHETIC: u16    = 0x1000;
pub const ACC_ANNOTATION: u16   = 0x1000;
pub const ACC_ENUM: u16         = 0x4000;


impl ConstantPoolTag {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<ConstantPoolTag> {
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
                Ok(ConstantPoolTag::Utf8 {
                    length: length,
                    bytes: bytes
               })
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
            _ => Err(Error::InvalidConstantPoolTag(tag))
        }
    }
}

#[derive(Debug)]
pub struct AttributeInfo {
    attribute_name_index: u16,
    attribute_name_length: u32,
    info: Vec<u8>
}

impl AttributeInfo {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<AttributeInfo> {
        let attribute_name_index = try!(read_u16(rdr));
        let attribute_name_length = try!(read_u32(rdr));
        let mut info: Vec<u8> = vec![];
        for _ in 0..attribute_name_length {
            let byte = try!(read_u8(rdr));
            info.push(byte);
        }
        Ok(AttributeInfo {
            attribute_name_index: attribute_name_index,
            attribute_name_length: attribute_name_length,
            info: info
        })
    }
}

#[derive(Debug)]
pub struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attribute_info: Vec<AttributeInfo>
}

impl FieldInfo {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<FieldInfo> {
        let access_flags = try!(read_u16(rdr));
        let name_index = try!(read_u16(rdr));
        let descriptor_index = try!(read_u16(rdr));
        let attributes_count = try!(read_u16(rdr));
        let mut attribute_info: Vec<AttributeInfo> = vec![];
        for _ in 0..attributes_count {
            let attribute = try!(AttributeInfo::read(rdr));
            attribute_info.push(attribute);
        }
        Ok(FieldInfo {
            access_flags: access_flags,
            name_index: name_index,
            descriptor_index: descriptor_index,
            attributes_count: attributes_count,
            attribute_info: attribute_info
        })
    }
}

#[derive(Debug)]
pub struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attribute_info: Vec<AttributeInfo>
}

impl MethodInfo {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<MethodInfo> {
        let access_flags = try!(read_u16(rdr));
        let name_index = try!(rdr.read_u16::<BigEndian>());
        let descriptor_index = try!(rdr.read_u16::<BigEndian>());
        let attributes_count = try!(rdr.read_u16::<BigEndian>());
        let mut attribute_info: Vec<AttributeInfo> = vec![];
        for _ in 0..attributes_count {
            let attribute = try!(AttributeInfo::read(rdr));
            attribute_info.push(attribute);
        }
        Ok(MethodInfo {
            access_flags: access_flags,
            name_index: name_index,
            descriptor_index: descriptor_index,
            attributes_count: attributes_count,
            attribute_info: attribute_info
        })
    }
}

#[derive(Debug)]
pub struct ClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool: ConstantPool,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    field_info: Vec<FieldInfo>,
    method_info: Vec<MethodInfo>,
    attribute_info: Vec<AttributeInfo>
}

impl ClassFile {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<ClassFile> {
        let magic = try!(read_u32(rdr));
        let minor_version = try!(read_u16(rdr));
        let major_version = try!(read_u16(rdr));
        let constant_pool = try!(ConstantPool::read(rdr));
        let access_flags = try!(read_u16(rdr));
        let this_class = try!(read_u16(rdr));
        let super_class = try!(read_u16(rdr));
        let interfaces_count = try!(read_u16(rdr));
        let mut interfaces: Vec<u16> = vec![];
        for _ in 0..interfaces_count {
            let entry = try!(read_u16(rdr));
            interfaces.push(entry);
        }
        let fields_count = try!(read_u16(rdr));
        let mut field_info: Vec<FieldInfo> = vec![];
        for _ in 0..fields_count {
            let entry = try!(FieldInfo::read(rdr));
            field_info.push(entry);
        }
        let methods_count = try!(read_u16(rdr));
        let mut method_info: Vec<MethodInfo> = vec![];
        for _ in 0..methods_count {
            let entry = try!(MethodInfo::read(rdr));
            method_info.push(entry);
        }
        let attributes_count = try!(read_u16(rdr));
        let mut attribute_info: Vec<AttributeInfo> = vec![];
        for _ in 0..attributes_count {
            let entry = try!(AttributeInfo::read(rdr));
            attribute_info.push(entry);
        }
        Ok(ClassFile {
            magic: magic,
            minor_version: minor_version,
            major_version: major_version,
            constant_pool: constant_pool,
            access_flags: access_flags,
            this_class: this_class,
            super_class: super_class,
            interfaces: interfaces,
            field_info: field_info,
            method_info: method_info,
            attribute_info: attribute_info
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_load_hello_world_class() {
        // Given
        let mut file = File::open("test-classes/HelloWorld.class").unwrap();

        // When
        let classfile = ClassFile::read(&mut file).unwrap();

        // Then
        assert_eq!(0xcafebabe, classfile.magic);
        assert_eq!(52, classfile.major_version);
        assert_eq!(0, classfile.minor_version);
        assert_eq!(29, classfile.constant_pool.len());
        assert_eq!(
            ACC_PUBLIC | ACC_SUPER,
            classfile.access_flags);

        // Constant pool entries
        assert_eq!(
            ConstantPoolTag::Methodref {
                class_index: 6,
                name_and_type_index: 15
            },
            classfile.constant_pool[1]);
        assert_eq!(
            ConstantPoolTag::Fieldref {
                class_index: 16,
                name_and_type_index: 17
            },
            classfile.constant_pool[2]);
        assert_eq!(
            ConstantPoolTag::String {
                string_index: 18
            },
            classfile.constant_pool[3]);
        assert_eq!(
            ConstantPoolTag::Methodref {
                class_index: 19,
                name_and_type_index: 20
            },
            classfile.constant_pool[4]);
        assert_eq!(
            ConstantPoolTag::Class {
                name_index: 21
            },
            classfile.constant_pool[5]);
        assert_eq!(
            ConstantPoolTag::Class {
                name_index: 22
            },
            classfile.constant_pool[6]);
        assert_utf8_tag(
            "<init>",
            &classfile.constant_pool[7]);
        assert_utf8_tag(
            "()V",
            &classfile.constant_pool[8]);
        assert_utf8_tag(
            "Code",
            &classfile.constant_pool[9]);
        assert_utf8_tag(
            "LineNumberTable",
            &classfile.constant_pool[10]);
        assert_utf8_tag(
            "main",
            &classfile.constant_pool[11]);
        assert_utf8_tag(
            "([Ljava/lang/String;)V",
            &classfile.constant_pool[12]);
        assert_utf8_tag(
            "SourceFile",
            &classfile.constant_pool[13]);
        assert_utf8_tag(
            "HelloWorld.java",
            &classfile.constant_pool[14]);
        assert_eq!(
            ConstantPoolTag::NameAndType {
                name_index: 7,
                descriptor_index: 8
            },
            classfile.constant_pool[15]);
        assert_eq!(
            ConstantPoolTag::Class {
                name_index: 23
            },
            classfile.constant_pool[16]);
        assert_eq!(
            ConstantPoolTag::NameAndType {
                name_index: 24,
                descriptor_index: 25
            },
            classfile.constant_pool[17]);
        assert_utf8_tag(
            "Hello World!",
            &classfile.constant_pool[18]);
        assert_eq!(
            ConstantPoolTag::Class {
                name_index: 26
            },
            classfile.constant_pool[19]);
        assert_eq!(
            ConstantPoolTag::NameAndType {
                name_index: 27,
                descriptor_index: 28
            },
            classfile.constant_pool[20]);
        assert_utf8_tag(
            "io/hcf/frappe/HelloWorld",
            &classfile.constant_pool[21]);
        assert_utf8_tag(
            "java/lang/Object",
            &classfile.constant_pool[22]);
        assert_utf8_tag(
            "java/lang/System",
            &classfile.constant_pool[23]);
        assert_utf8_tag(
            "out",
            &classfile.constant_pool[24]);
        assert_utf8_tag(
            "Ljava/io/PrintStream;",
            &classfile.constant_pool[25]);
        assert_utf8_tag(
            "java/io/PrintStream",
            &classfile.constant_pool[26]);
        assert_utf8_tag(
            "println",
            &classfile.constant_pool[27]);
        assert_utf8_tag(
            "(Ljava/lang/String;)V",
            &classfile.constant_pool[28]);

        assert_eq!(0, classfile.interfaces.len());
        assert_eq!(0, classfile.field_info.len());

        // 2 methods
        assert_eq!(2, classfile.method_info.len());
        // ctor
        let ctor_info = &classfile.method_info[0];
        assert_eq!(
            ACC_PUBLIC,
            ctor_info.access_flags);
        assert_utf8_tag(
            "<init>",
            &classfile.constant_pool[ctor_info.name_index]);
        assert_utf8_tag(
            "()V",
            &classfile.constant_pool[ctor_info.descriptor_index]);
        // main
        let main_info = &classfile.method_info[1];
        assert_eq!(
            ACC_PUBLIC | ACC_STATIC,
            main_info.access_flags);
        assert_utf8_tag(
            "main",
            &classfile.constant_pool[main_info.name_index]);
        assert_utf8_tag(
            "([Ljava/lang/String;)V",
            &classfile.constant_pool[main_info.descriptor_index]);

        // 1 attribute
        assert_eq!(1, classfile.attribute_info.len());
    }

    fn assert_utf8_tag(value: &str, tag: &ConstantPoolTag) {
        let expected_length = value.len() as u16;
        let expected_bytes = value.as_bytes().to_vec();
        let expected_tag = ConstantPoolTag::Utf8 {
            length: expected_length,
            bytes: expected_bytes
        };
        assert_eq!(expected_tag, *tag);
    }
}

