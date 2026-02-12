use ::std::{fs::File, io::BufReader};

use ::cbz_thumbnailer_lib::{image::ImageFormat, thumbnail};
use ::clap::Parser;
use ::color_eyre::eyre::eyre;
use ::tap::Pipe;
use ::thumbnailer_common::{Size, ThumbnailerArgs};

/// Generate thumbnails for cbz archives.
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

        thumbnail(file, width, height)
            .map_err(|err| eyre!("could not generate thumbnail for {input:?}\n{err}"))?
            .save_with_format(&output, ImageFormat::Png)
            .map_err(|err| eyre!("could not save thumbnail for {input:?} to {output:?}\n{err}"))?;

        Ok(())
    }
}
