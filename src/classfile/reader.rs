use std::io;

use byteorder::{BigEndian, ReadBytesExt};

use classfile::error::*;
use classfile::{ClassFile, ClassAccessFlags};
use classfile::field::{FieldInfo, FieldAccessFlags};
use classfile::method::{MethodInfo, MethodAccessFlags};
use classfile::attr::*;
use classfile::constant_pool::*;

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

pub struct ClassReader<T: io::Read> {
    reader: Box<T>,
}

impl<T: io::Read> ClassReader<T> {
    pub fn new(reader: T) -> ClassReader<T> {
        ClassReader { reader: Box::new(reader) }
    }

    pub fn read_class(&mut self) -> Result<ClassFile> {
        let magic = try!(self.read_u32());
        let minor_version = try!(self.read_u16());
        let major_version = try!(self.read_u16());
        let constant_pool = try!(self.read_constant_pool());
        let access_flags = try!(self.read_u16());
        let this_class_index = try!(self.read_u16());
        let super_class_index = try!(self.read_u16());
        let interfaces_count = try!(self.read_u16());
        let mut interfaces: Vec<u16> = vec![];
        for _ in 0..interfaces_count {
            let entry = try!(self.read_u16());
            interfaces.push(entry);
        }
        let fields = try!(self.read_fields(&constant_pool));
        let methods = try!(self.read_methods(&constant_pool));
        let attributes = try!(self.read_attributes(&constant_pool));

        Ok(ClassFile {
            magic: magic,
            minor_version: minor_version,
            major_version: major_version,
            constant_pool: constant_pool,
            access_flags: ClassAccessFlags::new(access_flags),
            this_class_index: this_class_index,
            super_class_index: super_class_index,
            interfaces: interfaces,
            fields: fields,
            methods: methods,
            attributes: attributes,
        })
    }

    fn read_methods(&mut self, constant_pool: &ConstantPool) -> Result<Vec<MethodInfo>> {
        let methods_count = try!(self.read_u16());
        let mut methods: Vec<MethodInfo> = vec![];
        for _ in 0..methods_count {
            let access_flags = try!(self.read_u16());
            let name_index = try!(self.read_u16());
            let descriptor_index = try!(self.read_u16());
            let attributes = try!(self.read_attributes(constant_pool));
            let entry = MethodInfo {
                access_flags: MethodAccessFlags::from_bits_truncate(access_flags),
                name_index: name_index,
                descriptor_index: descriptor_index,
                attributes: attributes,
            };
            methods.push(entry);
        }
        Ok(methods)
    }

    fn read_fields(&mut self, constant_pool: &ConstantPool) -> Result<Vec<FieldInfo>> {
        let fields_count = try!(self.read_u16());
        let mut fields: Vec<FieldInfo> = vec![];
        for _ in 0..fields_count {
            let access_flags = try!(self.read_u16());
            let name_index = try!(self.read_u16());
            let descriptor_index = try!(self.read_u16());
            let attributes: Attributes = try!(self.read_attributes(constant_pool));
            let entry = FieldInfo {
                access_flags: FieldAccessFlags::from_bits_truncate(access_flags),
                name_index: name_index,
                descriptor_index: descriptor_index,
                attributes: attributes,
            };
            fields.push(entry);
        }
        Ok(fields)
    }

