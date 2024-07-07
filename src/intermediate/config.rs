use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use anyhow::Result;

use crate::{
    intermediate::colorscheme::compilation_strategy,
    template::{Colorscheme, Template},
};

use super::{colorscheme::ColorschemeIntermediate, template::TemplateIntermediate};

pub struct ConfigIntermediate {
    pub colorschemes: HashMap<String, ColorschemeIntermediate>,
    pub templates: HashMap<String, TemplateIntermediate>,
}

impl ConfigIntermediate {
    fn construct_colorschemes(&self) -> Result<HashMap<String, Colorscheme>> {
        let strategy = compilation_strategy(&self.colorschemes)?;
        let mut res: HashMap<String, Colorscheme> = HashMap::new();

        dbg!(&strategy);

        for name in strategy {
            let value = self.colorschemes.get(&name).unwrap().compile(&res);
            dbg!(&value);
            res.insert(name, value);
        }

        Ok(res)
    }

    fn construct_templates(
        &self,
        colorschemes: &HashMap<String, Colorscheme>,
        templates_loc: &Path,
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

    pub fn construct(&self, templates_loc: &Path) -> Result<HashSet<Template>> {
        let colors = self.construct_colorschemes()?;
        Ok(self.construct_templates(&colors, templates_loc))
    }
}
