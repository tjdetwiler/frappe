use std::vec::Vec;
use std::fmt;
use std::io;

use util::*;
use error::{ClassResult, ClassError};
use attr::AttributeInfo;
use constant_pool::ConstantPool;
use field::FieldInfo;
use method::MethodInfo;

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

#[derive(Debug)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub field_info: Vec<FieldInfo>,
    pub method_info: Vec<MethodInfo>,
    pub attribute_info: Vec<AttributeInfo>
}

impl ClassFile {
    pub fn read<T: io::Read>(rdr: &mut T) -> ClassResult<ClassFile> {
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
        let expected_tag = ConstantPoolTag::Utf8(value.into());
        assert_eq!(expected_tag, *tag);
    }
}

