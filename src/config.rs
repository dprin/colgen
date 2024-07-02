use anyhow::{ensure, Result};
use std::{collections::HashSet, fs, path::PathBuf, str};

use crate::{input::ConfigInput, template::Template};

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
        let config_input: ConfigInput = toml::from_str(contents)?;
        config_input.validate(&templates_location)?;

        // load all templates
        let mut templates: HashSet<Template> = if let Some(settings) = &config_input.settings {
            settings
                .iter()
                .map(|(name, v)| {
                    v.convert_to_template(
                        &output_location,
                        name,
                        &templates_location,
                        &config_input.colorschemes,
                    )
                })
                .collect()
        } else {
            HashSet::new()
        };

        let input_paths: Vec<_> = templates
            .iter()
            .map(|template| template.input.clone())
            .collect();

        for entry in fs::read_dir(&templates_location)? {
            let entry = entry.unwrap();

            if input_paths.contains(&entry.path()) {
                continue;
            }

            let name = entry.file_name().into_string().unwrap();

            templates.insert(Template {
                input: templates_location.join(&name),
                theme: config_input.colorschemes.get("default").unwrap().clone(),
                output: output_location.join(&name),
            });
        }

        Ok(Self { templates })
    }

    pub fn output(&self) -> Result<()> {
        for template in self.templates.iter() {
            template.output()?;
        }

        Ok(())
    }
}
