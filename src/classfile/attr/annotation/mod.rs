use std::io;

use util::*;
use classfile::error::{Error, Result};

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
