use std::{collections::HashMap, fs, path::PathBuf, str};

use anyhow::{ensure, Result};
use serde::Deserialize;

// TODO: more universal implementation for most formats
//       of colors
/// For now a color is as simple as a string
#[derive(Debug, Deserialize, Clone)]
pub struct Color(String);

/// Colorscheme object that holds all colors of a colorscheme
#[derive(Debug, Deserialize, Clone)]
pub struct Colorscheme {
    colors: HashMap<String, Color>,
}

/// The settings for a template file.
#[derive(Debug)]
pub struct Template {
    /// The theme to use.
    pub theme: Colorscheme,

    /// The input **template file**.
    pub input: PathBuf,

    /// The output **directory**.
    pub output: PathBuf,

    /// The new file name.
    pub name: String,
}

impl Template {
    // TODO: Make a better implementation
    pub fn output(&self) -> Result<()> {
        if !self.output.exists() {
            fs::create_dir(&self.output)?;
        }

        ensure!(self.output.is_dir(), "Output is not a directory!");

        let input_file = fs::read(&self.input)?;
        let mut input_file = str::from_utf8(&input_file)?.to_string();

        for (k, v) in self.theme.colors.iter() {
            // replace k -> {k}
            let to_replace = format!("{{{}}}", k);
            let Color(replace_with) = v;

            input_file = input_file.replace(&to_replace, replace_with);
        }

        fs::write(self.output.join(&self.name), input_file)?;

        Ok(())
    }
}
