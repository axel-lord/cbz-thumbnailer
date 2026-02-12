use ::core::{fmt::Display, str::FromStr};
use ::std::path::PathBuf;

use ::clap::Args;

const DEFAULT_SIZE: u32 = 128;

/// Common thumbnailer arguments.
#[derive(Debug, Args, Clone)]
pub struct ThumbnailerArgs {
    /// File to generate thumbnail for.
    pub input: PathBuf,

    /// Destination to write thumbnail to.
    pub output: PathBuf,

    /// Size of thumbnail.
    #[arg(default_value_t)]
    pub size: Size,
}

/// Thumbnail size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size {
    /// Width of thumbnail.
    pub width: u32,
    /// Height of thumbnail.
    pub height: u32,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { width, height } = self;
        write!(f, "{width}x{height}")
    }
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: DEFAULT_SIZE,
            height: DEFAULT_SIZE,
        }
    }
}

impl FromStr for Size {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((w, h)) = s.split_once('x') {
            Ok(Self {
                width: w.parse()?,
                height: h.parse()?,
            })
        } else {
            let dim = s.parse()?;
            Ok(Self {
                width: dim,
                height: dim,
            })
        }
    }
}
