use ::std::{
    ffi::OsStr,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use ::clap::Parser;
use ::color_eyre::eyre::eyre;
use ::either::Either;
use ::katalog_proxy_lib::{Contents, katalog_name_hash};
use ::log::LevelFilter;
use ::tap::Pipe;

/// Create and open proxies for catalogs.
#[derive(Debug, Parser)]
struct Cli {
    /// File to open/create. If '-' stdin/stdout is used.
    proxy_file: PathBuf,

    /// Catalog to create proxy for.
    catalog: Option<PathBuf>,

    /// Do not update/include name hash.
    #[clap(long)]
    skip_hash: bool,
}

fn open_reader(p: &Path) -> ::color_eyre::Result<BufReader<impl Read>> {
    if p.as_os_str() == OsStr::new("-") {
        ::std::io::stdin().lock().pipe(Either::Left)
    } else {
        File::open(p)
            .map_err(|err| eyre!("could not open {p:?}\n{err}"))?
            .pipe(Either::Right)
    }
    .pipe(BufReader::new)
    .pipe(Ok)
}

fn open_writer(p: &Path) -> ::color_eyre::Result<BufWriter<impl Write>> {
    if p.as_os_str() == OsStr::new("-") {
        ::std::io::stdout().lock().pipe(Either::Left)
    } else {
        File::create(p)
            .map_err(|err| eyre!("could not open {p:?}\n{err}"))?
            .pipe(Either::Right)
    }
    .pipe(BufWriter::new)
    .pipe(Ok)
}

fn main() -> ::color_eyre::Result<()> {
    let Cli {
        proxy_file,
        catalog,
        skip_hash,
    } = Cli::parse();
    ::color_eyre::install()?;
    ::env_logger::builder()
        .filter_module("katalog_proxy_lib", LevelFilter::Info)
        .filter_module("katalog_proxy", LevelFilter::Info)
        .init();

    if let Some(_catalog) = catalog {
    } else {
        let content = open_reader(&proxy_file)?
            .pipe(Contents::read)
            .map_err(|err| eyre!("could not read/parse {proxy_file:?}\n{err}"))?;
        let katalog = &content.katalog;

        ::open::that_detached(&content.katalog)
            .map_err(|err| eyre!("could not open {katalog:?}\n{err}"))?;

        if !skip_hash {
            let name_hash = katalog_name_hash(&content.katalog);
            if content.name_hash.as_ref().is_none_or(|n| n != &name_hash) {
                Contents {
                    name_hash: if name_hash.is_empty() {
                        None
                    } else {
                        Some(name_hash)
                    },
                    ..content
                }
                .write(open_writer(&proxy_file)?)
                .map_err(|err| eyre!("could not write to {proxy_file:?}\n{err}"))?;
            }
        }
    }

    Ok(())
}
