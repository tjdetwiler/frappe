use std::io;

use util::*;
use classfile::error::{Error, Result};

#[derive(Debug)]
pub struct StackMapTableAttribute {
    pub entries: Vec<StackMapFrame>,
}

impl StackMapTableAttribute {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<StackMapTableAttribute> {
        let number_of_entries = try!(read_u16(rdr));
        let mut entries: Vec<StackMapFrame> = vec![];
        for _ in 0..number_of_entries {
            let stack_map_frame = try!(StackMapFrame::read(rdr));
            entries.push(stack_map_frame);
        }
        Ok(StackMapTableAttribute { entries: entries })
    }
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

impl VerificationTypeInfo {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<VerificationTypeInfo> {
        let tag = try!(read_u8(rdr));
        match tag {
            0x0 => Ok(VerificationTypeInfo::Top),
            0x1 => Ok(VerificationTypeInfo::Integer),
            0x2 => Ok(VerificationTypeInfo::Float),
            0x3 => Ok(VerificationTypeInfo::Double),
            0x4 => Ok(VerificationTypeInfo::Long),
            0x5 => Ok(VerificationTypeInfo::Null),
            0x6 => Ok(VerificationTypeInfo::UninitializedThis),
            0x7 => {
                let object_variable_info = try!(ObjectVariableInfo::read(rdr));
                Ok(VerificationTypeInfo::Object(object_variable_info))
            }
            0x8 => {
                let uninitialized_variable_info = try!(UninitializedVariableInfo::read(rdr));
                Ok(VerificationTypeInfo::Uninitialized(uninitialized_variable_info))
            }
            _ => Err(Error::InvalidVerificationTypeInfoTag(tag)),
        }
    }
}

#[derive(Debug)]
pub struct ObjectVariableInfo {
    pub cpool_index: u16,
}

impl ObjectVariableInfo {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<ObjectVariableInfo> {
        let cpool_index = try!(read_u16(rdr));
        Ok(ObjectVariableInfo { cpool_index: cpool_index })
    }
}

#[derive(Debug)]
pub struct UninitializedVariableInfo {
    pub offset: u16,
}

impl UninitializedVariableInfo {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<UninitializedVariableInfo> {
        let offset = try!(read_u16(rdr));
        Ok(UninitializedVariableInfo { offset: offset })
    }
}

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

impl SameLocals1StackItemFrame {
    pub fn read<T: io::Read>(rdr: &mut T, frame_type: u8) -> Result<SameLocals1StackItemFrame> {
        let verification_type_info = try!(VerificationTypeInfo::read(rdr));
        Ok(SameLocals1StackItemFrame {
            frame_type: frame_type,
            stack: [verification_type_info],
        })
    }
}

#[derive(Debug)]
pub struct SameLocals1StackItemFrameExtended {
    pub offset_delta: u16,
    pub stack: [VerificationTypeInfo; 1],
}

impl SameLocals1StackItemFrameExtended {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<SameLocals1StackItemFrameExtended> {
        let offset_delta = try!(read_u16(rdr));
        let verification_type_info = try!(VerificationTypeInfo::read(rdr));
        Ok(SameLocals1StackItemFrameExtended {
            offset_delta: offset_delta,
            stack: [verification_type_info],
        })
    }
}

#[derive(Debug)]
pub struct ChopFrame {
    pub frame_type: u8,
    pub offset_delta: u16,
}

impl ChopFrame {
    pub fn read<T: io::Read>(rdr: &mut T, frame_type: u8) -> Result<ChopFrame> {
        let offset_delta = try!(read_u16(rdr));
        Ok(ChopFrame {
            frame_type: frame_type,
            offset_delta: offset_delta,
        })
    }
}

#[derive(Debug)]
pub struct SameFrameExtended {
    pub offset_delta: u16,
}

impl SameFrameExtended {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<SameFrameExtended> {
        let offset_delta = try!(read_u16(rdr));
        Ok(SameFrameExtended { offset_delta: offset_delta })
    }
}

#[derive(Debug)]
pub struct AppendFrame {
    pub frame_type: u8,
    pub offset_delta: u16,
    pub locals: Vec<VerificationTypeInfo>,
}

impl AppendFrame {
    pub fn read<T: io::Read>(rdr: &mut T, frame_type: u8) -> Result<AppendFrame> {
        let offset_delta = try!(read_u16(rdr));
        let num_locals = frame_type - 251;
        let mut locals: Vec<VerificationTypeInfo> = vec![];
        for _ in 0..num_locals {
            let verification_type_info = try!(VerificationTypeInfo::read(rdr));
            locals.push(verification_type_info);
        }
        Ok(AppendFrame {
            frame_type: frame_type,
            offset_delta: offset_delta,
            locals: locals,
        })
    }
}

#[derive(Debug)]
pub struct FullFrame {
    pub offset_delta: u16,
    pub locals: Vec<VerificationTypeInfo>,
    pub stack: Vec<VerificationTypeInfo>,
}

impl FullFrame {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<FullFrame> {
        let offset_delta = try!(read_u16(rdr));
        let number_of_locals = try!(read_u16(rdr));
        let mut locals: Vec<VerificationTypeInfo> = vec![];
        for _ in 0..number_of_locals {
            let verification_type_info = try!(VerificationTypeInfo::read(rdr));
            locals.push(verification_type_info);
        }
        let number_of_stack_items = try!(read_u16(rdr));
        let mut stack: Vec<VerificationTypeInfo> = vec![];
        for _ in 0..number_of_stack_items {
            let verification_type_info = try!(VerificationTypeInfo::read(rdr));
            stack.push(verification_type_info);
        }
        Ok(FullFrame {
            offset_delta: offset_delta,
            locals: locals,
            stack: stack,
        })
    }
}

impl StackMapFrame {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<StackMapFrame> {
        let frame_type = try!(read_u8(rdr));
        match frame_type {
            0...63 => Ok(StackMapFrame::SameFrame(SameFrame { frame_type: frame_type })),
            64...127 => {
                let frame = try!(SameLocals1StackItemFrame::read(rdr, frame_type));
                Ok(StackMapFrame::SameLocals1StackItemFrame(frame))
            }
            247 => {
                let frame = try!(SameLocals1StackItemFrameExtended::read(rdr));
                Ok(StackMapFrame::SameLocals1StackItemFrameExtended(frame))
            }
            248...250 => {
                let frame = try!(ChopFrame::read(rdr, frame_type));
                Ok(StackMapFrame::ChopFrame(frame))
            }
            251 => {
                let frame = try!(SameFrameExtended::read(rdr));
                Ok(StackMapFrame::SameFrameExtended(frame))
            }
            252...254 => {
                let frame = try!(AppendFrame::read(rdr, frame_type));
                Ok(StackMapFrame::AppendFrame(frame))
            }
            255 => {
                let frame = try!(FullFrame::read(rdr));
                Ok(StackMapFrame::FullFrame(frame))
            }
            _ => Err(Error::InvalidStackFrameType(frame_type)),
        }
    }
}
