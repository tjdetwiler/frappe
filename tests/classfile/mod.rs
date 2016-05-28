use std::fs::File;

use frappe::classfile;
use frappe::classfile::cp;
use frappe::classfile::attr;
use frappe::classfile::cp::Constant;
use frappe::classfile::method;
use frappe::classfile::field;
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
    assert_utf8_tag(
        "Hello World!",
        &classfile.constant_pool[18]);
    assert_eq!(
        Constant::Class(26),
        classfile.constant_pool[19]);
    assert_eq!(
        Constant::NameAndType(cp::NameAndTypeConstant {
            name_index: 27,
            descriptor_index: 28
        }),
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
    assert_utf8_tag( "(Ljava/lang/String;)V",
        &classfile.constant_pool[28]);

    assert_eq!(0, classfile.fields.len());
    assert_eq!(0, classfile.fields.len());

    // 2 methods
    assert_eq!(2, classfile.methods.len());
    // ctor
    let ctor_info = &classfile.methods[0];
    assert_eq!(method::METHOD_ACC_PUBLIC,
               ctor_info.access_flags);
    assert_utf8_tag(
        "<init>",
        &classfile.constant_pool[ctor_info.name_index]);
    assert_utf8_tag(
        "()V",
        &classfile.constant_pool[ctor_info.descriptor_index]);
    // main
    let main_info = &classfile.methods[1];
    assert_eq!(method::METHOD_ACC_STATIC |
               method::METHOD_ACC_PUBLIC,
               main_info.access_flags);
    assert_utf8_tag(
        "main",
        &classfile.constant_pool[main_info.name_index]);
    assert_utf8_tag(
        "([Ljava/lang/String;)V",
        &classfile.constant_pool[main_info.descriptor_index]);

    // 1 attribute
    assert_eq!(1, classfile.attributes.len());
    assert_eq!(
        "HelloWorld.java",
        classfile.attributes.source_file(cp).unwrap());
}

#[test]
fn should_load_point_class() {
    // Given
    let mut file = File::open("test-classes/Point.class").unwrap();

    // When
    let class = ClassReader::new(&mut file).read_class().unwrap();
    let cp = &class.constant_pool;

    // Then
    assert_eq!(2, class.fields.len());
    assert_eq!(classfile::CLASS_ACC_PUBLIC |
               classfile::CLASS_ACC_SUPER,
               class.access_flags);
    let this_class_desc = class.constant_pool[class.this_class].as_class();
    let this_class_desc = &class.constant_pool[this_class_desc];
    assert_utf8_tag("io/hcf/frappe/Point", &this_class_desc);
    let super_class_desc = class.constant_pool[class.super_class].as_class();
    let super_class_desc = &class.constant_pool[super_class_desc];
    assert_utf8_tag("java/lang/Object", &super_class_desc);

    // Fields
    let x_field = class.find_field("x").unwrap();
    assert_eq!(field::FIELD_ACC_PRIVATE, x_field.access_flags);
    assert_utf8_tag("I", &class.constant_pool[x_field.descriptor_index]);

    let y_field = class.find_field("y").unwrap();
    assert_eq!(field::FIELD_ACC_PRIVATE, y_field.access_flags);
    assert_utf8_tag("I", &class.constant_pool[y_field.descriptor_index]);

    // Methods
    let ctor_method = class.find_method("<init>").unwrap();
    assert_eq!(
        "(II)V",
        class.constant_pool[ctor_method.descriptor_index].as_utf8());
    assert_eq!(
        method::METHOD_ACC_PUBLIC,
        ctor_method.access_flags);
    let ctor_code = ctor_method.attributes.code().unwrap();
    assert_eq!(
        2,
        ctor_code.max_stack);
    assert_eq!(
        3,
        ctor_code.max_locals);
    let line_number_table = ctor_code.attributes.line_number_table().unwrap();
    assert_eq!(
        4,
        line_number_table.len());
    assert_eq!(
        attr::LineNumberTableEntry { start_pc: 0 , line_number: 7 },
        line_number_table[0]);

    assert_eq!(1, class.attributes.len());
    assert_eq!(
        "Point.java", 
        class.attributes.source_file(cp).unwrap());
}

fn assert_utf8_tag(value: &str, tag: &Constant) {
    let expected_tag = Constant::Utf8(value.into());
    assert_eq!(expected_tag, *tag);
}
