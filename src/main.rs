use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::Parser;
use config::Config;
use xdg::BaseDirectories;

mod config;
mod input;
mod intermediate;
mod template;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// location of the config file
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// location of the input template directories
    #[arg(short, long)]
    templates: Option<PathBuf>,

    /// location of the output directory
    #[arg(short, long)]
    output: Option<PathBuf>,
}

struct DefaultArgs {
    xdg: BaseDirectories,
}

impl DefaultArgs {
    fn new() -> Result<Self> {
        let xdg = BaseDirectories::new()?;

        Ok(Self { xdg })
    }

    fn create_file(&self, path: impl AsRef<Path>) -> Result<PathBuf> {
        let config_dir = self.xdg.create_config_directory("colgen")?;
        let complete_path = config_dir.join(path);

        if !complete_path.exists() {
            File::create(&complete_path)?;
        }

        Ok(complete_path)
    }

    fn create_dir(&self, path: impl AsRef<Path>) -> Result<PathBuf> {
        let config_dir = self.xdg.create_config_directory("colgen")?;
        let complete_path = config_dir.join(path);

        if !complete_path.exists() {
            fs::create_dir(&complete_path)?;
        }

        Ok(complete_path)
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let defaults = DefaultArgs::new()?;

    let config = match args.config {
        Some(n) => n,
        None => defaults.create_file("config.toml")?,
    };

    let templates = match args.templates {
        Some(n) => n,
        None => defaults.create_dir("templates")?,
    };

    let output = match args.output {
        Some(n) => n,
        None => defaults.create_dir("output")?,
    };

    let config = Config::new(config, output, templates)?;
    config.output()?;

    Ok(())
}