    fn read_constant_pool(&mut self) -> Result<ConstantPool> {
        let size = try!(self.read_u16());
        let mut constant_pool: Vec<Tag> = vec![];
        for _ in 0..(size - 1) {
            let tag = try!(self.read_u8());
            let entry = match tag {
                CONSTANT_UTF8 => {
                    let length = try!(self.read_u16());
                    let mut bytes: Vec<u8> = vec![];
                    for _ in 0..length {
                        let byte = try!(self.read_u8());
                        bytes.push(byte);
                    }
                    let value = try!(String::from_utf8(bytes));
                    Ok(Tag::Utf8(value))
                }
                CONSTANT_INTEGER => {
                    let bytes = try!(self.read_u32());
                    Ok(Tag::Integer(bytes as i32))
                }
                CONSTANT_FLOAT => {
                    let bytes = try!(self.read_u32());
                    Ok(Tag::Float { bytes: bytes })
                }
                CONSTANT_LONG => {
                    let high_bytes = try!(self.read_u32());
                    let low_bytes = try!(self.read_u32());
                    Ok(Tag::Long {
                        high_bytes: high_bytes,
                        low_bytes: low_bytes,
                    })
                }
                CONSTANT_DOUBLE => {
                    let high_bytes = try!(self.read_u32());
                    let low_bytes = try!(self.read_u32());
                    Ok(Tag::Double {
                        high_bytes: high_bytes,
                        low_bytes: low_bytes,
                    })
                }
                CONSTANT_CLASS => {
                    let name_index = try!(self.read_u16());
                    Ok(Tag::Class(ClassTag { name_index: name_index }))
                }
                CONSTANT_STRING => {
                    let string_index = try!(self.read_u16());
                    Ok(Tag::String(StringTag { string_index: string_index }))
                }
                CONSTANT_FIELDREF => {
                    let class_index = try!(self.read_u16());
                    let name_and_type_index = try!(self.read_u16());
                    Ok(Tag::Fieldref(TypedEntityTag {
                        class_index: class_index,
                        name_and_type_index: name_and_type_index,
                    }))
                }
                CONSTANT_METHODREF => {
                    let class_index = try!(self.read_u16());
                    let name_and_type_index = try!(self.read_u16());
                    Ok(Tag::Methodref(TypedEntityTag {
                        class_index: class_index,
                        name_and_type_index: name_and_type_index,
                    }))
                }
                CONSTANT_INTERFACE_METHODREF => {
                    let class_index = try!(self.read_u16());
                    let name_and_type_index = try!(self.read_u16());
                    Ok(Tag::InterfaceMethodref(TypedEntityTag {
                        class_index: class_index,
                        name_and_type_index: name_and_type_index,
                    }))
                }
                CONSTANT_NAME_AND_TYPE => {
                    let name_index = try!(self.read_u16());
                    let descriptor_index = try!(self.read_u16());
                    Ok(Tag::NameAndType(NameAndTypeTag {
                        name_index: name_index,
                        descriptor_index: descriptor_index,
                    }))
                }
                CONSTANT_METHOD_HANDLE => {
                    let reference_kind = try!(self.read_u8());
                    let reference_index = try!(self.read_u16());
                    Ok(Tag::MethodHandle {
                        reference_kind: reference_kind,
                        reference_index: reference_index,
                    })
                }
                CONSTANT_METHOD_TYPE => {
                    let descriptor_index = try!(self.read_u16());
                    Ok(Tag::MethodType { descriptor_index: descriptor_index })
                }
                CONSTANT_INVOKE_DYNAMIC => {
                    let bootstrap_method_attr_index = try!(self.read_u16());
                    let name_and_type_index = try!(self.read_u16());
                    Ok(Tag::InvokeDynamic {
                        bootstrap_method_attr_index: bootstrap_method_attr_index,
                        name_and_type_index: name_and_type_index,
                    })
                }
                _ => Err(Error::InvalidConstantPoolTag(tag)),
            };
            let entry = try!(entry);
            constant_pool.push(entry);
        }
        Ok(ConstantPool { pool: constant_pool })
    }

    fn read_attributes(&mut self, constant_pool: &ConstantPool) -> Result<Attributes> {
        let num_attributes = try!(self.read_u16());
        let mut attributes: Vec<AttributeInfo> = vec![];
        for _ in 0..num_attributes {
            let attribute_info = try!(self.read_attribute(constant_pool));
            attributes.push(attribute_info);
        }
        Ok(Attributes { attributes: attributes })
    }

