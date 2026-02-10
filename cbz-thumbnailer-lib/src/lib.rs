use ::std::{
    collections::BTreeMap,
    ffi::OsStr,
    io::{BufReader, Read, Seek},
    os::unix::ffi::OsStrExt,
};

use ::image::{DynamicImage, ImageReader};

/// Reexport of image crate.
pub use ::image;
use ::tap::Pipe as _;
use ::zip::{ZipArchive, result::ZipError};

/// Errors which may occur opening archive.
#[derive(Debug, ::thiserror::Error)]
pub enum Error {
    /// Given archive could not be read as a zip file.
    #[error("archive could not be read as a zip file\n{0}")]
    NotZip(ZipError),

    /// No suitable image file was found in archive.
    #[error("archive contained no suitable image file")]
    NoImage,
}

/// Generate a thumbnail from a zip archive.
pub fn thumbnail<A: Read + Seek>(
    archive: A,
    width: u32,
    height: u32,
) -> Result<DynamicImage, Error> {
    let mut file = ZipArchive::new(archive).map_err(Error::NotZip)?;
    let mut filenames = BTreeMap::new();

    for idx in 0..file.len() {
        let data = match file.by_index_raw(idx) {
            Ok(data) => data,
            Err(err) => {
                ::log::warn!("could not get file with index {idx} in archive\n{err}");
                continue;
            }
        };

        if data.is_file() {
            filenames.insert(OsStr::from_bytes(data.name_raw()).to_os_string(), idx);
        }
    }

    for idx in filenames.into_values() {
        let data = match file.by_index_seek(idx) {
            Ok(data) => data,
            Err(err) => {
                ::log::warn!("could not get file with index {idx} in archive\n{err}");
                continue;
            }
        };

        if let Ok(reader) = data
            .pipe(BufReader::new)
            .pipe(ImageReader::new)
            .with_guessed_format()
            && let Ok(image) = reader.decode()
        {
            return Ok(image.thumbnail(width, height));
        };
    }

    Err(Error::NoImage)
}
