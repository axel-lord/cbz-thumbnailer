use ::clap::Parser;
use ::log::LevelFilter;

use ::katalog_proxy::Cli;

fn main() -> ::color_eyre::Result<()> {
    let cli = Cli::parse();
    ::color_eyre::install()?;
    ::env_logger::builder()
        .filter_module("katalog_proxy_lib", LevelFilter::Info)
        .filter_module("katalog_proxy", LevelFilter::Info)
        .init();
    cli.run()
}
