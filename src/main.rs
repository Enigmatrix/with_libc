#![feature(slice_patterns)]

extern crate clap;
extern crate elf;

mod libc;

use clap::{App, Arg};
use libc::*;


fn main() {
    let matches = App::new("with_libc")
       .version("1.0")
       .about("Change a program's libc to a new one, while automatically setting up the ld.so. libc and ld will be in a new subdirectory.")
       .author("Enigmatrix")
       .arg(Arg::with_name("libc_path")
            .long("libc")
            .short("l")
            .value_name("LIBC")
            .help("Sets the new libc")
            .required(true)
            .takes_value(true))
       .arg(Arg::with_name("PROGRAM")
            .help("Sets the program")
            .required(true))
       .get_matches();

    let libc_path = matches.value_of("libc_path").unwrap();
    let prog_path = matches.value_of("PROGRAM").unwrap();
        
    let libc = Libc::from_path(libc_path);
    println!("{:?}", libc);
}
