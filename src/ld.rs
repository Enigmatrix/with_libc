use libc::Libc;
use std::error::Error;
use std::str;
use std::ffi::OsString;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Descendant};
use xz2::read::XzDecoder;
use tar;
use ar;
use reqwest;

fn ld_fname(version: String) -> OsString {
    let short_version = version.split("-").collect::<Vec<_>>()[0];
    let fname_string = format!("ld-{}.so", short_version);
    From::from(fname_string)
}

fn extract_ld(url: String, version: String) -> Result<(), Box<Error>> {
    let ld_fname_s = ld_fname(version);
    let ld_file_name = ld_fname_s.as_os_str();

    let deb_read = reqwest::get(&*url)?;
    let mut deb = ar::Archive::new(deb_read);

    while let Some(Ok(entry)) = deb.next_entry() {
        if str::from_utf8(entry.header().identifier())? != "data.tar.xz" {
            continue
        }
        let xzdec = XzDecoder::new(entry);
        let mut archive = tar::Archive::new(xzdec);

        let ld_opt = archive.entries()?
            .map(|e| e.unwrap())
            .find(|e| Some(ld_file_name) ==
                  e.path().unwrap().file_name());

        if let Some(ld) = ld_opt {
            // ld.unpack(<path>);
        }
    }
    Ok(())
}

fn get_deb_link(build_link: String, architecture: &String, version: &String) -> Result<String, Box<Error>> {
    let url = &*build_link;
    let builds = reqwest::get(url)?;
    let doc = Document::from_read(builds)?;

    let download_text= format!("libc6_{}_{}.deb", version, architecture);
    doc.find(Class("download"))
        .find(|n| n.text() == download_text)
        .and_then(|a| a.attr("href"))
        .map(|link| link.to_string())
        .ok_or(From::from("".to_string()))
}

fn get_build_link(architecture:&String, version:&String) -> Result<String, Box<Error>> {
    let url = &*format!("https://launchpad.net/ubuntu/+source/glibc/{}", version);
    let search = reqwest::get(url)?;
    let doc = Document::from_read(search)?;

    doc.find(Descendant(Attr("id", "source-builds"), Name("a")))
        .find(|n| architecture.eq(&n.text()))
        .and_then(|a| a.attr("href"))
        .map(|link| format!("https://launchpad.net{}", link))
        .ok_or(From::from("".to_string()))
}

pub fn download_ld(libc: &Libc) -> Result<String, Box<Error>>{
    let architecture = libc.architecture.to_string();
    let version = libc.version.to_string();
    let build_link = get_build_link(&architecture, &version)?;
    let deb_link = get_deb_link(build_link, &architecture, &version)?;
    extract_ld(deb_link, version)?;
    Ok("".to_string())
}
