use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{ensure, Result};
use serde::Deserialize;

use crate::{
    input::ConfigLoadError,
    intermediate::{colorscheme::ColorschemeIntermediate, template::TemplateIntermediate},
};

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct TemplateInput {
    /// The theme to use.
    pub(super) theme: Option<String>,
    /// The output **directory**.
    output: Option<PathBuf>,
    /// The new file name.
    name: Option<String>,
}

impl TemplateInput {
    pub(crate) fn validate(
        &self,
        filename: String,
        colorschemes: &HashMap<String, ColorschemeIntermediate>,
        template_loc: &Path,
    ) -> Result<TemplateIntermediate> {
        ensure!(
            template_loc.join(&filename).exists(),
            format!("File \"{filename}\" does not exist")
        );

        let theme = if let Some(theme) = &self.theme {
            ensure!(
                colorschemes.contains_key(theme),
                ConfigLoadError::ColorschemeNotFound(theme.to_string())
            );
            theme.clone()
        } else {
            "default".to_string()
        };

        let name = if let Some(name) = &self.name {
            name.clone()
        } else {
            filename
        };

        let output = if let Some(output) = &self.output {
            output.join(&name)
        } else {
            template_loc.join(&name)
        };

        Ok(TemplateIntermediate { theme, output })
    }
}
