use elf::File;
use elf::ParseError;
use elf::types::Machine;
use std::error::Error;
use std::path::Path;
use std::fmt;


#[derive(Debug)]
pub enum LibcParseError{
    ELFReadError(ParseError),
    UnsupportedArchitecture(Machine),
    InvalidLibc,
}

impl fmt::Display for LibcParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl Error for LibcParseError {
}

#[derive(Debug)]
pub struct Libc {
    pub linux_platform: String,
    pub version: String,
    pub libc_kind: String,
    pub architecture: Architecture,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Architecture {
    i386,
    amd64,
    //TODO handle soft float also
    armhf,
    arm64
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

fn find_subsequence<T>(haystack: &[T], needle: &[T]) -> Option<usize>
    where for<'a> &'a [T]: PartialEq
{
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn get_version_info(libc: &File) -> Result<(String, String, String), LibcParseError>{
    let rodata = &libc.get_section(".rodata")
        .ok_or(LibcParseError::InvalidLibc)?
        .data.clone()[..];

    let needle = b"GNU C Library (";
    let idx = find_subsequence(rodata, needle)
        .ok_or(LibcParseError::InvalidLibc)?;
    let start_idx = idx + needle.len();

    let version_info_extra = &rodata[start_idx..];
    let end_idx = find_subsequence(version_info_extra, b")")
        .ok_or(LibcParseError::InvalidLibc)?;

    let version_info:Vec<_> = (&version_info_extra[..end_idx])
        .split(|&v| v as char == ' ')
        .map(|v| String::from_utf8(v.to_vec()).unwrap())
        .collect();
    
    match &version_info[..]{
        [linux_platform, libc_kind, version,..] =>
            Ok((linux_platform.to_string(), libc_kind.to_string().to_lowercase(), version.to_string())),
        _ => Err(LibcParseError::InvalidLibc)
    }
}

fn get_architecture(libc: &File) -> Result<Architecture, LibcParseError> {
    let headers = libc.ehdr;
    match headers.machine {
        Machine(3) => Ok(Architecture::i386),
        Machine(62) => Ok(Architecture::amd64),
        Machine(40) => Ok(Architecture::armhf),
        Machine(183) => Ok(Architecture::arm64),
        x => Err(LibcParseError::UnsupportedArchitecture(x))
    }
}

impl Libc{
    pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Libc, Box<Error>>{
        let libc = File::open_path(&path)
            .map_err(|e| LibcParseError::ELFReadError(e) )?;

        let (linux_platform, libc_kind, version) = get_version_info(&libc)?;
        let architecture = get_architecture(&libc)?;

        Ok(Libc {
            linux_platform: linux_platform,
            version: version,
            architecture: architecture,
            libc_kind: libc_kind
        })
    }
}
