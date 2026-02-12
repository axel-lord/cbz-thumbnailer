use ::clap::Parser;
use ::katalog_proxy_thumbnailer::Cli;
use ::log::LevelFilter;

fn main() -> ::color_eyre::Result<()> {
    let cli = Cli::parse();
    ::color_eyre::install()?;
    ::env_logger::builder()
        .filter_module("katalog_proxy_thumbnailer", LevelFilter::Info)
        .filter_module("cbz_thumbnailer_lib", LevelFilter::Info)
        .init();

    cli.run()
}