    fn read_attribute(&mut self, constant_pool: &ConstantPool) -> Result<AttributeInfo> {
        let name_index = try!(self.read_u16());
        if let Tag::Utf8(ref attribute_name) = constant_pool[name_index] {
            let attribute_length = try!(self.read_u32());
            match attribute_name.as_ref() {
                "SourceFile" => {
                    let sourcefile_index = try!(self.read_u16());
                    Ok(AttributeInfo::SourceFile { sourcefile_index: sourcefile_index })
                }
                "InnerClasses" => {
                    let inner_classes = try!(self.read_inner_classes_attribute());
                    Ok(AttributeInfo::InnerClasses(inner_classes))
                }
                "EnclosingMethod" => {
                    let class_index = try!(self.read_u16());
                    let method_index = try!(self.read_u16());
                    let enclosing_method = EnclosingMethodAttribute {
                        class_index: class_index,
                        method_index: method_index,
                    };
                    Ok(AttributeInfo::EnclosingMethod(Box::new(enclosing_method)))
                }
                "SourceDebugExtension" => {
                    let mut debug_extension: Vec<u8> = vec![];
                    for _ in 0..attribute_length {
                        let byte = try!(self.read_u8());
                        debug_extension.push(byte);
                    }
                    Ok(AttributeInfo::SourceDebugExtension(debug_extension))
                }
                "BootstrapMethods" => {
                    let bootstrap_methods = try!(self.read_bootstrap_methods());
                    Ok(AttributeInfo::BootstrapMethods(bootstrap_methods))
                }
                "ConstantValue" => {
                    let constantvalue_index = try!(self.read_u16());
                    Ok(AttributeInfo::ConstantValue { constantvalue_index: constantvalue_index })
                }
                "Code" => {
                    let max_stack = try!(self.read_u16());
                    let max_locals = try!(self.read_u16());
                    let code_length = try!(self.read_u32());
                    let mut code: Vec<u8> = vec![];
                    for _ in 0..code_length {
                        let byte = try!(self.read_u8());
                        code.push(byte);
                    }
                    let exception_table_length = try!(self.read_u16());
                    let mut exception_table: Vec<ExceptionInfo> = vec![];
                    for _ in 0..exception_table_length {
                        let exception_info = try!(self.read_exception_info());
                        exception_table.push(exception_info);
                    }
                    let attributes = try!(self.read_attributes(constant_pool));
                    Ok(AttributeInfo::Code(Box::new(CodeAttribute {
                        max_stack: max_stack,
                        max_locals: max_locals,
                        code: code,
                        exception_table: exception_table,
                        attributes: attributes,
                    })))
                }
                "Exceptions" => {
                    let number_of_exceptions = try!(self.read_u16());
                    let mut exception_index_table: Vec<u16> = vec![];
                    for _ in 0..number_of_exceptions {
                        let exception_index = try!(self.read_u16());
                        exception_index_table.push(exception_index);
                    }
                    Ok(AttributeInfo::Exceptions(exception_index_table))
                }
                "LineNumberTable" => {
                    let line_number_table_length = try!(self.read_u16());
                    let mut line_number_table: Vec<LineNumberTableEntry> = vec![];
                    for _ in 0..line_number_table_length {
                        let start_pc = try!(self.read_u16());
                        let line_number = try!(self.read_u16());
                        let line_number_table_entry = LineNumberTableEntry {
                            start_pc: start_pc,
                            line_number: line_number,
                        };
                        line_number_table.push(line_number_table_entry);
                    }
                    Ok(AttributeInfo::LineNumberTable(line_number_table))
                }
                "LocalVariableTable" => {
                    let local_variable_table_length = try!(self.read_u16());
                    let mut local_variable_table: Vec<LocalVariableTableEntry> = vec![];
                    for _ in 0..local_variable_table_length {
                        let start_pc = try!(self.read_u16());
                        let length = try!(self.read_u16());
                        let name_index = try!(self.read_u16());
                        let descriptor_index = try!(self.read_u16());
                        let index = try!(self.read_u16());
                        let local_variable_table_entry = LocalVariableTableEntry {
                            start_pc: start_pc,
                            length: length,
                            name_index: name_index,
                            descriptor_index: descriptor_index,
                            index: index,
                        };
                        local_variable_table.push(local_variable_table_entry);
                    }
                    Ok(AttributeInfo::LocalVariableTable(local_variable_table))
                }
                "LocalVariableTypeTable" => {
                    let local_variable_type_table_length = try!(self.read_u16());
                    let mut local_variable_type_table: Vec<LocalVariableTypeTableEntry> = vec![];
                    for _ in 0..local_variable_type_table_length {
                        let start_pc = try!(self.read_u16());
                        let length = try!(self.read_u16());
                        let name_index = try!(self.read_u16());
                        let signature_index = try!(self.read_u16());
                        let index = try!(self.read_u16());
                        let entry = LocalVariableTypeTableEntry {
                            start_pc: start_pc,
                            length: length,
                            name_index: name_index,
                            signature_index: signature_index,
                            index: index,
                        };
                        local_variable_type_table.push(entry);
                    }
                    Ok(AttributeInfo::LocalVariableTypeTable(local_variable_type_table))
                }
                "StackMapTable" => {
                    let number_of_entries = try!(self.read_u16());
                    let mut entries: Vec<StackMapFrame> = vec![];
                    for _ in 0..number_of_entries {
                        let stack_map_frame = try!(self.read_stack_map_frame());
                        entries.push(stack_map_frame);
                    }
                    Ok(AttributeInfo::StackMapTable(entries))
                }
                "Synthetic" => Ok(AttributeInfo::Synthetic),
                "Deprecated" => Ok(AttributeInfo::Deprecated),
                "Signature" => {
                    let signature_index = try!(self.read_u16());
                    Ok(AttributeInfo::Signature { signature_index: signature_index })
                }
                "AnnotationDefault" => {
                    let element_value = try!(self.read_element_value());
                    Ok(AttributeInfo::AnnotationDefault(element_value))
                }
                "MethodParameters" => {
                    let parameters_count = try!(self.read_u8());
                    let mut parameters: Vec<MethodParameterInfo> = vec![];
                    for _ in 0..parameters_count {
                        let name_index = try!(self.read_u16());
                        let access_flags = try!(self.read_u16());
                        let parameter_info = MethodParameterInfo {
                            name_index: name_index,
                            access_flags:
                                MethodParameterAccessFlags::from_bits_truncate(access_flags),
                        };
                        parameters.push(parameter_info);
                    }
                    Ok(AttributeInfo::MethodParameters(parameters))
                }
                "RuntimeVisibleAnnotations" => {
                    let annotations = try!(self.read_annotations());
                    Ok(AttributeInfo::RuntimeVisibleAnnotations(annotations))
                }
                "RuntimeInvisibleAnnotations" => {
                    let annotations = try!(self.read_annotations());
                    Ok(AttributeInfo::RuntimeInvisibleAnnotations(annotations))
                }
                "RuntimeVisibleTypeAnnotations" => {
                    let annotations = try!(self.read_type_annotations());
                    Ok(AttributeInfo::RuntimeVisibleTypeAnnotations(annotations))
                }
                "RuntimeInvisibleTypeAnnotations" => {
                    let annotations = try!(self.read_type_annotations());
                    Ok(AttributeInfo::RuntimeInvisibleTypeAnnotations(annotations))
                }
                "RuntimeVisibleParameterAnnotations" => {
                    let annotations = try!(self.read_parameter_annotations());
                    Ok(AttributeInfo::RuntimeVisibleParameterAnnotations(annotations))

                }
                "RuntimeInvisibleParameterAnnotations" => {
                    let annotations = try!(self.read_parameter_annotations());
                    Ok(AttributeInfo::RuntimeInvisibleParameterAnnotations(annotations))
                }
                attr_name => {
                    println!("UNKNOWN ATTRIBUTE {}", attr_name);
                    let mut info: Vec<u8> = vec![];
                    for _ in 0..attribute_length {
                        let byte = try!(self.read_u8());
                        info.push(byte);
                    }
                    Ok(AttributeInfo::Raw(Box::new(info)))
                }
            }
        } else {
            Err(Error::IOError)
        }
    }

