use std::collections::HashMap;

use anyhow::{Error, Result};
use serde::Deserialize;

use crate::template::{Color, Colorscheme};

#[derive(Debug, Deserialize, Clone, Default)]
pub(crate) struct SettingsInput {
    inherits: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) enum ColorschemeValue {
    Settings(SettingsInput),
    Color(Color),
}

#[derive(Debug, Deserialize)]
pub(crate) struct ColorschemeInput(HashMap<String, ColorschemeValue>);

#[derive(Debug, Deserialize)]
pub(crate) struct ColorschemeIntermediate {
    settings: SettingsInput,
    colors: HashMap<String, Color>,
}

impl ColorschemeIntermediate {
    pub(crate) fn compile(&self, current_state: &HashMap<String, Colorscheme>) -> Colorscheme {
        let mut colorscheme = Colorscheme::new();

        for dependency in &self.settings.inherits {
            let dependency = current_state.get(dependency).unwrap();
            colorscheme.inherit(dependency)
        }

        colorscheme.inherit(&Colorscheme(self.colors.clone()));
        colorscheme
    }
}

impl ColorschemeInput {
    pub fn validate(&mut self) -> Result<ColorschemeIntermediate> {
        let settings = if let Some(setting) = self.0.get("settings") {
            if let ColorschemeValue::Settings(v) = setting {
                v.clone()
            } else {
                return Err(Error::msg("Settings does not have the correct type."));
            }
        } else {
            SettingsInput::default()
        };

        // This is why it's mutable, i don't think it's a good
        // idea to clone it because it's a big waste of memory
        self.0.remove("settings");
        let colors: Result<HashMap<String, Color>> = self
            .0
            .iter()
            .map(|(k, v)| match v {
                ColorschemeValue::Color(c) => Ok((k.clone(), c.clone())),
                _ => return Err(Error::msg("Color is not the correct type.")),
            })
            .collect();
        let colors = colors?;

        Ok(ColorschemeIntermediate { settings, colors })
    }
}

/// This function takes in all of the colorschemes that have been
/// inputted and creates a list of colorschemes that should be compiled
/// in order so that everything works.
///
/// If there is a cyclic dependency (meaning that there is no topological
/// ordering in the dependencies) then an error will be thrown
pub fn compilation_strategy(
    colorschemes: &HashMap<String, ColorschemeIntermediate>,
) -> Result<Vec<String>> {
    // TODO: write how to sort it
    #[derive(PartialEq, PartialOrd, Ord, Eq, Clone)]
    struct Intermediate<'a> {
        name: &'a String,
        dependencies: Vec<String>,
    }

    let mut order: Vec<String> = Vec::with_capacity(colorschemes.len());
    let mut pending_colors: Vec<Intermediate> = colorschemes
        .iter()
        .map(|(name, colorscheme)| Intermediate {
            name,
            dependencies: colorscheme.settings.inherits.clone(),
        })
        .collect();

    while !pending_colors.is_empty() {
        pending_colors.sort();
        let first = &pending_colors[0].clone();

        if !first.dependencies.is_empty() {
            let dependent = &first.dependencies[0];
            let name = first.name;
            let msg = format!("Cyclic dependency found: {} with {}", dependent, name);

            return Err(Error::msg(msg));
        }

        order.push(first.name.clone());
        pending_colors.remove(0);
        pending_colors
            .iter_mut()
            .for_each(|item| item.dependencies.retain(|x| x != first.name));
    }

    Ok(order)
}
