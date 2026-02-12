use ::std::{
    collections::BTreeSet,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use ::clap::Parser;
use ::color_eyre::eyre::eyre;
use ::katalog_proxy_lib::Contents;
use ::tap::Pipe as _;
use ::thumbnailer_common::{Size, ThumbnailerArgs};
use ::walkdir::WalkDir;

/// Kind of file found by dir walk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum FoundKind {
    Cover = 1,
    Archive = 2,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Found {
    /// Kind of file found.
    kind: FoundKind,
    /// Path of file.
    path: PathBuf,
}

/// Generate thumbnails for katalog proxies.
#[derive(Debug, Parser)]
pub struct Cli {
    #[command(flatten)]
    args: ThumbnailerArgs,
}

impl Cli {
    pub fn run(self) -> ::color_eyre::Result<()> {
        let Cli {
            args:
                ThumbnailerArgs {
                    input,
                    output,
                    size: Size { width, height },
                },
        } = self;

        let file = File::open(&input)
            .map_err(|err| eyre!("could not open {input:?}\n{err}"))?
            .pipe(BufReader::new);

        let content =
            Contents::read(file).map_err(|err| eyre!("could not read/parse {input:?}\n{err}"))?;

        let walkdir = WalkDir::new(&content.katalog)
            .same_file_system(true)
            .follow_links(false)
            .into_iter()
            .filter_map(|entry| {
                entry
                    .ok()
                    .filter(|entry| entry.metadata().is_ok_and(|meta| meta.is_file()))
            });

        fn is_cbz(file: &Path) -> bool {
            let Some(ext) = file.extension() else {
                return false;
            };

            ext.eq_ignore_ascii_case("cbz")
        }

        fn is_cover(file: &Path) -> bool {
            let Some(name) = file.file_prefix() else {
                return false;
            };

            name.eq_ignore_ascii_case("cover") || name.eq_ignore_ascii_case(".cover")
        }

        let mut found = BTreeSet::new();

        for entry in walkdir {
            let path = entry.into_path();
            let kind = if is_cbz(&path) {
                FoundKind::Archive
            } else if is_cover(&path) {
                FoundKind::Cover
            } else {
                continue;
            };

            found.insert(Found { kind, path });
        }

        for found in found {
            let path = found.path;
            let thumbnail = match found.kind {
                FoundKind::Cover => {
                    let Ok(cover) = ::image::open(&path) else {
                        continue;
                    };

                    cover.thumbnail(width, height)
                }
                FoundKind::Archive => {
                    let Ok(archive) = File::open(&path) else {
                        continue;
                    };

                    let Ok(thumbnail) =
                        ::cbz_thumbnailer_lib::thumbnail(BufReader::new(archive), width, height)
                    else {
                        continue;
                    };

                    thumbnail
                }
            };

            return thumbnail
                .save_with_format(&output, ::image::ImageFormat::Png)
                .map_err(|err| {
                    eyre!("could not save cover thumbnail read from {path:?} to {output:?}\n{err}")
                });
        }

        Ok(())
    }
}
