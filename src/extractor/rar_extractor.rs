use crate::extractor::Extractor;
use std::io;
use unrar;

pub struct RarExtractor {
    src: String,
}

impl RarExtractor {
    pub fn new(src: &String) -> Self {
        RarExtractor { src: src.clone() }
    }
}

impl Extractor for RarExtractor {
    fn get_files(&self) -> Result<Vec<String>, io::Error> {
        let archive = unrar::Archive::new(&self.src)
            .open_for_listing()
            .map_err(io::Error::other)?;
        let file_names = archive
            .filter_map(Result::ok)
            .filter(|x| x.is_file())
            .map(|y| y.filename.to_string_lossy().to_string())
            .collect();
        Ok(file_names)
    }

    fn extract(&self, chosen: &str) -> Result<Vec<u8>, io::Error> {
        let mut archive = unrar::Archive::new(&self.src)
            .open_for_processing()
            .map_err(io::Error::other)?;
        while let Some(header) = archive.read_header().map_err(io::Error::other)? {
            archive = if header.entry().filename.to_string_lossy() == chosen {
                let (data, _) = header.read().map_err(io::Error::other)?;
                return Ok(data);
            } else {
                header.skip().map_err(io::Error::other)?
            }
        }
        Err(io::Error::other("File not found"))
    }
}
