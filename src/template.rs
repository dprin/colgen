use std::{
    collections::HashMap,
    fs::{self, File},
    hash::Hash,
    io::Write,
    path::PathBuf,
    str,
};

use anyhow::{ensure, Result};
use serde::Deserialize;

// TODO: more universal implementation for most formats
//       of colors
/// For now a color is as simple as a string
#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd, Hash)]
pub struct Color(String);

/// Colorscheme object that holds all colors of a colorscheme
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Colorscheme(HashMap<String, Color>);

impl Hash for Colorscheme {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let Colorscheme(map) = self;

        for (k, v) in map.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl Eq for Colorscheme {}

/// The settings for a template file.
#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Template {
    /// The theme to use.
    pub theme: Colorscheme,

    /// The input **template file**.
    pub input: PathBuf,

    /// The output **directory**.
    pub output: PathBuf,
}

impl Template {
    // TODO: Make a better implementation
    pub fn output(&self) -> Result<()> {
        let parent = self.output.parent().unwrap();

        if !parent.exists() {
            fs::create_dir(parent)?;
        }

        ensure!(parent.is_dir(), "Output folder is not a directory!");

        let input_file = fs::read(&self.input)?;
        let mut input_file = str::from_utf8(&input_file)?.to_string();
        let Colorscheme(theme) = &self.theme;

        for (k, Color(v)) in theme.iter() {
            // replace k -> {k}
            let to_replace = format!("{{{}}}", k);

            input_file = input_file.replace(&to_replace, v);
        }

        File::create(&self.output)?.write_all(input_file.as_bytes())?;
        Ok(())
    }
}
