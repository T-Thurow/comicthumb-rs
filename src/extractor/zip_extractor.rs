use crate::extractor::Extractor;
use std::{
    fs,
    io::{self, Read},
};

pub struct ZipExtractor {
    src: String,
}

impl ZipExtractor {
    pub fn new(src: &str) -> Self {
        ZipExtractor {
            src: src.to_string(),
        }
    }
}

impl Extractor for ZipExtractor {
    fn extract(&self, chosen: &str) -> Result<Vec<u8>, io::Error> {
        let mut buffer: Vec<u8> = Vec::new();
        let file = fs::File::open(&self.src)?;
        let mut archive = zip::ZipArchive::new(file)?;
        let mut entry = archive.by_name(chosen)?;
        entry.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    fn get_files(&self) -> Result<Vec<String>, io::Error> {
        let file = fs::File::open(&self.src)?;
        let zipfile = zip::ZipArchive::new(file)?;
        let file_names = zipfile.file_names().map(|f| f.to_string()).collect();
        Ok(file_names)
    }
}
