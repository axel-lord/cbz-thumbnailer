use ::cbz_thumbnailer::Cli;
use ::clap::Parser;
use ::log::LevelFilter;

fn main() -> ::color_eyre::Result<()> {
    let cli = Cli::parse();
    ::color_eyre::install()?;
    ::env_logger::builder()
        .filter_module("cbz_thumbnailer", LevelFilter::Info)
        .filter_module("cbz_thumbnailer_lib", LevelFilter::Info)
        .init();
    if !::jxl_oxide::integration::register_image_decoding_hook() {
        ::log::warn!("could not register jpeg xl hook");
    }

    cli.run()
}
