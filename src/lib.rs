extern crate byteorder;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;
extern crate env_logger;

pub mod classfile;
pub mod util;
pub mod javap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
