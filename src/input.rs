use std::{
    collections::HashMap, fmt::Display, path::{Path, PathBuf}
};

use anyhow::{ensure, Result};
use serde::Deserialize;

use crate::template::{Colorscheme, Template};

#[derive(Debug, Deserialize)]
pub struct ConfigInput {
    pub colorschemes: HashMap<String, Colorscheme>,
    pub settings: Option<HashMap<String, TemplateInput>>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct TemplateInput {
    /// The theme to use.
    theme: Option<String>,
    /// The output **directory**.
    output: Option<PathBuf>,
    /// The new file name.
    name: Option<String>,
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

impl TemplateInput {
    pub(crate) fn convert_to_template(
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

impl ConfigInput {
    pub(crate) fn validate(&self, template_loc: &Path) -> Result<()> {
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
