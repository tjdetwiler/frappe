use std::io;

use util::*;
use classfile::error::{Error, Result};

use super::Annotation;

#[derive(Debug)]
pub enum ElementValue {
    ConstantValue(ConstantValue),
    EnumConstValue(EnumConstValue),
    ClassInfo(u16),
    AnnotationValue(Box<Annotation>),
    ArrayValue(Box<ArrayValue>)
}

impl ElementValue {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<ElementValue> {
        let tag = try!(read_u8(rdr));
        match tag as char {
            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' => {
                let const_value = try!(ConstantValue::read(rdr, tag));
                Ok(ElementValue::ConstantValue(const_value))
            }
            'e' => {
                let enum_value = try!(EnumConstValue::read(rdr));
                Ok(ElementValue::EnumConstValue(enum_value))
            }
            'c' => {
                let class_info = try!(read_u16(rdr));
                Ok(ElementValue::ClassInfo(class_info))
            }
            '@' => {
                let annotation_value = try!(Annotation::read(rdr));
                Ok(ElementValue::AnnotationValue(Box::new(annotation_value)))
            }
            '[' => {
                let array_value = try!(ArrayValue::read(rdr));
                Ok(ElementValue::ArrayValue(Box::new(array_value)))
            }
            _ => Err(Error::InvalidElementValueTag(tag))
        }
    }
}

#[derive(Debug)]
pub struct ElementValuePair {
    element_name_index: u16,
    value: ElementValue,
}

impl ElementValuePair {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<ElementValuePair> {
        let element_name_index = try!(read_u16(rdr));
        let value = try!(ElementValue::read(rdr));
        Ok(ElementValuePair {
            element_name_index: element_name_index,
            value: value,
        })
    }
}

#[derive(Debug)]
pub struct ConstantValue {
    pub tag: u8,
    pub const_value_index: u16,

}

impl ConstantValue {
    pub fn read<T: io::Read>(rdr: &mut T, tag: u8) -> Result<ConstantValue> {
        let const_value_index = try!(read_u16(rdr));
        Ok(ConstantValue {
            tag: tag,
            const_value_index: const_value_index,
        })
    }
}

#[derive(Debug)]
pub struct ArrayValue {
    pub values: Vec<ElementValue>,
}

impl ArrayValue {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<ArrayValue> {
        let num_values = try!(read_u16(rdr));
        let mut values: Vec<ElementValue> = vec![];
        for _ in 0..num_values {
            let element_value = try!(ElementValue::read(rdr));
            values.push(element_value);
        }
        Ok(ArrayValue {
            values: values,
        })
    }
}

#[derive(Debug)]
pub struct EnumConstValue {
    pub type_name_index: u16,
    pub const_name_index: u16,
}

impl EnumConstValue {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<EnumConstValue> {
        let type_name_index = try!(read_u16(rdr));
        let const_name_index = try!(read_u16(rdr));
        Ok(EnumConstValue {
            type_name_index: type_name_index,
            const_name_index: const_name_index,
        })
    }
}
