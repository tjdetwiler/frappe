use std::io;

use util::*;
use classfile::error::{Error, Result};

#[derive(Debug)]
pub enum TargetType {
    TypeParameter(TypeParameterTarget),
    Supertype(SupertypeTarget),
    TypeParameterBound(TypeParameterBoundTarget),
    Empty,
    MethodFormalParameter(FormalParameterTarget),
    Throws(ThrowsTarget),
    Localvar(LocalvarTarget),
    Catch(CatchTarget),
    Offset(OffsetTarget),
    TypeArgument(TypeArgumentTarget),
}

impl TargetType {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<TargetType> {
        let target_type = try!(read_u8(rdr));
        match target_type {
            0x00...0x01 => {
                let type_parameter_target = try!(TypeParameterTarget::read(rdr));
                Ok(TargetType::TypeParameter(type_parameter_target))
            }
            0x10 => {
                let supertype_target = try!(SupertypeTarget::read(rdr));
                Ok(TargetType::Supertype(supertype_target))
            }
            0x11...0x12 => {
                let type_parameter_bound_target = try!(TypeParameterBoundTarget::read(rdr));
                Ok(TargetType::TypeParameterBound(type_parameter_bound_target))
            }
            0x13...0x15 => Ok(TargetType::Empty),
            0x16 => {
                let method_formal_parameter_target = try!(FormalParameterTarget::read(rdr));
                Ok(TargetType::MethodFormalParameter(method_formal_parameter_target))
            }
            0x17 => {
                let throws_target = try!(ThrowsTarget::read(rdr));
                Ok(TargetType::Throws(throws_target))
            }
            0x40...0x41 => {
                let localvar_target = try!(LocalvarTarget::read(rdr));
                Ok(TargetType::Localvar(localvar_target))
            }
            0x42 => {
                let catch_target = try!(CatchTarget::read(rdr));
                Ok(TargetType::Catch(catch_target))
            }
            0x43...0x46 => {
                let offset_target = try!(OffsetTarget::read(rdr));
                Ok(TargetType::Offset(offset_target))
            }
            0x47...0x4b => {
                let type_argument_target = try!(TypeArgumentTarget::read(rdr));
                Ok(TargetType::TypeArgument(type_argument_target))
            }
            target_type => Err(Error::InvalidTargetTypeTag(target_type)),
        }
    }
}

#[derive(Debug)]
pub struct TypeParameterTarget {
    type_parameter_index: u8,
}

impl TypeParameterTarget {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<TypeParameterTarget> {
        let type_parameter_index = try!(read_u8(rdr));
        Ok(TypeParameterTarget { type_parameter_index: type_parameter_index })
    }
}

#[derive(Debug)]
pub struct SupertypeTarget {
    supertype_index: u8,
}

impl SupertypeTarget {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<SupertypeTarget> {
        let supertype_index = try!(read_u8(rdr));
        Ok(SupertypeTarget { supertype_index: supertype_index })
    }
}

#[derive(Debug)]
pub struct TypeParameterBoundTarget {
    type_parameter_index: u8,
    bound_index: u8,
}

impl TypeParameterBoundTarget {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<TypeParameterBoundTarget> {
        let type_parameter_index = try!(read_u8(rdr));
        let bound_index = try!(read_u8(rdr));
        Ok(TypeParameterBoundTarget {
            type_parameter_index: type_parameter_index,
            bound_index: bound_index,
        })
    }
}

#[derive(Debug)]
pub struct FormalParameterTarget {
    formal_parameter_index: u8,
}

impl FormalParameterTarget {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<FormalParameterTarget> {
        let formal_parameter_index = try!(read_u8(rdr));
        Ok(FormalParameterTarget { formal_parameter_index: formal_parameter_index })
    }
}

#[derive(Debug)]
pub struct ThrowsTarget {
    throws_type_index: u8,
}

impl ThrowsTarget {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<ThrowsTarget> {
        let throws_type_index = try!(read_u8(rdr));
        Ok(ThrowsTarget { throws_type_index: throws_type_index })
    }
}

#[derive(Debug)]
pub struct LocalvarInfo {
    start_pc: u16,
    length: u16,
    index: u16,
}

impl LocalvarInfo {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<LocalvarInfo> {
        let start_pc = try!(read_u16(rdr));
        let length = try!(read_u16(rdr));
        let index = try!(read_u16(rdr));
        Ok(LocalvarInfo {
            start_pc: start_pc,
            length: length,
            index: index,
        })
    }
}

#[derive(Debug)]
pub struct LocalvarTarget {
    table: Vec<LocalvarInfo>,
}

impl LocalvarTarget {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<LocalvarTarget> {
        let table_length = try!(read_u16(rdr));
        let mut table: Vec<LocalvarInfo> = vec![];
        for _ in 0..table_length {
            let localvar_info = try!(LocalvarInfo::read(rdr));
            table.push(localvar_info);
        }
        Ok(LocalvarTarget { table: table })
    }
}

#[derive(Debug)]
pub struct CatchTarget {
    exception_table_index: u16,
}

impl CatchTarget {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<CatchTarget> {
        let exception_table_index = try!(read_u16(rdr));
        Ok(CatchTarget { exception_table_index: exception_table_index })
    }
}

#[derive(Debug)]
pub struct OffsetTarget {
    offset: u16,
}

impl OffsetTarget {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<OffsetTarget> {
        let offset = try!(read_u16(rdr));
        Ok(OffsetTarget { offset: offset })
    }
}

#[derive(Debug)]
pub struct TypeArgumentTarget {
    offset: u16,
    type_argument_index: u8,
}

impl TypeArgumentTarget {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<TypeArgumentTarget> {
        let offset = try!(read_u16(rdr));
        let type_argument_index = try!(read_u8(rdr));
        Ok(TypeArgumentTarget {
            offset: offset,
            type_argument_index: type_argument_index,
        })
    }
}
