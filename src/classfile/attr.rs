use std::vec::Vec;
use std::ops::Deref;

use classfile::cp::ConstantPool;

#[derive(Debug, Eq, PartialEq)]
pub enum TargetInfo {
    TypeParameter(u8),
    Supertype(u8),
    TypeParameterBound {
        type_parameter_index: u8,
        bound_index: u8,
    },
    Empty,
    MethodFormalParameter(u8),
    Throws(u8),
    Localvar(Vec<LocalvarInfo>),
    Catch(u16),
    Offset(u16),
    TypeArgument {
        offset: u16,
        type_argument_index: u8,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub struct LocalvarInfo {
    pub start_pc: u16,
    pub length: u16,
    pub index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ElementValue {
    ConstantValue(ConstantValue),
    EnumConstValue(EnumConstValue),
    ClassInfo(u16),
    AnnotationValue(Annotation),
    ArrayValue(ArrayValue),
}

#[derive(Debug, Eq, PartialEq)]
pub struct ElementValuePair {
    pub element_name_index: u16,
    pub value: ElementValue,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ConstantValue {
    pub tag: u8,
    pub const_value_index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ArrayValue {
    pub values: Vec<ElementValue>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct EnumConstValue {
    pub type_name_index: u16,
    pub const_name_index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Annotation {
    pub type_index: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TypePathEntry {
    pub type_path_kind: u8,
    pub type_argument_index: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TypeAnnotation {
    pub target_info: TargetInfo,
    pub target_path: Vec<TypePathEntry>,
    pub type_index: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum VerificationTypeInfo {
    Top,
    Integer,
    Float,
    Long,
    Double,
    Null,
    UninitializedThis,
    Object(ObjectVariableInfo),
    Uninitialized(UninitializedVariableInfo),
}

#[derive(Debug, Eq, PartialEq)]
pub struct ObjectVariableInfo {
    pub cpool_index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UninitializedVariableInfo {
    pub offset: u16,
}

/// Defines an entry in a `StackMapTableAttribute`.
#[derive(Debug, Eq, PartialEq)]
pub enum StackMapFrame {
    SameFrame {
        frame_type: u8,
    },
    SameLocals1StackItemFrame {
        frame_type: u8,
        stack: [VerificationTypeInfo; 1],
    },
    SameLocals1StackItemFrameExtended {
        offset_delta: u16,
        stack: [VerificationTypeInfo; 1],
    },
    ChopFrame {
        frame_type: u8,
        offset_delta: u16,
    },
    SameFrameExtended {
        offset_delta: u16,
    },
    AppendFrame {
        frame_type: u8,
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
    },
    FullFrame {
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
        stack: Vec<VerificationTypeInfo>,
    },
}

bitflags! {
    pub flags MethodParameterAccessFlags: u16 {
        const MP_ACC_FINAL         = 0x0010,
        const MP_ACC_SYNTHETIC     = 0x1000,
        const MP_ACC_MANDATED      = 0x8000
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct MethodParameterInfo {
    pub name_index: u16,
    pub access_flags: MethodParameterAccessFlags,
}


#[derive(Debug, Eq, PartialEq)]
pub struct LocalVariableTypeTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct LocalVariableTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct LineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16,
}

bitflags! {
    pub flags InnerClassAccessFlags: u16 {
        const IC_ACC_PUBLIC        = 0x0001,
        const IC_ACC_PRIVATE       = 0x0002,
        const IC_ACC_PROTECTED     = 0x0004,
        const IC_ACC_STATIC        = 0x0008,
        const IC_ACC_FINAL         = 0x0010,
        const IC_ACC_INTERFACE     = 0x0200,
        const IC_ACC_ABSTRACT      = 0x0400,
        const IC_ACC_SYNTHETIC     = 0x1000,
        const IC_ACC_ANNOTATION    = 0x2000,
        const IC_ACC_ENUM          = 0x4000
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct InnerClassInfo {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: InnerClassAccessFlags,
}

#[derive(Debug, Eq, PartialEq)]
pub struct EnclosingMethodAttribute {
    pub class_index: u16,
    pub method_index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ExceptionInfo {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionInfo>,
    pub attrs: Attributes,
}

#[derive(Debug, Eq, PartialEq)]
pub struct BootstrapMethodInfo {
    pub bootstrap_method_ref: u16,
    pub bootstrap_arguments: Vec<u16>,
}

/// Indicates where a set of attributes is sourced from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttributeLocation {
    /// Attributes are associated with a `ClassFile` structure.
    ClassFile,
    /// Attributes are associated with a `FieldInfo` structure.
    FieldInfo,
    /// Attributes are associated with a `MethodInfo` structure.
    MethodInfo,
    /// Attributes are associated with a `CodeAttribute` structure.
    Code,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Attributes {
    location: AttributeLocation,
    attributes: Vec<AttributeInfo>,
}

impl Deref for Attributes {
    type Target = Vec<AttributeInfo>;

    fn deref(&self) -> &Vec<AttributeInfo> {
        &self.attributes
    }
}

impl Attributes {
    pub fn new(location: AttributeLocation, attributes: Vec<AttributeInfo>) -> Attributes {
        Attributes {
            location: location,
            attributes: attributes,
        }
    }

    pub fn location(&self) -> AttributeLocation {
        self.location
    }

    /// Resolves the source file attribute in this class if it exists and returns
    /// the value. If there is no source file attribute then `None` is returned.
    pub fn source_file<'a>(&self, cp: &'a ConstantPool) -> Option<&'a String> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::SourceFile(sourcefile_index) = *attr {
                let source_file = cp[sourcefile_index].as_utf8();
                return Some(source_file);
            }
        }
        None
    }

    /// Resolves the boostrap method attribute if it exists in this classes
    /// attributes and returns the value. Otherwise returns `None`.
    pub fn bootstrap_methods(&self) -> Option<&Vec<BootstrapMethodInfo>> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::BootstrapMethods(ref bootstrap_methods) = *attr {
                return Some(bootstrap_methods);
            }
        }
        None
    }

    /// Resolves the source debug extension attribute it if is present. Otherwise
    /// returns `None`.
    pub fn source_debug_extension(&self) -> Option<&Vec<u8>> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::SourceDebugExtension(ref extension) = *attr {
                return Some(extension);
            }
        }
        None
    }

    /// Resolves the enclosing method attribute if present. Otherwise returns
    /// `None`.
    pub fn enclosing_method(&self) -> Option<&EnclosingMethodAttribute> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::EnclosingMethod(ref enclosing_method) = *attr {
                return Some(enclosing_method);
            }
        }
        None
    }

    /// Resolves the inner classes attribute if present. Otherwise returns
    /// `None`.
    pub fn inner_classes(&self) -> Option<&Vec<InnerClassInfo>> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::InnerClasses(ref inner_classes) = *attr {
                return Some(inner_classes);
            }
        }
        None
    }

    /// If either `METHOD_ACC_ABSTRACT` or `METHOD_ACC_NATIVE` are set, this method
    /// should not have a code attribute. Otherwise it must have exactly one.
    pub fn code(&self) -> Option<&CodeAttribute> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::Code(ref code) = *attr {
                return Some(code);
            }
        }
        None
    }

    /// Returns the list of exceptions this method may throw. Each value in the
    /// returned vector is an entry in the constant pool that points to a
    /// `Constant::Class` value.
    pub fn exceptions(&self) -> Option<&Vec<u16>> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::Exceptions(ref exceptions) = *attr {
                return Some(exceptions);
            }
        }
        None
    }

    /// The annotation default attribute is provided on annotation methods that may
    /// have a default value.
    pub fn annotation_default(&self) -> Option<&ElementValue> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::AnnotationDefault(ref element_value) = *attr {
                return Some(element_value);
            }
        }
        None
    }

    /// Resolves the method parameters attribute and returns it if present. Otherwise
    /// returns `None`.
    pub fn method_parameters(&self) -> Option<&Vec<MethodParameterInfo>> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::MethodParameters(ref parameters) = *attr {
                return Some(parameters);
            }
        }
        None
    }

    /// Resolves the signature attribute to a UTF8 string if present. Otherwise
    /// returns `None`.
    pub fn signature<'a>(&self, cp: &'a ConstantPool) -> Option<&'a String> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::Signature(signature_index) = *attr {
                let signature = cp[signature_index].as_utf8();
                return Some(signature);
            }
        }
        None
    }

    /// Resolves the RuntimeVisibleAnnotations attribute and returns the list of
    /// annotations if it is present.
    pub fn runtime_visible_annotations(&self) -> Option<&Vec<Annotation>> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::RuntimeVisibleAnnotations(ref annotations) = *attr {
                return Some(annotations);
            }
        }
        None
    }

    /// Resolves the RuntimeInvisibleAnnotations attribute and returns the list of
    /// annotations if it is present.
    pub fn runtime_invisible_annotations(&self) -> Option<&Vec<Annotation>> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::RuntimeInvisibleAnnotations(ref annotations) = *attr {
                return Some(annotations);
            }
        }
        None
    }

    /// Resolves the RuntimeVisibleTypeAnnotations attribute and returns the list of
    /// annotations if it is present.
    pub fn runtime_visible_type_annotations(&self) -> Option<&Vec<TypeAnnotation>> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::RuntimeVisibleTypeAnnotations(ref annotations) = *attr {
                return Some(annotations);
            }
        }
        None
    }

    /// Resolves the RuntimeInvisibleTypeAnnotations attribute and returns the list of
    /// annotations if it is present.
    pub fn runtime_invisible_type_annotations(&self) -> Option<&Vec<TypeAnnotation>> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::RuntimeInvisibleTypeAnnotations(ref annotations) = *attr {
                return Some(annotations);
            }
        }
        None
    }

    /// Returns `true` iff the `AttributeInfo::Synthetic` attribute is present.
    pub fn is_synthetic(&self) -> bool {
        for attr in self.attributes.iter() {
            if let AttributeInfo::Synthetic = *attr {
                return true;
            }
        }
        false
    }

    /// Returns `true` iff the `AttributeInfo::Deprecated` attribute is present.
    pub fn is_deprecated(&self) -> bool {
        for attr in self.attributes.iter() {
            if let AttributeInfo::Deprecated = *attr {
                return true;
            }
        }
        false
    }

    /// Resolves the line number table attribute and returns the list of entries if
    /// it is present.
    pub fn line_number_table(&self) -> Option<&Vec<LineNumberTableEntry>> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::LineNumberTable(ref table) = *attr {
                return Some(table);
            }
        }
        None
    }

    /// Resolves the local variable table attribute and returns the list of entries if
    /// it is present.
    pub fn local_variable_table(&self) -> Option<&Vec<LocalVariableTableEntry>> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::LocalVariableTable(ref table) = *attr {
                return Some(table);
            }
        }
        None
    }

    /// Resolves the local variable type table attribute and returns the list of
    /// entries if it is present.
    pub fn local_variable_type_table(&self) -> Option<&Vec<LocalVariableTypeTableEntry>> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::LocalVariableTypeTable(ref table) = *attr {
                return Some(table);
            }
        }
        None
    }

    /// Resolves the constant value attribute of a field info structure and returns the index
    /// of the value if present.
    pub fn constant_value(&self) -> Option<u16> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::ConstantValue(value) = *attr {
                return Some(value);
            }
        }
        None
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum AttributeInfo {
    SourceFile(u16),
    InnerClasses(Vec<InnerClassInfo>),
    EnclosingMethod(EnclosingMethodAttribute),
    SourceDebugExtension(Vec<u8>),
    BootstrapMethods(Vec<BootstrapMethodInfo>),
    ConstantValue(u16),
    Code(Box<CodeAttribute>),
    Exceptions(Vec<u16>),
    LineNumberTable(Vec<LineNumberTableEntry>),
    LocalVariableTable(Vec<LocalVariableTableEntry>),
    LocalVariableTypeTable(Vec<LocalVariableTypeTableEntry>),
    StackMapTable(Vec<StackMapFrame>),
    Synthetic,
    Signature(u16),
    AnnotationDefault(ElementValue),
    MethodParameters(Vec<MethodParameterInfo>),
    RuntimeVisibleAnnotations(Vec<Annotation>),
    RuntimeInvisibleAnnotations(Vec<Annotation>),
    RuntimeVisibleTypeAnnotations(Vec<TypeAnnotation>),
    RuntimeInvisibleTypeAnnotations(Vec<TypeAnnotation>),
    RuntimeVisibleParameterAnnotations(Vec<Vec<Annotation>>),
    RuntimeInvisibleParameterAnnotations(Vec<Vec<Annotation>>),
    Deprecated,
    Raw(Box<Vec<u8>>),
}
