#[macro_use]
extern crate log;
extern crate env_logger;
extern crate javap;
extern crate zip;
extern crate classfile;

use std::io;
use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::fmt::Arguments;

use zip::ZipArchive;

use classfile::reader::ClassReader;
use javap::{Disassemble, Formatter, Options};

#[test]
#[ignore]
fn parse_all_files_in_rt_jar() {
    env_logger::init().unwrap();
    let java_home = match env::var("JAVA_HOME") {
        Ok(java_home) => java_home,
        _ => panic!("Test requires JAVA_HOME to be set.")
    };
    let jar_path = format!("{}/jre/lib/rt.jar", java_home);
    let jar_file = match File::open(jar_path) {
        Ok(file) => file,
        Err(e) => panic!("Failed to open rt.jar: {:?}", e),
    };
    let mut zip = match ZipArchive::new(jar_file) {
        Ok(zip) => zip,
        Err(e) => panic!("Failed to unpack rt.jar: {:?}", e),
    };
    for i in 0..zip.len() {
        let file = zip.by_index(i).unwrap();
        if !file.name().ends_with(".class") {
            continue;
        }
        println!("Reading: {}...", file.name());
        let classfile = ClassReader::new(file).read_class().unwrap();
        let mut fmt = Formatter::with_output(NullWriter);
        let opts = Options {
            verbose: true,
            constants: &classfile.constants,
        };
        classfile.pretty_print(&mut fmt, &opts).unwrap();
    }

    panic!("Fail!");
}

struct NullWriter;

impl Write for NullWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write_all(&mut self, _: &[u8]) -> io::Result<()> {
        Ok(())
    }

    fn write_fmt(&mut self, _: Arguments) -> io::Result<()> {
        Ok(())
    }

    fn by_ref(&mut self) -> &mut Self where Self: Sized {
        self
    }
}
