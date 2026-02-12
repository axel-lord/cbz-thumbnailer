use ::std::{
    ffi::OsStr,
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use ::clap::Parser;
use ::color_eyre::eyre::eyre;
use ::katalog_proxy_lib::Contents;
use ::log::LevelFilter;

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

fn main() -> ::color_eyre::Result<()> {
    let Cli {
        proxy_file,
        catalog,
        skip_hash: _,
    } = Cli::parse();
    ::color_eyre::install()?;
    ::env_logger::builder()
        .filter_module("katalog_proxy_lib", LevelFilter::Info)
        .filter_module("katalog_proxy", LevelFilter::Info)
        .init();

    if let Some(_catalog) = catalog {
    } else {
        let mut file;
        let mut stdin;
        let reader: &mut dyn Read = if proxy_file.as_os_str() == OsStr::new("-") {
            stdin = ::std::io::stdin().lock();
            &mut stdin
        } else {
            file = File::open(&proxy_file)
                .map_err(|err| eyre!("could not open {proxy_file:?}\n{err}"))?;
            &mut file
        };
        let content = Contents::read(BufReader::new(reader))
            .map_err(|err| eyre!("could not read/parse {proxy_file:?}\n{err}"))?;
        let katalog = &content.katalog;

        ::open::that_detached(&content.katalog)
            .map_err(|err| eyre!("could not open {katalog:?}\n{err}"))?;
    }

    Ok(())
}
