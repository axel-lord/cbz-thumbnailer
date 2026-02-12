use ::std::{
    ffi::OsStr,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use ::clap::{Parser, ValueEnum};
use ::color_eyre::eyre::eyre;
use ::either::Either;
use ::katalog_proxy_lib::{Contents, NameHash};
use ::tap::Pipe;

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
    if p.as_os_str() == "-" {
        ::std::io::stdout().lock().pipe(Either::Left)
    } else {
        File::create(p)
            .map_err(|err| eyre!("could not open {p:?}\n{err}"))?
            .pipe(Either::Right)
    }
    .pipe(BufWriter::new)
    .pipe(Ok)
}

/// Boolean like value which may be always, never or auto.
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ValueEnum)]
pub enum TriState {
    /// Always true.
    Always,
    /// Never true.
    Never,
    /// Decided by other factors.
    #[default]
    Auto,
}

impl TriState {
    /// Return a bool based either on value or conditional function.
    pub fn or_else(self, map_auto: impl FnOnce() -> bool) -> bool {
        match self {
            TriState::Always => true,
            TriState::Never => false,
            TriState::Auto => map_auto(),
        }
    }

    /// Return a bool based either on value or conditional value.
    pub fn or(self, map_auto: bool) -> bool {
        match self {
            TriState::Always => true,
            TriState::Never => false,
            TriState::Auto => map_auto,
        }
    }

    /// Using either value or on_auto create option with wiht value if true.
    pub fn then_some<T>(
        self,
        map_auto: impl FnOnce() -> bool,
        on_true: impl FnOnce() -> T,
    ) -> Option<T> {
        if self.or_else(map_auto) {
            Some(on_true())
        } else {
            None
        }
    }

    /// Using either value or on_auto create option with wiht value if false.
    pub fn else_some<T>(
        self,
        map_auto: impl FnOnce() -> bool,
        on_false: impl FnOnce() -> T,
    ) -> Option<T> {
        if self.or_else(map_auto) {
            None
        } else {
            Some(on_false())
        }
    }
}

/// Create and open proxies for catalogs.
#[derive(Debug, Parser)]
pub struct Cli {
    /// File to open/create. If '-' stdin/stdout is used.
    proxy_file: PathBuf,

    /// Catalog to create proxy for.
    catalog: Option<PathBuf>,

    /// Do not update/include name hash.
    #[clap(
        long,
        value_enum,
        require_equals = true,
        num_args = 0..=1,
        default_missing_value = "always",
        default_value_t
    )]
    skip_hash: TriState,
}

impl Cli {
    pub fn run(self) -> ::color_eyre::Result<()> {
        let Cli {
            proxy_file,
            catalog,
            skip_hash,
        } = self;
        if let Some(parent) = proxy_file.parent() {
            _ = ::std::env::set_current_dir(parent);
        }
        if let Some(catalog) = catalog {
            let name_hash = skip_hash.else_some(|| false, || NameHash::from_katalog(&catalog));

            Contents {
                katalog: catalog.clone(),
                name_hash,
            }
            .write(open_writer(&proxy_file)?)
            .map_err(|err| eyre!("could not write to {proxy_file:?}\n{err}"))?;
        } else {
            let content = open_reader(&proxy_file)?
                .pipe(Contents::read)
                .map_err(|err| eyre!("could not read/parse {proxy_file:?}\n{err}"))?;
            let katalog = &content.katalog;

            ::open::that_detached(&content.katalog)
                .map_err(|err| eyre!("could not open {katalog:?}\n{err}"))?;

            let name_hash = skip_hash.else_some(
                || content.name_hash.is_none(),
                || NameHash::from_katalog(&content.katalog),
            );
            if (name_hash.is_some() && content.name_hash != name_hash)
                || proxy_file.as_os_str() == "-"
            {
                Contents {
                    name_hash,
                    ..content
                }
                .write(open_writer(&proxy_file)?)
                .map_err(|err| eyre!("could not write to {proxy_file:?}\n{err}"))?;
            }
        }

        Ok(())
    }
}
