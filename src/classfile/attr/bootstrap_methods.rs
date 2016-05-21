use std::io;

use util::*;
use classfile::error::Result;

#[derive(Debug)]
pub struct BootstrapMethodInfo {
    bootstrap_method_ref: u16,
    bootstrap_arguments: Vec<u16>,
}

impl BootstrapMethodInfo {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<BootstrapMethodInfo> {
        let bootstrap_method_ref = try!(read_u16(rdr));
        let num_bootstrap_arguments = try!(read_u16(rdr));
        let mut bootstrap_arguments: Vec<u16> = vec![];
        for _ in 0..num_bootstrap_arguments {
            let bootstrap_argument = try!(read_u16(rdr));
            bootstrap_arguments.push(bootstrap_argument);
        }
        Ok(BootstrapMethodInfo {
            bootstrap_method_ref: bootstrap_method_ref,
            bootstrap_arguments: bootstrap_arguments,
        })
    }
}

#[derive(Debug)]
pub struct BootstrapMethodsAttribute {
    pub bootstrap_methods: Vec<BootstrapMethodInfo>,
}

impl BootstrapMethodsAttribute {
    pub fn read<T: io::Read>(rdr: &mut T) -> Result<BootstrapMethodsAttribute> {
        let num_bootstrap_methods = try!(read_u16(rdr));
        let mut bootstrap_methods: Vec<BootstrapMethodInfo> = vec![];
        for _ in 0..num_bootstrap_methods {
            let bootstrap_method = try!(BootstrapMethodInfo::read(rdr));
            bootstrap_methods.push(bootstrap_method);
        }
        Ok(BootstrapMethodsAttribute { bootstrap_methods: bootstrap_methods })
    }
}
