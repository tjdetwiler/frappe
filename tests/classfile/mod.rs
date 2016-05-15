use std::fs::File;

use frappe::classfile::ClassFile;
use frappe::classfile::constant_pool as cp;
use frappe::classfile::constant_pool::Tag;

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
        Tag::Methodref(cp::MethodrefTag {
            class_index: 6,
            name_and_type_index: 15
        }),
        classfile.constant_pool[1]);
    assert_eq!(
        Tag::Fieldref(cp::FieldrefTag {
            class_index: 16,
            name_and_type_index: 17
        }),
        classfile.constant_pool[2]);
    assert_eq!(
        Tag::String(cp::StringTag {
            string_index: 18
        }),
        classfile.constant_pool[3]);
    assert_eq!(
        Tag::Methodref(cp::MethodrefTag {
            class_index: 19,
            name_and_type_index: 20
        }),
        classfile.constant_pool[4]);
    assert_eq!(
        Tag::Class(cp::ClassTag {
            name_index: 21
        }),
        classfile.constant_pool[5]);
    assert_eq!(
        Tag::Class(cp::ClassTag {
            name_index: 22
        }),
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
        Tag::Class(cp::ClassTag {
            name_index: 23
        }),
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
        Tag::Class(cp::ClassTag {
            name_index: 26
        }),
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

#[test]
fn should_load_point_class() {
    // Given
    let mut file = File::open("test-classes/Point.class").unwrap();

    // When
    let class = ClassFile::read(&mut file).unwrap();

    // Then
    assert_eq!(2, class.fields.len());
    assert!(!class.access_flags.is_interface());
    assert!(!class.access_flags.is_annotation());
    let this_class_desc = &class.constant_pool[class.this_class().name_index];
    assert_utf8_tag("io/hcf/frappe/Point", &this_class_desc);
    let super_class_desc = &class.constant_pool[class.super_class().unwrap().name_index];
    assert_utf8_tag("java/lang/Object", &super_class_desc);

    let x_field = &class.fields[0];
    assert_utf8_tag("x", &class.constant_pool[x_field.name_index]);
    assert!(x_field.access_flags.is_private());
    assert!(!x_field.access_flags.is_protected());
    assert!(!x_field.access_flags.is_public());
    assert!(!x_field.access_flags.is_static());
    assert_utf8_tag("I", &class.constant_pool[x_field.descriptor_index]);

    let y_field = &class.fields[1];
    assert_utf8_tag("y", &class.constant_pool[y_field.name_index]);
    assert!(y_field.access_flags.is_private());
    assert!(!y_field.access_flags.is_protected());
    assert!(!y_field.access_flags.is_public());
    assert!(!y_field.access_flags.is_static());
    assert_utf8_tag("I", &class.constant_pool[y_field.descriptor_index]);
}

fn assert_utf8_tag(value: &str, tag: &Tag) {
    let expected_tag = Tag::Utf8(value.into());
    assert_eq!(expected_tag, *tag);
}