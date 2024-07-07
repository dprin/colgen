use std::{collections::HashMap, fs, path::Path};

use anyhow::{ensure, Result};
use serde::Deserialize;

use crate::{input::ConfigLoadError, intermediate::{colorscheme::ColorschemeIntermediate, config::ConfigIntermediate, template::TemplateIntermediate}};

use super::{colorscheme::ColorschemeInput, template::TemplateInput};

#[derive(Debug, Deserialize)]
pub struct ConfigInput {
    pub colorschemes: HashMap<String, ColorschemeInput>,
    pub templates: Option<HashMap<String, TemplateInput>>,
}

impl ConfigInput {
    pub(crate) fn validate(
        &mut self,
        template_loc: &Path,
        output_loc: &Path,
    ) -> Result<ConfigIntermediate> {
        // Colorschemes
        ensure!(
            self.colorschemes.contains_key("default"),
            ConfigLoadError::NoDefaultFound
        );

        // Validate all colorschemes
        let colorschemes: Result<HashMap<String, ColorschemeIntermediate>> = self
            .colorschemes
            .iter_mut()
            .map(|(k, v)| Ok((k.clone(), v.validate()?)))
            .collect();
        let colorschemes = colorschemes?;

        // Templates
        let mut templates = if self.templates.is_some() {
            let templates = self.templates.as_ref().unwrap();
            let mut output: HashMap<String, TemplateIntermediate> = HashMap::new();

            // Validate all templates
            for (name, template) in templates.iter() {
                let template_intermediate =
                    template.validate(name.clone(), &colorschemes, template_loc)?;
                output.insert(name.clone(), template_intermediate);
            }

            output
        } else {
            HashMap::new()
        };

        for entry in fs::read_dir(template_loc)? {
            let entry = entry.unwrap();

            let name = entry.file_name().into_string().unwrap();

            templates
                .entry(name.clone())
                .or_insert_with(|| TemplateIntermediate::new(&name, output_loc));
        }

        Ok(ConfigIntermediate {
            colorschemes,
            templates,
        })
    }
}
