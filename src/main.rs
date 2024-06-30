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
    config: String,

    /// location of the input template directories
    #[arg(short, long, default_value = "./templates")]
    templates: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{args:?}");

    let config = Config::new(args.config.into(), args.templates.into())?;
    println!("{config:?}");

    config.output()?;

    Ok(())
}
