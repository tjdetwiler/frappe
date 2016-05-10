use std::io;
use byteorder::{BigEndian, ReadBytesExt};

pub fn read_u8<T: io::Read>(rdr: &mut T) -> io::Result<u8> {
    rdr.read_u8()
}

pub fn read_u16<T: io::Read>(rdr: &mut T) -> io::Result<u16> {
    rdr.read_u16::<BigEndian>()
}

pub fn read_u32<T: io::Read>(rdr: &mut T) -> io::Result<u32> {
    rdr.read_u32::<BigEndian>()
}
