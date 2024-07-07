use anyhow::{ensure, Result};
use std::{collections::HashSet, fs, path::PathBuf, str};

use crate::{input::config::ConfigInput, template::Template};

#[derive(Debug)]
pub struct Config {
    templates: HashSet<Template>,
}

impl Config {
    pub fn new(
        config_location: PathBuf,
        templates_location: PathBuf,
        output_location: PathBuf,
    ) -> Result<Self> {
        // check if everything is valid
        ensure!(config_location.is_file(), "Config location is not a file");
        ensure!(
            templates_location.is_dir(),
            "Template location is not a dir"
        );

        // get contents of the config
        let contents = fs::read(config_location)?;
        let contents = str::from_utf8(&contents)?;

        // deserialize to struct ConfigInput
        let mut config_input: ConfigInput = toml::from_str(contents)?;

        // get templates from the validated config
        let templates = config_input
            .validate(&templates_location, &output_location)?
            .construct(&templates_location)?;

        Ok(Self { templates })
    }

    pub fn output(&self) -> Result<()> {
        for template in self.templates.iter() {
            template.output()?;
        }

        Ok(())
    }
}