    fn read_bootstrap_methods(&mut self) -> Result<Vec<BootstrapMethodInfo>> {
        let num_bootstrap_methods = try!(self.read_u16());
        let mut bootstrap_methods: Vec<BootstrapMethodInfo> = vec![];
        for _ in 0..num_bootstrap_methods {
            let bootstrap_method_ref = try!(self.read_u16());
            let num_bootstrap_arguments = try!(self.read_u16());
            let mut bootstrap_arguments: Vec<u16> = vec![];
            for _ in 0..num_bootstrap_arguments {
                let bootstrap_argument = try!(self.read_u16());
                bootstrap_arguments.push(bootstrap_argument);
            }
            let bootstrap_method = BootstrapMethodInfo {
                bootstrap_method_ref: bootstrap_method_ref,
                bootstrap_arguments: bootstrap_arguments,
            };
            bootstrap_methods.push(bootstrap_method);
        }
        Ok(bootstrap_methods)

    }

    fn read_inner_classes_attribute(&mut self) -> Result<Vec<InnerClassInfo>> {
        let number_of_classes = try!(self.read_u16());
        let mut inner_classes: Vec<InnerClassInfo> = vec![];
        for _ in 0..number_of_classes {
            let inner_class_info_index = try!(self.read_u16());
            let outer_class_info_index = try!(self.read_u16());
            let inner_name_index = try!(self.read_u16());
            let inner_class_access_flags = try!(self.read_u16());
            let inner_class_info = InnerClassInfo {
                inner_class_info_index: inner_class_info_index,
                outer_class_info_index: outer_class_info_index,
                inner_name_index: inner_name_index,
                inner_class_access_flags:
                    InnerClassAccessFlags::from_bits_truncate(inner_class_access_flags),
            };
            inner_classes.push(inner_class_info);
        }
        Ok(inner_classes)
    }

