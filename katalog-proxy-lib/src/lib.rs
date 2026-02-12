use ::std::{
    io::{Read, Write},
    path::{Path, PathBuf},
};

use ::serde::{Deserialize, Serialize};
use ::sha2::{Digest, Sha256};
use ::walkdir::WalkDir;

/// Contents of katalog proxy file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contents {
    /// Path to katalog.
    pub katalog: PathBuf,
    /// Hash of filenames in catalog.
    #[serde(with = "serde_bytes", skip_serializing_if = "Option::is_none", default)]
    pub name_hash: Option<Vec<u8>>,
}

/// Calculate name hash for a katalog directory.
pub fn katalog_name_hash(katalog: &Path) -> Vec<u8> {
    let hash = WalkDir::new(katalog)
        .same_file_system(true)
        .follow_links(false)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            entry.metadata().ok()?.is_file().then_some(entry)
        })
        .fold(Sha256::new(), |hasher, entry| {
            hasher.chain_update(entry.file_name().as_encoded_bytes())
        })
        .finalize();

    let mut output = Vec::with_capacity(hash.len());
    output.extend_from_slice(&hash);

    output
}

/// Erros which may occur reading content.
#[derive(Debug, ::thiserror::Error)]
pub enum ReadError {
    /// Io Error.
    #[error(transparent)]
    Io(#[from] ::std::io::Error),
    /// Deserialization error.
    #[error(transparent)]
    De(#[from] ::toml::de::Error),
}

impl Contents {
    /// Write content to writer.
    pub fn write<W: Write>(&self, mut dest: W) -> ::std::io::Result<()> {
        let content =
            ::toml::to_string_pretty(self).expect("Contents should always serialize to toml");

        dest.write_all(b"#katalog-proxy\n")?;
        dest.write_all(content.as_bytes())
    }

    /// Read content from reader.
    pub fn read<R: Read>(src: R) -> Result<Self, ReadError> {
        let content = ::std::io::read_to_string(src)?;
        let content = ::toml::from_str(&content)?;
        Ok(content)
    }
}
