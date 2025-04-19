use crate::extractor::Extractor;
use std::{
    fs,
    io::{self, Read},
};

pub struct TarExtractor {
    src: String,
}

impl TarExtractor {
    pub fn new(src: &str) -> Self {
        TarExtractor {
            src: src.to_string(),
        }
    }
}

impl Extractor for TarExtractor {
    fn extract(&self, chosen: &str) -> Result<Vec<u8>, io::Error> {
        let mut buffer: Vec<u8> = Vec::new();
        let file = fs::File::open(&self.src)?;
        let mut archive = tar::Archive::new(file);
        let result = archive
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
        let mut tfile = tar::Archive::new(file);
        let entries = tfile.entries()?;
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
