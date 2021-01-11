use anyhow::Result;
use std::{io::Read, str::from_utf8};
use thiserror::Error;

#[derive(Debug, Error)]
enum DbFileError {
    #[error("HeaderBegginingError: {0}")]
    HeaderBegginingError(String),
    #[error("InvalidFileFormatWriteVersion: {0}")]
    InvalidFileFormatWriteVersion(u8),
    #[error("InvalidFileFormatReadVersion: {0}")]
    InvalidFileFormatReadVersion(u8),
}

enum FileFormatWriteVersion {
    LEGACY,
    WAL,
}
enum FileFormatReadVersion {
    LEGACY,
    WAL,
}

pub struct DbFile {
    db_page_size_in_bytes: u32,
    file_format_write_version: FileFormatWriteVersion,
    file_format_read_version: FileFormatReadVersion,
}

impl DbFile {
    pub fn from_read(read: &mut Read) -> Result<DbFile> {
        let buffer: &mut [u8; 16] = &mut [0; 16];
        read.read_exact(buffer)?;
        let beggining = std::str::from_utf8(buffer)?;
        if beggining != "SQLite format 3\u{0}" {
            Err(DbFileError::HeaderBegginingError(beggining.to_string()))?
        }

        let buffer: &mut [u8; 2] = &mut [0; 2];
        read.read_exact(buffer)?;
        let db_page_size_in_bytes = if *buffer == [0u8, 1] {
            65536
        } else {
            u32::from(u16::from_be_bytes(*buffer))
        };

        let buffer: &mut [u8; 1] = &mut [0; 1];
        read.read_exact(buffer)?;
        let file_format_write_version = match u8::from_be_bytes(*buffer) {
            1 => FileFormatWriteVersion::LEGACY,
            2 => FileFormatWriteVersion::WAL,
            other => Err(DbFileError::InvalidFileFormatWriteVersion(other))?,
        };

        let buffer: &mut [u8; 1] = &mut [0; 1];
        read.read_exact(buffer)?;
        let file_format_read_version = match u8::from_be_bytes(*buffer) {
            1 => FileFormatReadVersion::LEGACY,
            2 => FileFormatReadVersion::WAL,
            other => Err(DbFileError::InvalidFileFormatReadVersion(other))?,
        };

        Ok(DbFile {
            db_page_size_in_bytes,
            file_format_write_version,
            file_format_read_version,
        })
    }
}
