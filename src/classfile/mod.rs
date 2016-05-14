pub mod method;
pub mod field;
pub mod constant_pool;
pub mod attr;
pub mod error;

use std::vec::Vec;
use std::io;

use util::*;
use classfile::error::ClassResult;
use classfile::attr::Attributes;
use classfile::constant_pool::ConstantPool;
use classfile::field::Fields;
use classfile::method::Methods;


const ACC_PUBLIC: u16       = 0x0001;
const ACC_FINAL: u16        = 0x0010;
const ACC_SUPER: u16        = 0x0020;
const ACC_INTERFACE: u16    = 0x0200;
const ACC_ABSTRACT: u16     = 0x0400;
const ACC_SYNTHETIC: u16    = 0x1000;
const ACC_ANNOTATION: u16   = 0x2000;
const ACC_ENUM: u16         = 0x4000;

#[derive(Debug, Eq, PartialEq)]
pub struct ClassAccessFlags {
    access_flags: u16
}

impl ClassAccessFlags {
    fn new(access_flags: u16) -> ClassAccessFlags {
        ClassAccessFlags {
            access_flags: access_flags
        }
    }

    pub fn is_public(&self) -> bool {
        (self.access_flags & ACC_PUBLIC) != 0
    }

    pub fn is_final(&self) -> bool {
        (self.access_flags & ACC_FINAL) != 0
    }

    pub fn is_super(&self) -> bool {
        (self.access_flags & ACC_SUPER) != 0
    }

    pub fn is_interface(&self) -> bool {
        (self.access_flags & ACC_INTERFACE) != 0
    }

    pub fn is_abstract(&self) -> bool {
        (self.access_flags & ACC_ABSTRACT) != 0
    }

    pub fn is_synthetic(&self) -> bool {
        (self.access_flags & ACC_SYNTHETIC) != 0
    }

    pub fn is_annotation(&self) -> bool {
        (self.access_flags & ACC_ANNOTATION) != 0
    }

    pub fn is_enum(&self) -> bool {
        (self.access_flags & ACC_ENUM) != 0
    }
}

#[derive(Debug)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: ClassAccessFlags,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Fields,
    pub methods: Methods,
    pub attributes: Attributes
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
        let fields = try!(Fields::read(rdr, &constant_pool));
        let methods = try!(Methods::read(rdr, &constant_pool));
        let attributes = try!(Attributes::read(rdr, &constant_pool));
        Ok(ClassFile {
            magic: magic,
            minor_version: minor_version,
            major_version: major_version,
            constant_pool: constant_pool,
            access_flags: ClassAccessFlags::new(access_flags),
            this_class: this_class,
            super_class: super_class,
            interfaces: interfaces,
            fields: fields,
            methods: methods,
            attributes: attributes
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    use classfile::constant_pool::Tag;

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
        assert!(classfile.access_flags.is_public());
        assert!(classfile.access_flags.is_super());

        // Constant pool entries
        assert_eq!(
            Tag::Methodref {
                class_index: 6,
                name_and_type_index: 15
            },
            classfile.constant_pool[1]);
        assert_eq!(
            Tag::Fieldref {
                class_index: 16,
                name_and_type_index: 17
            },
            classfile.constant_pool[2]);
        assert_eq!(
            Tag::String {
                string_index: 18
            },
            classfile.constant_pool[3]);
        assert_eq!(
            Tag::Methodref {
                class_index: 19,
                name_and_type_index: 20
            },
            classfile.constant_pool[4]);
        assert_eq!(
            Tag::Class {
                name_index: 21
            },
            classfile.constant_pool[5]);
        assert_eq!(
            Tag::Class {
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
            Tag::NameAndType {
                name_index: 7,
                descriptor_index: 8
            },
            classfile.constant_pool[15]);
        assert_eq!(
            Tag::Class {
                name_index: 23
            },
            classfile.constant_pool[16]);
        assert_eq!(
            Tag::NameAndType {
                name_index: 24,
                descriptor_index: 25
            },
            classfile.constant_pool[17]);
        assert_utf8_tag(
            "Hello World!",
            &classfile.constant_pool[18]);
        assert_eq!(
            Tag::Class {
                name_index: 26
            },
            classfile.constant_pool[19]);
        assert_eq!(
            Tag::NameAndType {
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

        assert_eq!(0, classfile.fields.len());
        assert_eq!(0, classfile.fields.len());

        // 2 methods
        assert_eq!(2, classfile.methods.len());
        // ctor
        let ctor_info = &classfile.methods[0];
        assert!(ctor_info.access_flags.is_public());
        assert!(!ctor_info.access_flags.is_static());
        assert_utf8_tag(
            "<init>",
            &classfile.constant_pool[ctor_info.name_index]);
        assert_utf8_tag(
            "()V",
            &classfile.constant_pool[ctor_info.descriptor_index]);
        // main
        let main_info = &classfile.methods[1];
        assert!(main_info.access_flags.is_static());
        assert!(main_info.access_flags.is_public());
        assert_utf8_tag(
            "main",
            &classfile.constant_pool[main_info.name_index]);
        assert_utf8_tag(
            "([Ljava/lang/String;)V",
            &classfile.constant_pool[main_info.descriptor_index]);

        // 1 attribute
        assert_eq!(1, classfile.attributes.len());
    }

    fn assert_utf8_tag(value: &str, tag: &Tag) {
        let expected_tag = Tag::Utf8(value.into());
        assert_eq!(expected_tag, *tag);
    }
}

