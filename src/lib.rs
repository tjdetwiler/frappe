extern crate byteorder;
#[macro_use]
extern crate bitflags;

pub mod classfile;
pub mod util;
pub mod instr;
pub mod disassemble;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
