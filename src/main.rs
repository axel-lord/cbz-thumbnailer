use ::std::{fs::File, io::BufReader, path::PathBuf};

use ::cbz_thumbnailer_lib::{image::ImageFormat, thumbnail};
use ::clap::Parser;
use ::color_eyre::eyre::eyre;
use ::tap::Pipe;

#[derive(Debug, Parser)]
struct Cli {
    input: PathBuf,
    output: PathBuf,
    size: Option<String>,
}

fn main() -> ::color_eyre::Result<()> {
    let Cli {
        input,
        output,
        size,
    } = Cli::parse();
    ::color_eyre::install()?;
    ::env_logger::builder()
        .filter_module("cbz_thumbnailer", ::log::LevelFilter::Warn)
        .init();

    let [w, h] = size
        .and_then(|s| {
            s.split_once('x')
                .and_then(|(w, h)| Some([w.parse().ok()?, h.parse().ok()?]))
                .or_else(|| {
                    let dim = s.parse().ok()?;
                    Some([dim, dim])
                })
        })
        .unwrap_or([128u32, 128u32]);

    let file = File::open(&input)
        .map_err(|err| eyre!("could not open {input:?}\n{err}"))?
        .pipe(BufReader::new);

    thumbnail(file, w, h)
        .map_err(|err| eyre!("could not generate thumbnail for {input:?}\n{err}"))?
        .save_with_format(&output, ImageFormat::Png)
        .map_err(|err| eyre!("could not save thumbnail for {input:?} to {output:?}\n{err}"))?;

    Ok(())
}
