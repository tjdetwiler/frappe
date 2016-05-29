extern crate frappe;

use std::fs::File;

use frappe::classfile::*;
use frappe::classfile::reader::ClassReader;

#[test]
fn test_load_hello_world_class() {
    // Given
    let mut file = File::open("test-classes/HelloWorld.class").unwrap();

    // When
    let classfile = ClassReader::new(&mut file).read_class().unwrap();
    let cp = &classfile.constants;

    // Then
    assert_eq!(0xcafebabe, classfile.magic);
    assert_eq!(52, classfile.major_version);
    assert_eq!(0, classfile.minor_version);
    assert_eq!(29, classfile.constants.len());
    assert_eq!(CLASS_ACC_PUBLIC |
               CLASS_ACC_SUPER,
               classfile.access_flags);

    // Constant pool entries
    assert_eq!(
        Constant::Methodref(TypedEntityConstant {
            class_index: 6,
            name_and_type_index: 15
        }),
        classfile.constants[1]);
    assert_eq!(
        Constant::Fieldref(TypedEntityConstant {
            class_index: 16,
            name_and_type_index: 17
        }),
        classfile.constants[2]);
    assert_eq!(
        Constant::String(18),
        classfile.constants[3]);
    assert_eq!(
        Constant::Methodref(TypedEntityConstant {
            class_index: 19,
            name_and_type_index: 20
        }),
        classfile.constants[4]);
    assert_eq!(
        Constant::Class(21),
        classfile.constants[5]);
    assert_eq!(
        Constant::Class(22),
        classfile.constants[6]);
    assert_eq!(
        "<init>",
        classfile.constants[7].as_utf8());
    assert_eq!(
        "()V",
        classfile.constants[8].as_utf8());
    assert_eq!(
        "Code",
        classfile.constants[9].as_utf8());
    assert_eq!(
        "LineNumberTable",
        classfile.constants[10].as_utf8());
    assert_eq!(
        "main",
        classfile.constants[11].as_utf8());
    assert_eq!(
        "([Ljava/lang/String;)V",
        classfile.constants[12].as_utf8());
    assert_eq!(
        "SourceFile",
        classfile.constants[13].as_utf8());
    assert_eq!(
        "HelloWorld.java",
        classfile.constants[14].as_utf8());
    assert_eq!(
        Constant::NameAndType(NameAndTypeConstant {
            name_index: 7,
            descriptor_index: 8
        }),
        classfile.constants[15]);
    assert_eq!(
        Constant::Class(23),
        classfile.constants[16]);
    assert_eq!(
        Constant::NameAndType(NameAndTypeConstant {
            name_index: 24,
            descriptor_index: 25
        }),
        classfile.constants[17]);
    assert_eq!(
        "Hello World!",
        classfile.constants[18].as_utf8());
    assert_eq!(
        Constant::Class(26),
        classfile.constants[19]);
    assert_eq!(
        Constant::NameAndType(NameAndTypeConstant {
            name_index: 27,
            descriptor_index: 28
        }),
        classfile.constants[20]);
    assert_eq!(
        "io/hcf/frappe/HelloWorld",
        classfile.constants[21].as_utf8());
    assert_eq!(
        "java/lang/Object",
        classfile.constants[22].as_utf8());
    assert_eq!(
        "java/lang/System",
        classfile.constants[23].as_utf8());
    assert_eq!(
        "out",
        classfile.constants[24].as_utf8());
    assert_eq!(
        "Ljava/io/PrintStream;",
        classfile.constants[25].as_utf8());
    assert_eq!(
        "java/io/PrintStream",
        classfile.constants[26].as_utf8());
    assert_eq!(
        "println",
        classfile.constants[27].as_utf8());
    assert_eq!(
        "(Ljava/lang/String;)V",
        classfile.constants[28].as_utf8());

    assert_eq!(0, classfile.fields.len());
    assert_eq!(0, classfile.fields.len());

    // 2 methods
    assert_eq!(2, classfile.methods.len());
    // ctor
    let ctor_info = &classfile.methods[0];
    assert_eq!(METHOD_ACC_PUBLIC,
               ctor_info.access_flags);
    assert_eq!(
        "<init>",
        classfile.constants[ctor_info.name_index].as_utf8());
    assert_eq!(
        "()V",
        classfile.constants[ctor_info.descriptor_index].as_utf8());
    // main
    let main_info = &classfile.methods[1];
    assert_eq!(METHOD_ACC_STATIC |
               METHOD_ACC_PUBLIC,
               main_info.access_flags);
    assert_eq!(
        "main",
        classfile.constants[main_info.name_index].as_utf8());
    assert_eq!(
        "([Ljava/lang/String;)V",
        classfile.constants[main_info.descriptor_index].as_utf8());

    // 1 attribute
    assert_eq!(1, classfile.attrs.len());
    assert_eq!(
        "HelloWorld.java",
        classfile.attrs.source_file(cp).unwrap());
}
