use std::vec::Vec;
use std::ops::Deref;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct LocalvarInfo {
    pub start_pc: u16,
    pub length: u16,
    pub index: u16,
}

#[derive(Debug)]
pub enum ElementValue {
    ConstantValue(ConstantValue),
    EnumConstValue(EnumConstValue),
    ClassInfo(u16),
    AnnotationValue(Annotation),
    ArrayValue(ArrayValue),
}

#[derive(Debug)]
pub struct ElementValuePair {
    pub element_name_index: u16,
    pub value: ElementValue,
}

#[derive(Debug)]
pub struct ConstantValue {
    pub tag: u8,
    pub const_value_index: u16,
}

#[derive(Debug)]
pub struct ArrayValue {
    pub values: Vec<ElementValue>,
}

#[derive(Debug)]
pub struct EnumConstValue {
    pub type_name_index: u16,
    pub const_name_index: u16,
}

#[derive(Debug)]
pub struct Annotation {
    pub type_index: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Debug)]
pub struct TypePathEntry {
    pub type_path_kind: u8,
    pub type_argument_index: u8,
}

#[derive(Debug)]
pub struct TypeAnnotation {
    pub target_info: TargetInfo,
    pub target_path: Vec<TypePathEntry>,
    pub type_index: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ObjectVariableInfo {
    pub cpool_index: u16,
}

#[derive(Debug)]
pub struct UninitializedVariableInfo {
    pub offset: u16,
}

/// Defines an entry in a `StackMapTableAttribute`.
#[derive(Debug)]
pub enum StackMapFrame {
    SameFrame(SameFrame),
    SameLocals1StackItemFrame(SameLocals1StackItemFrame),
    SameLocals1StackItemFrameExtended(SameLocals1StackItemFrameExtended),
    ChopFrame(ChopFrame),
    SameFrameExtended(SameFrameExtended),
    AppendFrame(AppendFrame),
    FullFrame(FullFrame),
}

#[derive(Debug)]
pub struct SameFrame {
    pub frame_type: u8,
}

#[derive(Debug)]
pub struct SameLocals1StackItemFrame {
    pub frame_type: u8,
    pub stack: [VerificationTypeInfo; 1],
}

#[derive(Debug)]
pub struct SameLocals1StackItemFrameExtended {
    pub offset_delta: u16,
    pub stack: [VerificationTypeInfo; 1],
}

#[derive(Debug)]
pub struct ChopFrame {
    pub frame_type: u8,
    pub offset_delta: u16,
}

#[derive(Debug)]
pub struct SameFrameExtended {
    pub offset_delta: u16,
}

#[derive(Debug)]
pub struct AppendFrame {
    pub frame_type: u8,
    pub offset_delta: u16,
    pub locals: Vec<VerificationTypeInfo>,
}

#[derive(Debug)]
pub struct FullFrame {
    pub offset_delta: u16,
    pub locals: Vec<VerificationTypeInfo>,
    pub stack: Vec<VerificationTypeInfo>,
}

bitflags! {
    pub flags MethodParameterAccessFlags: u16 {
        const MP_ACC_FINAL         = 0x0010,
        const MP_ACC_SYNTHETIC     = 0x1000,
        const MP_ACC_MANDATED      = 0x8000
    }
}

#[derive(Debug)]
pub struct MethodParameterInfo {
    pub name_index: u16,
    pub access_flags: MethodParameterAccessFlags,
}


#[derive(Debug)]
pub struct LocalVariableTypeTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

#[derive(Debug)]
pub struct LocalVariableTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct InnerClassInfo {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: InnerClassAccessFlags,
}

#[derive(Debug)]
pub struct EnclosingMethodAttribute {
    pub class_index: u16,
    pub method_index: u16,
}

#[derive(Debug)]
pub struct ExceptionInfo {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Debug)]
pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionInfo>,
    pub attributes: Attributes,
}

#[derive(Debug)]
pub struct BootstrapMethodInfo {
    pub bootstrap_method_ref: u16,
    pub bootstrap_arguments: Vec<u16>,
}

#[derive(Debug)]
pub struct Attributes {
    pub attributes: Vec<AttributeInfo>,
}

impl Deref for Attributes {
    type Target = Vec<AttributeInfo>;

    fn deref(&self) -> &Vec<AttributeInfo> {
        &self.attributes
    }
}

#[derive(Debug)]
pub enum AttributeInfo {
    SourceFile(u16),
    InnerClasses(Vec<InnerClassInfo>),
    EnclosingMethod(EnclosingMethodAttribute),
    SourceDebugExtension(Vec<u8>),
    BootstrapMethods(Vec<BootstrapMethodInfo>),
    ConstantValue {
        constantvalue_index: u16,
    },
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
