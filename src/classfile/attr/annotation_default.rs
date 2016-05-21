use std::io;

use classfile::error::Result;
use classfile::attr::annotation::ElementValue;

#[derive(Debug)]
pub struct AnnotationDefaultAttribute {
    pub default_value: ElementValue,
}

impl AnnotationDefaultAttribute {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<AnnotationDefaultAttribute> {
        let default_value = try!(ElementValue::read(rdr));
        Ok(AnnotationDefaultAttribute { default_value: default_value })
    }
}
