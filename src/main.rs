use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use config::Config;

mod config;
mod template;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// location of the config file
    #[arg(short, long, default_value = "./config.toml")]
    config: PathBuf,

    /// location of the input template directories
    #[arg(short, long, default_value = "./templates")]
    templates: PathBuf,

    /// location of the output directory
    #[arg(short, long, default_value = "./output")]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{args:?}");

    let config = Config::new(args.config, args.templates, args.output)?;
    println!("{config:?}");

    config.output()?;

    Ok(())
}
