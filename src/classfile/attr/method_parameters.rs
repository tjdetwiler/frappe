use std::io;

use util::*;
use classfile::error::Result;

#[derive(Debug)]
pub struct MethodParametersAttribute {
    pub parameters: Vec<MethodParameterInfo>,
}

impl MethodParametersAttribute {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<MethodParametersAttribute> {
        let parameters_count = try!(read_u8(rdr));
        let mut parameters: Vec<MethodParameterInfo> = vec![];
        for _ in 0..parameters_count {
            let parameter_info = try!(MethodParameterInfo::read(rdr));
            parameters.push(parameter_info);
        }
        Ok(MethodParametersAttribute { parameters: parameters })
    }
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
    name_index: u16,
    access_flags: MethodParameterAccessFlags,
}

impl MethodParameterInfo {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<MethodParameterInfo> {
        let name_index = try!(read_u16(rdr));
        let access_flags = try!(read_u16(rdr));
        Ok(MethodParameterInfo {
            name_index: name_index,
            access_flags: MethodParameterAccessFlags::from_bits_truncate(access_flags),
        })
    }
}

