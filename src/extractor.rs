mod bzip2_extractor;
mod extractor_type;
mod gzip_extractor;
mod rar_extractor;
mod tar_extractor;
mod zip_extractor;

use bzip2_extractor::Bzip2Extractor;
use extractor_type::{archive_mime_type, ZipType};
use gzip_extractor::GzipExtractor;
use rar_extractor::RarExtractor;
use tar_extractor::TarExtractor;
use zip_extractor::ZipExtractor;

pub trait Extractor {
    fn get_files(&self) -> Result<Vec<String>, std::io::Error>;
    fn extract(&self, chosen: &str) -> Result<Vec<u8>, std::io::Error>;
}

pub fn get_extractor(src: &String) -> std::io::Result<Box<dyn Extractor>> {
    let archive_type = archive_mime_type(&src)?;
    match archive_type {
        ZipType::ZIP => Ok(Box::new(ZipExtractor::new(src))),
        ZipType::TAR => Ok(Box::new(TarExtractor::new(src))),
        ZipType::GZIP => Ok(Box::new(GzipExtractor::new(src))),
        ZipType::BZIP2 => Ok(Box::new(Bzip2Extractor::new(src))),
        ZipType::RAR => Ok(Box::new(RarExtractor::new(src))),
        _ => Err(std::io::Error::other("Unknown Archive")),
    }
}
