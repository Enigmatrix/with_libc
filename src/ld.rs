use libc::Libc;
use std::error::Error;
use std::str;
use std::path::Path;
use std::path::PathBuf;
use std::ffi::OsString;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Descendant};
use std::io::Read;
use xz2::read::XzDecoder;
use flate2::read::GzDecoder;
use std::fmt;
use tar;
use ar;
use reqwest;

#[derive(Debug)]
pub enum LdError {

}

impl fmt::Display for LdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl Error for LdError {
}


fn ld_fname(version: String) -> OsString {
    let short_version = version.split("-").collect::<Vec<_>>()[0];
    let fname_string = format!("ld-{}.so", short_version);
    From::from(fname_string)
}

fn extract_ld(url: String, download_dir:PathBuf, version: String) -> Result<(), Box<Error>> {
    let ld_fname_s = ld_fname(version);
    let ld_file_name = ld_fname_s.as_os_str();

    let deb_read = reqwest::get(&*url)?;
    let mut deb = ar::Archive::new(deb_read);

    while let Some(Ok(entry)) = deb.next_entry() {
        let header = str::from_utf8(entry.header().identifier())?.to_string();
        if !header.starts_with("data.tar") { continue }

        let extension = header.split('.').collect::<Vec<_>>()[2];
        let decoder: Box<Read> = match extension{
            "xz" => Ok(box XzDecoder::new(entry) as Box<Read>),
            "gz" => Ok(box GzDecoder::new(entry) as Box<Read>),
            _ => Err("Invalid")
        }?;
        let mut archive = tar::Archive::new(decoder);

        let ld_opt = archive.entries()?
            .filter_map(|e| e.ok())
            .find(|e| Some(ld_file_name) ==
                  e.path().unwrap().file_name());

        if let Some(mut ld) = ld_opt {
            let download_path = download_dir.join(ld_file_name);
            ld.unpack(download_path)?;
            return Ok(());
        }
        else{
            return Err(From::from("um".to_string()));
        }
    }
    Err(From::from("ok".to_string()))
}

fn get_deb_link(build_link: String, libc: &Libc) -> Result<String, Box<Error>> {
    let url = &*build_link;
    let builds = reqwest::get(url)?;
    let doc = Document::from_read(builds)?;

    let download_text= format!("libc6_{}_{}.deb", libc.version, libc.architecture);
    doc.find(Class("download"))
        .find(|n| n.text() == download_text)
        .and_then(|a| a.attr("href"))
        .map(|link| link.to_string())
        .ok_or(From::from("deb".to_string()))
}

//TODO this is wrong! need to download ELIBC instead of LIBC sometimes
fn get_build_link(libc: &Libc) -> Result<String, Box<Error>> {
    let url = &*format!("https://launchpad.net/ubuntu/+source/{}/{}",
        libc.libc_kind, libc.version);
    let search = reqwest::get(url)?;
    let doc = Document::from_read(search)?;

    doc.find(Descendant(Attr("id", "source-builds"), Name("a")))
        .find(|n| libc.architecture.to_string().eq(&n.text()))
        .and_then(|a| a.attr("href"))
        .map(|link| format!("https://launchpad.net{}", link))
        .ok_or(From::from("build".to_string()))
}

pub fn download_ld(libc: &Libc, download_dir: PathBuf) -> Result<(), Box<Error>>{
    let version = libc.version.to_string();
    let build_link = get_build_link(libc)?;
    let deb_link = get_deb_link(build_link, libc)?;
    Ok(extract_ld(deb_link, download_dir, version)?)
}

pub fn ld_download_dir(libc_path: String, dir_path: String) -> Result<PathBuf, Box<Error>> {
    let err:Box<Error> = From::from("wtf".to_string());
    let path = Path::new(&libc_path);
    let parent_dir = path.parent().ok_or(err)?;
    Ok(parent_dir.join(dir_path).canonicalize()?)
}