    fn read_exception_info(&mut self) -> Result<ExceptionInfo> {
        let start_pc = try!(self.read_u16());
        let end_pc = try!(self.read_u16());
        let handler_pc = try!(self.read_u16());
        let catch_type = try!(self.read_u16());
        Ok(ExceptionInfo {
            start_pc: start_pc,
            end_pc: end_pc,
            handler_pc: handler_pc,
            catch_type: catch_type,
        })
    }

    fn read_stack_map_frame(&mut self) -> Result<StackMapFrame> {
        let frame_type = try!(self.read_u8());
        match frame_type {
            0...63 => Ok(StackMapFrame::SameFrame(SameFrame { frame_type: frame_type })),
            64...127 => {
                let verification_type_info = try!(self.read_verification_type_info());
                let frame = SameLocals1StackItemFrame {
                    frame_type: frame_type,
                    stack: [verification_type_info],
                };
                Ok(StackMapFrame::SameLocals1StackItemFrame(frame))
            }
            247 => {
                let offset_delta = try!(self.read_u16());
                let verification_type_info = try!(self.read_verification_type_info());
                let frame = SameLocals1StackItemFrameExtended {
                    offset_delta: offset_delta,
                    stack: [verification_type_info],
                };
                Ok(StackMapFrame::SameLocals1StackItemFrameExtended(frame))
            }
            248...250 => {
                let offset_delta = try!(self.read_u16());
                let frame = ChopFrame {
                    frame_type: frame_type,
                    offset_delta: offset_delta,
                };
                Ok(StackMapFrame::ChopFrame(frame))
            }
            251 => {
                let offset_delta = try!(self.read_u16());
                let frame = SameFrameExtended { offset_delta: offset_delta };
                Ok(StackMapFrame::SameFrameExtended(frame))
            }
            252...254 => {
                let offset_delta = try!(self.read_u16());
                let num_locals = frame_type - 251;
                let mut locals: Vec<VerificationTypeInfo> = vec![];
                for _ in 0..num_locals {
                    let verification_type_info = try!(self.read_verification_type_info());
                    locals.push(verification_type_info);
                }
                let frame = AppendFrame {
                    frame_type: frame_type,
                    offset_delta: offset_delta,
                    locals: locals,
                };
                Ok(StackMapFrame::AppendFrame(frame))
            }
            255 => {
                let offset_delta = try!(self.read_u16());
                let number_of_locals = try!(self.read_u16());
                let mut locals: Vec<VerificationTypeInfo> = vec![];
                for _ in 0..number_of_locals {
                    let verification_type_info = try!(self.read_verification_type_info());
                    locals.push(verification_type_info);
                }
                let number_of_stack_items = try!(self.read_u16());
                let mut stack: Vec<VerificationTypeInfo> = vec![];
                for _ in 0..number_of_stack_items {
                    let verification_type_info = try!(self.read_verification_type_info());
                    stack.push(verification_type_info);
                }
                let frame = FullFrame {
                    offset_delta: offset_delta,
                    locals: locals,
                    stack: stack,
                };
                Ok(StackMapFrame::FullFrame(frame))
            }
            _ => Err(Error::InvalidStackFrameType(frame_type)),
        }
    }

