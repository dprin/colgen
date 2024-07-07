use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use anyhow::{ensure, Result};
use serde::Deserialize;

use crate::{
    input::{colorscheme::compilation_strategy, ConfigLoadError},
    template::{Colorscheme, Template},
};

use super::{
    colorscheme::{ColorschemeInput, ColorschemeIntermediate},
    template::{TemplateInput, TemplateIntermediate},
};

#[derive(Debug, Deserialize)]
pub struct ConfigInput {
    pub colorschemes: HashMap<String, ColorschemeInput>,
    pub templates: Option<HashMap<String, TemplateInput>>,
}

pub struct ConfigIntermediate {
    pub colorschemes: HashMap<String, ColorschemeIntermediate>,
    pub templates: HashMap<String, TemplateIntermediate>,
}

impl ConfigIntermediate {
    fn construct_colorschemes(&self) -> Result<HashMap<String, Colorscheme>> {
        let strategy = compilation_strategy(&self.colorschemes)?;
        let mut res: HashMap<String, Colorscheme> = HashMap::new();

        for name in strategy {
            let value = self.colorschemes.get(&name).unwrap().compile(&res);
            res.insert(name, value);
        }

        Ok(res)
    }

    fn construct_templates(
        &self,
        colorschemes: &HashMap<String, Colorscheme>,
        templates_loc: &PathBuf,
    ) -> HashSet<Template> {
        self.templates
            .iter()
            .map(|(name, template)| Template {
                // TODO: write this in err form instead of unwrap
                theme: colorschemes.get(&template.theme).unwrap().clone(),
                output: template.output.clone(),
                input: templates_loc.join(name),
            })
            .collect()
    }

    pub fn construct(&self, templates_loc: &PathBuf) -> Result<HashSet<Template>> {
        let colors = self.construct_colorschemes()?;
        Ok(self.construct_templates(&colors, templates_loc))
    }
}

impl ConfigInput {
    pub(crate) fn validate(
        &mut self,
        template_loc: &Path,
        output_loc: &PathBuf,
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

        for entry in fs::read_dir(&template_loc)? {
            let entry = entry.unwrap();

            let name = entry.file_name().into_string().unwrap();

            if !templates.contains_key(&name) {
                let v = TemplateIntermediate::new(&name, &output_loc);
                templates.insert(name, v);
            }
        }

        Ok(ConfigIntermediate {
            colorschemes,
            templates,
        })
    }
}
