use crate::extractor::Extractor;
use flate2::read::GzDecoder;
use std::{
    fs,
    io::{self, Read},
};
use tar;

pub struct GzipExtractor {
    src: String,
}

impl GzipExtractor {
    pub fn new(src: &String) -> Self {
        GzipExtractor { src: src.clone() }
    }
}

impl Extractor for GzipExtractor {
    fn extract(&self, chosen: &str) -> Result<Vec<u8>, io::Error> {
        let mut buffer: Vec<u8> = Vec::new();
        let file = fs::File::open(&self.src)?;
        let archive_outer = GzDecoder::new(file);
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
        let archive_outer = GzDecoder::new(file);
        let mut archive_inner = tar::Archive::new(archive_outer);
        let entries = archive_inner.entries()?;
        let file_names = entries
            .filter_map(Result::ok)
            .filter_map(|entry| match entry.path() {
                Ok(path) => Some(path.to_string_lossy().to_string()),
                _ => None,
            })
            .collect();
        Ok(file_names)
    }
}