    fn read_verification_type_info(&mut self) -> Result<VerificationTypeInfo> {
        let tag = try!(self.read_u8());
        match tag {
            0x0 => Ok(VerificationTypeInfo::Top),
            0x1 => Ok(VerificationTypeInfo::Integer),
            0x2 => Ok(VerificationTypeInfo::Float),
            0x3 => Ok(VerificationTypeInfo::Double),
            0x4 => Ok(VerificationTypeInfo::Long),
            0x5 => Ok(VerificationTypeInfo::Null),
            0x6 => Ok(VerificationTypeInfo::UninitializedThis),
            0x7 => {
                let cpool_index = try!(self.read_u16());
                let object_variable_info = ObjectVariableInfo { cpool_index: cpool_index };
                Ok(VerificationTypeInfo::Object(object_variable_info))
            }
            0x8 => {
                let offset = try!(self.read_u16());
                let uninitialized_variable_info = UninitializedVariableInfo { offset: offset };
                Ok(VerificationTypeInfo::Uninitialized(uninitialized_variable_info))
            }
            _ => Err(Error::InvalidVerificationTypeInfoTag(tag)),
        }
    }

    fn read_parameter_annotations(&mut self) -> Result<Vec<Vec<Annotation>>> {
        let num_parameters = try!(self.read_u8());
        let mut parameter_annotations: Vec<Vec<Annotation>> = vec![];
        for _ in 0..num_parameters {
            let annotations = try!(self.read_annotations());
            parameter_annotations.push(annotations);
        }
        Ok(parameter_annotations)
    }

    fn read_annotations(&mut self) -> Result<Vec<Annotation>> {
        let num_annotations = try!(self.read_u16());
        let mut annotations: Vec<Annotation> = vec![];
        for _ in 0..num_annotations {
            let annotation = try!(self.read_annotation());
            annotations.push(annotation);
        }
        Ok(annotations)
    }

    fn read_element_value_pair(&mut self) -> Result<ElementValuePair> {
        let element_name_index = try!(self.read_u16());
        let value = try!(self.read_element_value());
        Ok(ElementValuePair {
            element_name_index: element_name_index,
            value: value,
        })
    }

    fn read_annotation(&mut self) -> Result<Annotation> {
        let type_index = try!(self.read_u16());
        let num_element_value_pairs = try!(self.read_u16());
        let mut element_value_pairs: Vec<ElementValuePair> = vec![];
        for _ in 0..num_element_value_pairs {
            let element_value_pair = try!(self.read_element_value_pair());
            element_value_pairs.push(element_value_pair);
        }
        Ok(Annotation {
            type_index: type_index,
            element_value_pairs: element_value_pairs,
        })
    }

    fn read_type_annotations(&mut self) -> Result<Vec<TypeAnnotation>> {
        let num_annotations = try!(self.read_u8());
        let mut annotations: Vec<TypeAnnotation> = vec![];
        for _ in 0..num_annotations {
            let annotation = try!(self.read_type_annotation());
            annotations.push(annotation);
        }
        Ok(annotations)
    }

    fn read_type_annotation(&mut self) -> Result<TypeAnnotation> {
        let target_info = try!(self.read_target_info());
        let target_path = try!(self.read_type_path());
        let type_index = try!(self.read_u16());
        let num_element_value_pairs = try!(self.read_u16());
        let mut element_value_pairs: Vec<ElementValuePair> = vec![];
        for _ in 0..num_element_value_pairs {
            let element_value_pair = try!(self.read_element_value_pair());
            element_value_pairs.push(element_value_pair);
        }
        Ok(TypeAnnotation {
            target_info: target_info,
            target_path: target_path,
            type_index: type_index,
            element_value_pairs: element_value_pairs,
        })
    }

