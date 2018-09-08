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

fn download_and_extract_ld(url: String, version: String) -> Result<(), Box<Error>> {
    let fname: OsString = From::from(format!("ld-{}.so", version.split("-").collect::<Vec<_>>()[0]));
    let deb_read = reqwest::get(&*url)?;
    let mut deb = ar::Archive::new(deb_read);
    while let Some(entry_result) = deb.next_entry() {
        let mut entry = entry_result.unwrap();
        if str::from_utf8(entry.header().identifier()).unwrap() == "data.tar.xz"{
            let xzdec = XzDecoder::new(entry);
            let mut archive = tar::Archive::new(xzdec);
            for entry_result in archive.entries()?{
                let entry = entry_result.unwrap();
                let path = entry.path()?;
                let fname_res = path.file_name();
                if let Some(fname_this) = fname_res {
                    if &fname == fname_this {
                        println!("{:?}, {:?}", fname_this, entry.path());
                        break;
                    }
                }
            }
            break;
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
    download_and_extract_ld(deb_link, version)?;
    Ok("".to_string())
}
