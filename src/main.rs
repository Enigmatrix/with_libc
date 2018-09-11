#![feature(nll)]
#![feature(slice_patterns)]
#![feature(box_syntax, box_patterns)]
#![feature(extern_prelude)]

extern crate ar;
extern crate clap;
extern crate elf;
extern crate reqwest;
extern crate select;
extern crate xz2;
extern crate flate2;
extern crate tar;
extern crate ansi_term;

mod log;
mod ld;
mod libc;

use std::error::Error;
use std::path::PathBuf;
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
        .arg(Arg::with_name("dir")
            .short("d")
            .long("dir")
            .takes_value(true)
            .help("Sets the output directory of the ld.so, relative to the path of the libc. Default is the same path"))
       .get_matches();

    let libc_path = matches.value_of("libc_path").unwrap();
    let dir = matches.value_of("dir").unwrap_or(".");
    let prog_path = matches.value_of("PROGRAM").unwrap();

    let download_dir = ld_download_dir(libc_path.to_string(), dir.to_string())?;
        
    let libc = Libc::from_path(libc_path)?;
    let download_path = download_ld(&libc, download_dir)?;

    set_interpreter(download_path, PathBuf::from(prog_path.to_string()))?;

    Ok(())
}