    fn read_target_info(&mut self) -> Result<TargetInfo> {
        let target_type = try!(self.read_u8());
        match target_type {
            0x00...0x01 => {
                let type_parameter_index = try!(self.read_u8());
                Ok(TargetInfo::TypeParameter(type_parameter_index))
            }
            0x10 => {
                let supertype_index = try!(self.read_u8());
                Ok(TargetInfo::Supertype(supertype_index))
            }
            0x11...0x12 => {
                let type_parameter_index = try!(self.read_u8());
                let bound_index = try!(self.read_u8());
                Ok(TargetInfo::TypeParameterBound {
                    type_parameter_index: type_parameter_index,
                    bound_index: bound_index,
                })
            }
            0x13...0x15 => Ok(TargetInfo::Empty),
            0x16 => {
                let formal_parameter_index = try!(self.read_u8());
                Ok(TargetInfo::MethodFormalParameter(formal_parameter_index))
            }
            0x17 => {
                let throws_type_index = try!(self.read_u8());
                Ok(TargetInfo::Throws(throws_type_index))
            }
            0x40...0x41 => {
                let table_length = try!(self.read_u16());
                let mut table: Vec<LocalvarInfo> = vec![];
                for _ in 0..table_length {
                    let start_pc = try!(self.read_u16());
                    let length = try!(self.read_u16());
                    let index = try!(self.read_u16());
                    let localvar_target = LocalvarInfo {
                        start_pc: start_pc,
                        length: length,
                        index: index,
                    };
                    table.push(localvar_target);
                }
                Ok(TargetInfo::Localvar(table))
            }
            0x42 => {
                let exception_table_index = try!(self.read_u16());
                Ok(TargetInfo::Catch(exception_table_index))
            }
            0x43...0x46 => {
                let offset = try!(self.read_u16());
                Ok(TargetInfo::Offset(offset))
            }
            0x47...0x4b => {
                let offset = try!(self.read_u16());
                let type_argument_index = try!(self.read_u8());
                Ok(TargetInfo::TypeArgument {
                    offset: offset,
                    type_argument_index: type_argument_index,
                })
            }
            target_type => Err(Error::InvalidTargetTypeTag(target_type)),
        }
    }

    fn read_type_path(&mut self) -> Result<Vec<TypePathEntry>> {
        let path_length = try!(self.read_u8());
        let mut path: Vec<TypePathEntry> = vec![];
        for _ in 0..path_length {
            let type_path_kind = try!(self.read_u8());
            let type_argument_index = try!(self.read_u8());
            let path_entry = TypePathEntry {
                type_path_kind: type_path_kind,
                type_argument_index: type_argument_index,
            };
            path.push(path_entry);
        }
        Ok(path)

    }

    fn read_element_value(&mut self) -> Result<ElementValue> {
        let tag = try!(self.read_u8());
        match tag as char {
            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' => {
                let const_value_index = try!(self.read_u16());
                let const_value = ConstantValue {
                    tag: tag,
                    const_value_index: const_value_index,
                };
                Ok(ElementValue::ConstantValue(const_value))
            }
            'e' => {
                let type_name_index = try!(self.read_u16());
                let const_name_index = try!(self.read_u16());
                let enum_value = EnumConstValue {
                    type_name_index: type_name_index,
                    const_name_index: const_name_index,
                };
                Ok(ElementValue::EnumConstValue(enum_value))
            }
            'c' => {
                let class_info = try!(self.read_u16());
                Ok(ElementValue::ClassInfo(class_info))
            }
            '@' => {
                let annotation_value = try!(self.read_annotation());
                Ok(ElementValue::AnnotationValue(annotation_value))
            }
            '[' => {
                let num_values = try!(self.read_u16());
                let mut values: Vec<ElementValue> = vec![];
                for _ in 0..num_values {
                    let element_value = try!(self.read_element_value());
                    values.push(element_value);
                }
                let array_value = ArrayValue { values: values };
                Ok(ElementValue::ArrayValue(array_value))
            }
            _ => Err(Error::InvalidElementValueTag(tag)),
        }
    }

    fn read_u8(&mut self) -> io::Result<u8> {
        self.reader.read_u8()
    }

    fn read_u16(&mut self) -> io::Result<u16> {
        self.reader.read_u16::<BigEndian>()
    }

    fn read_u32(&mut self) -> io::Result<u32> {
        self.reader.read_u32::<BigEndian>()
    }
}
