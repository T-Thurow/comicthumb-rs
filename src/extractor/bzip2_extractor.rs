use crate::extractor::Extractor;
use bzip2::read::BzDecoder;
use std::{
    fs,
    io::{self, Read},
};
use tar;

pub struct Bzip2Extractor {
    src: String,
}

impl Bzip2Extractor {
    pub fn new(src: &str) -> Self {
        Bzip2Extractor {
            src: src.to_string(),
        }
    }
}

impl Extractor for Bzip2Extractor {
    fn extract(&self, chosen: &str) -> Result<Vec<u8>, io::Error> {
        let mut buffer: Vec<u8> = Vec::new();
        let file = fs::File::open(&self.src)?;
        let archive_outer = BzDecoder::new(file);
        let mut archive_inner = tar::Archive::new(archive_outer);
        let result = archive_inner
            .entries()?
            .filter_map(Result::ok)
            .find(|entry| match entry.path() {
                Ok(path) => path.to_string_lossy() == chosen,
                _ => false,
            });
        match result {
            Some(mut entry) => entry.read_to_end(&mut buffer)?,
            _ => return Err(io::Error::other("File not found")),
        };
        Ok(buffer)
    }

    fn get_files(&self) -> Result<Vec<String>, io::Error> {
        let file = fs::File::open(&self.src)?;
        let archive_outer = BzDecoder::new(file);
        let mut archive_inner = tar::Archive::new(archive_outer);
        let entries = archive_inner.entries()?;
        let file_names = entries
            .filter_map(Result::ok)
            .filter_map(|entry| match entry.path() {
                Ok(path) => Some(path.to_string_lossy().to_string()),
                Err(_) => None,
            })
            .collect();
        Ok(file_names)
    }
}
