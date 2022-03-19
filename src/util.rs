use anyhow::Result;
use std::ffi::OsStr;
use std::io;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use flate2::read::GzDecoder;

pub(crate) fn download(url: &str) -> Result<Vec<u8>> {
    let resp = ureq::get(url).call();
    if resp.error() {
        return Err(anyhow!("Got bad status code {}", resp.status()));
    }

    let mut reader = io::BufReader::new(resp.into_reader());
    let mut bytes = vec![];
    reader.read_to_end(&mut bytes)?;

    Ok(bytes)
}

pub fn unpack(archive_name: &PathBuf, destination: &PathBuf,bytes: Vec<u8>) -> Result<()> {
    match archive_name.extension().and_then(OsStr::to_str) {
        Some("zip") => Ok(zip_extract::extract(Cursor::new(bytes), destination, true)?),
        Some("gz") => {
            let mut archive = tar::Archive::new(GzDecoder::new(&bytes[..]));
            Ok(archive
                .entries()?
                .filter_map(|e| e.ok())
                .map(|mut entry| -> Result<PathBuf> {
                    let toplevel_layers = 2;
                    let path = entry.path()?.components().skip(toplevel_layers).collect();
                    entry.unpack(destination.join(&path))?;
                    Ok(path)
                })
                .filter_map(|e| e.ok())
               .for_each(|x| debug!("> {}", x.display()))
            )
        }
        _ => panic!("Archive format not supported!")
    }
}
