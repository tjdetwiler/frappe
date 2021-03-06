use std::ops::{Deref, Index};

#[derive(Debug)]
pub struct ConstantPool {
    constants: Vec<Constant>,
}

impl ConstantPool {
    pub fn new(constants: Vec<Constant>) -> ConstantPool {
        ConstantPool { constants: constants }
    }

    pub fn len(&self) -> u16 {
        self.constants.len() as u16 + 1
    }
}

impl Index<u16> for ConstantPool {
    type Output = Constant;

    fn index<'a>(&'a self, index: u16) -> &'a Constant {
        &self.constants[index as usize - 1]
    }
}

impl Deref for ConstantPool {
    type Target = Vec<Constant>;

    fn deref(&self) -> &Vec<Constant> {
        &self.constants
    }
}

/// Represents an entity in the constant pool has an associated class, as well
/// as a name and type.
///
/// This applies to `Fieldref`, `Methodref`, and `InterfaceMethodref` constants.
#[derive(Debug, Eq, PartialEq)]
pub struct TypedEntityConstant {
    /// An index into the constant pool that is of type `Constant::Class`.
    pub class_index: u16,
    /// An index into the constant pool that is of type `Constant::NameAndType`.
    pub name_and_type_index: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct NameAndTypeConstant {
    /// An index into the constant pool that is of type `Constant::Utf8`.
    pub name_index: u16,
    /// An index into the constant pool that is of type `Constant::Utf8`.
    ///
    /// This string is either a method or field descriptor (depending on context)
    /// that identifies the specific type of the entity.
    pub descriptor_index: u16,
}

/// Represents a single entry in the constant pool.
#[derive(Debug, PartialEq)]
pub enum Constant {
    Class(u16),
    Fieldref(TypedEntityConstant),
    Methodref(TypedEntityConstant),
    InterfaceMethodref(TypedEntityConstant),
    String(u16),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    NameAndType(NameAndTypeConstant),
    Utf8(String),
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    MethodType {
        descriptor_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    /// A pseudo-constant that is inserted in the empty indicies following the 8
    /// byte constant values (Double/Long).
    Skip,
}

impl Constant {
    /// Returns a human-readable name of this constant variant.
    pub fn name(&self) -> &str {
        match *self {
            Constant::Class(_) => "Class",
            Constant::Fieldref(_) => "Fieldref",
            Constant::Methodref(_) => "Methodref",
            Constant::InterfaceMethodref(_) => "InterfaceMethodref",
            Constant::String(_) => "String",
            Constant::Integer(_) => "Integer",
            Constant::Float(_) => "Float",
            Constant::Long(_) => "Long",
            Constant::Double(_) => "Double",
            Constant::NameAndType(_) => "NameAndType",
            Constant::Utf8(_) => "Utf8",
            Constant::MethodHandle { .. } => "MethodHandle",
            Constant::MethodType { .. } => "MethodType",
            Constant::InvokeDynamic { .. } => "InvokeDynamic",
            Constant::Skip => "Skip",
        }
    }

    /// Asserts that this constant is a `Constant::Class` and returns the class name index.
    pub fn as_class(&self) -> u16 {
        match *self {
            Constant::Class(class) => class,
            _ => {
                panic!("Constant is of incorrect type! Expected Class but was {}",
                       self.name())
            }
        }
    }

    /// Asserts that this constant is a `Constant::Fieldref` and returns the associated
    /// data.
    pub fn as_fieldref(&self) -> &TypedEntityConstant {
        match *self {
            Constant::Fieldref(ref entity) => entity,
            _ => {
                panic!("Constant is of incorrect type! Expected Fieldref but was {}",
                       self.name())
            }
        }
    }

    /// Asserts that this constant is a `Constant::Methodref` and returns the associated
    /// data.
    pub fn as_methodref(&self) -> &TypedEntityConstant {
        match *self {
            Constant::Methodref(ref entity) => entity,
            _ => {
                panic!("Constant is of incorrect type! Expected Methodref but was {}",
                       self.name())
            }
        }
    }

    /// Asserts that this constant is a `Constant::InterfaceMethodref` and returns the
    /// associated data.
    pub fn as_interface_methodref(&self) -> &TypedEntityConstant {
        match *self {
            Constant::InterfaceMethodref(ref entity) => entity,
            _ => {
                panic!("Constant is of incorrect type! Expected InterfaceMethodref but was {}",
                       self.name())
            }
        }
    }

    /// Asserts that this constant is a `Constant::String` and returns the index of the
    /// string value (a `Constant::Utf8`).
    pub fn as_string(&self) -> u16 {
        match *self {
            Constant::String(name_index) => name_index,
            _ => {
                panic!("Constant is of incorrect type! Expected String but was {}",
                       self.name())
            }
        }
    }

    /// Asserts that this constant is a `Constant::Utf8` and returns the associated string.
    pub fn as_utf8(&self) -> &String {
        match *self {
            Constant::Utf8(ref value) => value,
            _ => {
                panic!("Constant is of incorrect type! Expected Utf8 but was {}",
                       self.name())
            }
        }
    }

    /// Asserts that this constant is a `Constant::Integer` and returns the associated value.
    pub fn as_integer(&self) -> i32 {
        match *self {
            Constant::Integer(value) => value,
            _ => {
                panic!("Constant is of incorrect type! Expected Integer but was {}",
                       self.name())
            }
        }
    }

    /// Asserts that this constant is a `Constant::Long` and returns the associated value.
    pub fn as_long(&self) -> i64 {
        match *self {
            Constant::Long(value) => value,
            _ => {
                panic!("Constant is of incorrect type! Expected Long but was {}",
                       self.name())
            }
        }
    }

    /// Asserts that this constant is a `Constant::Float` and returns the associated value.
    pub fn as_float(&self) -> f32 {
        match *self {
            Constant::Float(value) => value,
            _ => {
                panic!("Constant is of incorrect type! Expected Float but was {}",
                       self.name())
            }
        }
    }

    /// Asserts that this constant is a `Constant::Double` and returns the associated value.
    pub fn as_double(&self) -> f64 {
        match *self {
            Constant::Double(value) => value,
            _ => {
                panic!("Constant is of incorrect type! Expected Float but was {}",
                       self.name())
            }
        }
    }

    /// Asserts that this constant is a `Constant::NameAndType` and returns the associated data.
    pub fn as_name_and_type(&self) -> &NameAndTypeConstant {
        match *self {
            Constant::NameAndType(ref name_and_type) => name_and_type,
            _ => {
                panic!("Constant is of incorrect type! Expected NameAndType but was {}",
                       self.name())
            }
        }
    }
}
