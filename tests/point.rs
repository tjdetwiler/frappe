extern crate frappe;

use std::fs::File;

use frappe::classfile;
use frappe::classfile::attr;
use frappe::classfile::method;
use frappe::classfile::field;
use frappe::classfile::reader::ClassReader;

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
    let this_class_desc = class.constant_pool[this_class_desc].as_utf8();
    assert_eq!(
        "io/hcf/frappe/Point",
        this_class_desc);
    let super_class_desc = class.constant_pool[class.super_class].as_class();
    let super_class_desc = class.constant_pool[super_class_desc].as_utf8();
    assert_eq!(
        "java/lang/Object",
        super_class_desc);

    // Fields
    let x_field = class.find_field("x").unwrap();
    assert_eq!(
        field::FIELD_ACC_PRIVATE,
        x_field.access_flags);
    assert_eq!(
        "I",
        class.constant_pool[x_field.descriptor_index].as_utf8());

    let y_field = class.find_field("y").unwrap();
    assert_eq!(
        field::FIELD_ACC_PRIVATE,
        y_field.access_flags);
    assert_eq!(
        "I",
        class.constant_pool[y_field.descriptor_index].as_utf8());

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
