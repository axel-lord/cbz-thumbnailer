use ::std::{
    collections::BTreeMap,
    ffi::OsStr,
    io::{Read, Seek},
    os::unix::ffi::OsStrExt,
};

use ::image::DynamicImage;

/// Reexport of image crate.
pub use ::image;
use ::tap::{Pipe, TryConv};
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
        let mut data = match file.by_index(idx) {
            Ok(data) => data,
            Err(err) => {
                ::log::warn!("could not get file with index {idx} in archive\n{err}");
                continue;
            }
        };

        // Probably not an image if compression is greater than by 4.
        if data
            .size()
            .checked_div(data.compressed_size())
            .unwrap_or(u64::MAX)
            > 4
        {
            continue;
        }

        let mut buf = data
            .size()
            .try_conv::<usize>()
            .unwrap_or_default()
            .pipe(Vec::with_capacity);
        if let Err(err) = data.read_to_end(&mut buf) {
            ::log::warn!("could not read file with index {idx} in archive\n{err}");
            continue;
        };

        if let Ok(img) = ::image::load_from_memory(&buf) {
            return Ok(img.thumbnail(width, height));
        };
    }

    Err(Error::NoImage)
}
