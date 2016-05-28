//! Top-level types for working with Java class files.
pub mod method;
pub mod field;
pub mod cp;
pub mod attr;
pub mod error;
pub mod reader;

use std::fmt;
use std::vec::Vec;

use classfile::attr::{
    Annotation,
    Attributes, 
    AttributeInfo,
    BootstrapMethodInfo,
    EnclosingMethodAttribute,
    InnerClassInfo,
    TypeAnnotation,
};
use classfile::cp::{ConstantPool, Constant};
use classfile::field::FieldInfo;
use classfile::method::MethodInfo;

#[derive(Debug)]
pub struct ClassFile {
    /// Java classfile magic number. `0xcafebabe` is the only valid value for
    /// this field.
    pub magic: u32,
    /// The minor version of this classfile.
    pub minor_version: u16,
    /// The major version of this classfile.
    pub major_version: u16,
    /// Classfile constant pool. Contains constant values (integer, long, string, etc)
    /// as well as metadata about classes and types.
    pub constant_pool: ConstantPool,
    /// Access flags for this class.
    pub access_flags: ClassAccessFlags,
    /// Index into the constant pool that resolves to a `Constant::Class` value.
    pub this_class: u16,
    /// Optional index into the constant pool that resolves to a `Constant::Class`
    /// value. If `super_class` is zero then there is no super class for this type.
    pub super_class: u16,
    /// A list of indicies into the constant pool that identify any interfaces
    /// directly applied to this class. These indicies must resolve to
    /// `Constant::Class` entries.
    pub interfaces: Vec<u16>,
    /// A list of field descriptors that identify the fields of this class.
    pub fields: Vec<FieldInfo>,
    /// A list of field descriptors that identify the methods of this class.
    pub methods: Vec<MethodInfo>,
    /// A list of attributes applied to this class.
    pub attributes: Attributes,
}

impl ClassFile {
    /// Resolves the `this_class` member to the UTF8 string in the constant pool
    /// that holds the class name.
    pub fn this_class_name(&self) -> &String {
        let name_index = self.constant_pool[self.this_class].as_class();
        self.constant_pool[name_index].as_utf8()
    }

    /// Resolves the `super_class` member to the UTF8 string in the constant pool
    /// that holds the super class name. If `super_class == 0` then `None` is
    /// returned.
    pub fn super_class_name(&self) -> Option<&String> {
        if self.super_class == 0 {
            return None;
        }
        let name_index = self.constant_pool[self.super_class].as_class();
        Some(self.constant_pool[name_index].as_utf8())
    }

    /// Resolves the source file attribute in this class if it exists and returns
    /// the value. If there is no source file attribute then `None` is returned.
    pub fn source_file(&self) -> Option<&String> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::SourceFile(sourcefile_index) = *attr {
                let source_file = self.constant_pool[sourcefile_index].as_utf8();
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

    /// Resolves the signature attribute to a UTF8 string if present. Otherwise
    /// returns `None`.
    pub fn signature(&self) -> Option<&String> {
        for attr in self.attributes.iter() {
            if let AttributeInfo::Signature(signature_index) = *attr {
                let signature = self.constant_pool[signature_index].as_utf8();
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

    /// Determines if this class is synthetic (either via access flags or the
    /// synthetic attribute).
    pub fn is_synthetic(&self) -> bool {
        if self.access_flags.is_synthetic() {
            return true;
        }
        for attr in self.attributes.iter() {
            if let AttributeInfo::Synthetic = *attr {
                return true;
            }
        }
        false
    }

    /// Returns `true` if this class contains the Deprecated attribute.
    pub fn is_deprecated(&self) -> bool {
        for attr in self.attributes.iter() {
            if let AttributeInfo::Deprecated = *attr {
                return true;
            }
        }
        false
    }
}

bitflags! {
    pub flags ClassAccessFlags: u16 {
        const CLASS_ACC_PUBLIC        = 0x0001,
        const CLASS_ACC_FINAL         = 0x0010,
        const CLASS_ACC_SUPER         = 0x0020,
        const CLASS_ACC_INTERFACE     = 0x0200,
        const CLASS_ACC_ABSTRACT      = 0x0400,
        const CLASS_ACC_SYNTHETIC     = 0x1000,
        const CLASS_ACC_ANNOTATION    = 0x2000,
        const CLASS_ACC_ENUM          = 0x4000

    }
}

impl ClassAccessFlags {
    pub fn is_public(&self) -> bool {
        self.contains(CLASS_ACC_PUBLIC)
    }

    pub fn is_final(&self) -> bool {
        self.contains(CLASS_ACC_FINAL)
    }

    pub fn is_super(&self) -> bool {
        self.contains(CLASS_ACC_SUPER)
    }

    pub fn is_interface(&self) -> bool {
        self.contains(CLASS_ACC_INTERFACE)
    }

    pub fn is_abstract(&self) -> bool {
        self.contains(CLASS_ACC_ABSTRACT)
    }

    pub fn is_synthetic(&self) -> bool {
        self.contains(CLASS_ACC_SYNTHETIC)
    }

    pub fn is_annotation(&self) -> bool {
        self.contains(CLASS_ACC_ANNOTATION)
    }

    pub fn is_enum(&self) -> bool {
        self.contains(CLASS_ACC_ENUM)
    }
}

impl fmt::Display for ClassAccessFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::new();
        if self.is_public() {
            v.push("ACC_PUBLIC");
        }
        if self.is_final() {
            v.push("ACC_FINAL");
        }
        if self.is_super() {
            v.push("ACC_SUPER");
        }
        if self.is_interface() {
            v.push("ACC_INTERFACE");
        }
        if self.is_abstract() {
            v.push("ACC_ABSTRACT");
        }
        if self.is_synthetic() {
            v.push("ACC_SYNTHETIC");
        }
        if self.is_annotation() {
            v.push("ACC_ANNOTATION");
        }
        if self.is_enum() {
            v.push("ACC_ENUM");
        }

        write!(f, "{}", v.join(", "))
    }
}
