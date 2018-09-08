use libc::Libc;
use std::error::Error;

pub fn download_ld(libc: &Libc) -> Result<String, Box<Error>>{
    Ok("".to_string())
}
