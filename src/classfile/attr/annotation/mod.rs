use std::io;

use util::*;
use classfile::error::Result;

mod target_info;
pub use self::target_info::*;
mod element_value;
pub use self::element_value::*;

#[derive(Debug)]
pub struct Annotation {
    type_index: u16,
    element_value_pairs: Vec<ElementValuePair>,
}

impl Annotation {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<Annotation> {
        let type_index = try!(read_u16(rdr));
        let num_element_value_pairs = try!(read_u16(rdr));
        let mut element_value_pairs: Vec<ElementValuePair> = vec![];
        for _ in 0..num_element_value_pairs {
            let element_value_pair = try!(ElementValuePair::read(rdr));
            element_value_pairs.push(element_value_pair);
        }
        Ok(Annotation {
            type_index: type_index,
            element_value_pairs: element_value_pairs,
        })
    }
}

#[derive(Debug)]
pub struct TypePathEntry {
    pub type_path_kind: u8,
    pub type_argument_index: u8,
}

impl TypePathEntry {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<TypePathEntry> {
        let type_path_kind = try!(read_u8(rdr));
        let type_argument_index = try!(read_u8(rdr));
        Ok(TypePathEntry {
            type_path_kind: type_path_kind,
            type_argument_index: type_argument_index,
        })
    }
}

#[derive(Debug)]
pub struct TypePath {
    path: Vec<TypePathEntry>,
}

impl TypePath {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<TypePath> {
        let path_length = try!(read_u8(rdr));
        let mut path: Vec<TypePathEntry> = vec![];
        for _ in 0..path_length {
            let path_entry = try!(TypePathEntry::read(rdr));
            path.push(path_entry);
        }
        Ok(TypePath { path: path })
    }
}

#[derive(Debug)]
pub struct TypeAnnotation {
    target_info: TargetInfo,
    target_path: TypePath,
    type_index: u16,
    element_value_pairs: Vec<ElementValuePair>,
}

impl TypeAnnotation {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<TypeAnnotation> {
        let target_info = try!(TargetInfo::read(rdr));
        let target_path = try!(TypePath::read(rdr));
        let type_index = try!(read_u16(rdr));
        let num_element_value_pairs = try!(read_u16(rdr));
        let mut element_value_pairs: Vec<ElementValuePair> = vec![];
        for _ in 0..num_element_value_pairs {
            let element_value_pair = try!(ElementValuePair::read(rdr));
            element_value_pairs.push(element_value_pair);
        }
        Ok(TypeAnnotation {
            target_info: target_info,
            target_path: target_path,
            type_index: type_index,
            element_value_pairs: element_value_pairs,
        })
    }
}

#[derive(Debug)]
pub struct Annotations {
    pub annotations: Vec<Annotation>,
}

impl Annotations {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<Annotations> {
        let num_annotations = try!(read_u16(rdr));
        let mut annotations: Vec<Annotation> = vec![];
        for _ in 0..num_annotations {
            let annotation = try!(Annotation::read(rdr));
            annotations.push(annotation);
        }
        Ok(Annotations { annotations: annotations })
    }
}

#[derive(Debug)]
pub struct ParameterAnnotations {
    pub parameter_annotations: Vec<Annotations>,
}

impl ParameterAnnotations {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<ParameterAnnotations> {
        let num_parameters = try!(read_u8(rdr));
        let mut parameter_annotations: Vec<Annotations> = vec![];
        for _ in 0..num_parameters {
            let annotations = try!(Annotations::read(rdr));
            parameter_annotations.push(annotations);
        }
        Ok(ParameterAnnotations { parameter_annotations: parameter_annotations })
    }
}

#[derive(Debug)]
pub struct TypeAnnotations {
    pub annotations: Vec<TypeAnnotation>,
}

impl TypeAnnotations {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<TypeAnnotations> {
        let num_annotations = try!(read_u8(rdr));
        let mut annotations: Vec<TypeAnnotation> = vec![];
        for _ in 0..num_annotations {
            let annotation = try!(TypeAnnotation::read(rdr));
            annotations.push(annotation);
        }
        Ok(TypeAnnotations { annotations: annotations })
    }
}
