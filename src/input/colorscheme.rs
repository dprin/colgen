use std::collections::HashMap;

use anyhow::{Error, Result};
use serde::Deserialize;

use crate::{
    intermediate::colorscheme::{ColorschemeIntermediate, SettingsIntermediate},
    template::Color,
};

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct SettingsInput {
    inherit: Option<Vec<String>>,
    rename: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum ColorschemeValue {
    Settings(SettingsInput),
    Color(Color),
}

#[derive(Debug, Deserialize)]
pub(crate) struct ColorschemeInput(HashMap<String, ColorschemeValue>);
impl ColorschemeInput {
    pub fn validate(&mut self) -> Result<ColorschemeIntermediate> {
        // extract settings to the correct type
        let settings = if let Some(settings) = self.0.get("settings") {
            match settings {
                ColorschemeValue::Settings(s) => SettingsIntermediate {
                    inherit: s.inherit.clone().unwrap_or_default(),
                    rename: s.rename.clone().unwrap_or_default(),
                },
                _ => return Err(Error::msg("Settings does not have the correct type.")),
            }
        } else {
            SettingsIntermediate::default()
        };

        // This is why it's mutable, i don't think it's a good
        // idea to clone it because it's a big waste of memory
        self.0.remove("settings");
        let colors: Result<HashMap<String, Color>> = self
            .0
            .iter()
            .map(|(k, v)| match v {
                ColorschemeValue::Color(c) => Ok((k.clone(), c.clone())),
                _ => Err(Error::msg("Color is not the correct type.")),
            })
            .collect();
        let colors = colors?;

        Ok(ColorschemeIntermediate { settings, colors })
    }
}
