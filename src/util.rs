use anyhow::Result;
use std::io::Cursor;
use std::path::PathBuf;

pub(crate) fn download(url: &str) -> Result<Vec<u8>> {
    let resp = ureq::get(url).call()?;
    let bytes = resp.into_body().read_to_vec()?;

    Ok(bytes)
}

pub fn unzip(destination: &PathBuf, bytes: Vec<u8>) -> Result<()> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes))?;
    let res = archive.extract_unwrapped_root_dir(destination, zip::read::root_dir_common_filter);
    Ok(res?)
}
