use anyhow::{ensure, Result};
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs,
    path::{Path, PathBuf},
    str,
};

use crate::template::{Colorscheme, Template};

#[derive(Debug)]
pub struct Config {
    templates: HashSet<Template>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigInput {
    colorschemes: HashMap<String, Colorscheme>,
    settings: Option<HashMap<String, TemplateInput>>,
}

#[derive(Debug, Deserialize, Clone)]
struct TemplateInput {
    /// The theme to use.
    theme: Option<String>,
    /// The output **directory**.
    output: Option<PathBuf>,
    /// The new file name.
    name: Option<String>,
}

impl TemplateInput {
    fn convert_to_template(
        &self,
        output: &PathBuf,
        input_name: &String,
        templates_path: &Path,
        colorschemes: &HashMap<String, Colorscheme>,
    ) -> Template {
        let colorscheme = if let Some(theme) = &self.theme {
            colorschemes.get(theme).unwrap()
        } else {
            colorschemes.get("default").unwrap()
        }
        .clone();

        let name = if let Some(name) = &self.name {
            name
        } else {
            input_name
        };

        let output = if let Some(output) = &self.output {
            output
        } else {
            output
        }
        .to_path_buf()
        .join(name);

        Template {
            theme: colorscheme,
            output,
            input: templates_path.join(input_name),
        }
    }
}

#[derive(Debug)]
enum ConfigLoadError {
    NoDefaultFound,
    ColorschemeNotFound(String),
}

impl Display for ConfigLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::NoDefaultFound => "No \"default\" color scheme found.".to_string(),
            Self::ColorschemeNotFound(name) => format!("Colorscheme {name} not found"),
        };

        f.write_str(&output)?;

        Ok(())
    }
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

impl ConfigInput {
    fn validate(&self, template_loc: &Path) -> Result<()> {
        ensure!(
            self.colorschemes.contains_key("default"),
            ConfigLoadError::NoDefaultFound
        );

        if self.settings.is_none() {
            Ok(())
        } else {
            let settings = self.settings.as_ref().unwrap();

            for (name, template) in settings.iter() {
                ensure!(
                    template_loc.join(name).exists(),
                    format!("File \"{name}\" does not exist")
                );

                if let Some(theme) = &template.theme {
                    ensure!(
                        self.colorschemes.contains_key(theme),
                        ConfigLoadError::ColorschemeNotFound(theme.to_string())
                    )
                }
            }

            Ok(())
        }
    }
}
