extern crate frappe;

use std::fs::File;

use frappe::classfile;
use frappe::classfile::cp;
use frappe::classfile::cp::Constant;
use frappe::classfile::method;
use frappe::classfile::reader::ClassReader;

#[test]
fn test_load_hello_world_class() {
    // Given
    let mut file = File::open("test-classes/HelloWorld.class").unwrap();

    // When
    let classfile = ClassReader::new(&mut file).read_class().unwrap();
    let cp = &classfile.constant_pool;

    // Then
    assert_eq!(0xcafebabe, classfile.magic);
    assert_eq!(52, classfile.major_version);
    assert_eq!(0, classfile.minor_version);
    assert_eq!(29, classfile.constant_pool.len());
    assert_eq!(classfile::CLASS_ACC_PUBLIC |
               classfile::CLASS_ACC_SUPER,
               classfile.access_flags);

    // Constant pool entries
    assert_eq!(
        Constant::Methodref(cp::TypedEntityConstant {
            class_index: 6,
            name_and_type_index: 15
        }),
        classfile.constant_pool[1]);
    assert_eq!(
        Constant::Fieldref(cp::TypedEntityConstant {
            class_index: 16,
            name_and_type_index: 17
        }),
        classfile.constant_pool[2]);
    assert_eq!(
        Constant::String(18),
        classfile.constant_pool[3]);
    assert_eq!(
        Constant::Methodref(cp::TypedEntityConstant {
            class_index: 19,
            name_and_type_index: 20
        }),
        classfile.constant_pool[4]);
    assert_eq!(
        Constant::Class(21),
        classfile.constant_pool[5]);
    assert_eq!(
        Constant::Class(22),
        classfile.constant_pool[6]);
    assert_eq!(
        "<init>",
        classfile.constant_pool[7].as_utf8());
    assert_eq!(
        "()V",
        classfile.constant_pool[8].as_utf8());
    assert_eq!(
        "Code",
        classfile.constant_pool[9].as_utf8());
    assert_eq!(
        "LineNumberTable",
        classfile.constant_pool[10].as_utf8());
    assert_eq!(
        "main",
        classfile.constant_pool[11].as_utf8());
    assert_eq!(
        "([Ljava/lang/String;)V",
        classfile.constant_pool[12].as_utf8());
    assert_eq!(
        "SourceFile",
        classfile.constant_pool[13].as_utf8());
    assert_eq!(
        "HelloWorld.java",
        classfile.constant_pool[14].as_utf8());
    assert_eq!(
        Constant::NameAndType(cp::NameAndTypeConstant {
            name_index: 7,
            descriptor_index: 8
        }),
        classfile.constant_pool[15]);
    assert_eq!(
        Constant::Class(23),
        classfile.constant_pool[16]);
    assert_eq!(
        Constant::NameAndType(cp::NameAndTypeConstant {
            name_index: 24,
            descriptor_index: 25
        }),
        classfile.constant_pool[17]);
    assert_eq!(
        "Hello World!",
        classfile.constant_pool[18].as_utf8());
    assert_eq!(
        Constant::Class(26),
        classfile.constant_pool[19]);
    assert_eq!(
        Constant::NameAndType(cp::NameAndTypeConstant {
            name_index: 27,
            descriptor_index: 28
        }),
        classfile.constant_pool[20]);
    assert_eq!(
        "io/hcf/frappe/HelloWorld",
        classfile.constant_pool[21].as_utf8());
    assert_eq!(
        "java/lang/Object",
        classfile.constant_pool[22].as_utf8());
    assert_eq!(
        "java/lang/System",
        classfile.constant_pool[23].as_utf8());
    assert_eq!(
        "out",
        classfile.constant_pool[24].as_utf8());
    assert_eq!(
        "Ljava/io/PrintStream;",
        classfile.constant_pool[25].as_utf8());
    assert_eq!(
        "java/io/PrintStream",
        classfile.constant_pool[26].as_utf8());
    assert_eq!(
        "println",
        classfile.constant_pool[27].as_utf8());
    assert_eq!(
        "(Ljava/lang/String;)V",
        classfile.constant_pool[28].as_utf8());

    assert_eq!(0, classfile.fields.len());
    assert_eq!(0, classfile.fields.len());

    // 2 methods
    assert_eq!(2, classfile.methods.len());
    // ctor
    let ctor_info = &classfile.methods[0];
    assert_eq!(method::METHOD_ACC_PUBLIC,
               ctor_info.access_flags);
    assert_eq!(
        "<init>",
        classfile.constant_pool[ctor_info.name_index].as_utf8());
    assert_eq!(
        "()V",
        classfile.constant_pool[ctor_info.descriptor_index].as_utf8());
    // main
    let main_info = &classfile.methods[1];
    assert_eq!(method::METHOD_ACC_STATIC |
               method::METHOD_ACC_PUBLIC,
               main_info.access_flags);
    assert_eq!(
        "main",
        classfile.constant_pool[main_info.name_index].as_utf8());
    assert_eq!(
        "([Ljava/lang/String;)V",
        classfile.constant_pool[main_info.descriptor_index].as_utf8());

    // 1 attribute
    assert_eq!(1, classfile.attributes.len());
    assert_eq!(
        "HelloWorld.java",
        classfile.attributes.source_file(cp).unwrap());
}
