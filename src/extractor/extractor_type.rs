use std::{
    fs,
    io::{self, Read, Seek},
    path::Path,
};

#[derive(Copy, Clone, Debug)]
pub enum ZipType {
    ZIP,
    RAR,
    TAR,
    GZIP,
    BZIP2,
    UNKNOWN,
}

pub fn archive_mime_type(path: &str) -> io::Result<ZipType> {
    if Path::new(path).is_file() {
        let mut file = fs::File::open(path)?;
        let mut magic = [0; 4];

        file.read_exact(&mut magic)?;

        if &magic[..4] == [80, 75, 3, 4] {
            return Ok(ZipType::ZIP);
        }

        if &magic[..4] == b"Rar!" {
            return Ok(ZipType::RAR);
        }

        if &magic[..2] == b"BZ" {
            return Ok(ZipType::BZIP2);
        }

        if &magic[..2] == b"\x1F\x8B" {
            return Ok(ZipType::GZIP);
        }

        let mut ustar = [0; 6];
        file.seek(io::SeekFrom::Start(257))?;
        file.read_exact(&mut ustar)?;

        if &ustar[..6] == b"ustar\0" {
            return Ok(ZipType::TAR);
        }
    }
    Ok(ZipType::UNKNOWN)
}
