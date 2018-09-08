#![feature(slice_patterns)]

extern crate ar;
extern crate clap;
extern crate elf;
extern crate reqwest;
extern crate select;
extern crate xz2;
extern crate tar;


mod libc;
mod ld;

use std::error::Error;
use clap::{App, Arg};
use libc::*;
use ld::*;

fn main() -> Result<(), Box<Error>> {
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
        
    let libc = Libc::from_path(libc_path)?;
    println!("{:?}", download_ld(&libc));
    Ok(())
}
