use std::{
    collections::HashMap,
    fs::{self, File},
    hash::Hash,
    io::Write,
    path::PathBuf,
    str,
};

use anyhow::{ensure, Error, Result};
use serde::Deserialize;

// TODO: more universal implementation for most formats
//       of colors
/// For now a color is as simple as a string
#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd, Hash)]
pub struct Color(String);

/// Colorscheme object that holds all colors of a colorscheme
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Colorscheme(pub HashMap<String, Color>);

impl Hash for Colorscheme {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let Colorscheme(map) = self;

        for (k, v) in map.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl Colorscheme {
    pub fn new() -> Self {
        Colorscheme(HashMap::new())
    }

    fn inherit(&mut self, other: &Self) {
        for (key, value) in &other.0 {
            self.0.insert(key.clone(), value.clone());
        }
    }

    fn rename(&mut self, from: &String, to: &String) -> Result<()> {
        if !self.0.contains_key(from) {
            return Err(Error::msg(format!(
                "Couldn't find {from} when renaming to {to}."
            )));
        }

        let v = self.0.remove(from).unwrap();
        self.0.insert(to.clone(), v);

        Ok(())
    }

    pub fn rename_all(&mut self, variables: &HashMap<String, String>) -> &mut Self {
        for (from, to) in variables {
            self.rename(from, to).unwrap();
        }

        self
    }

    pub fn inherit_all(
        &mut self,
        to_inherit: &Vec<String>,
        current_state: &HashMap<String, Self>,
    ) -> &mut Self {
        for dependency in to_inherit {
            let dependency = current_state.get(dependency).unwrap();
            self.inherit(dependency);
        }

        self
    }

    pub fn insert(&mut self, map: &HashMap<String, Color>) -> &mut Self {
        for (k, v) in map {
            self.0.insert(k.clone(), v.clone());
        }

        self
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

    /// The output **file**.
    pub output: PathBuf,
}

impl Template {
    pub fn output(&self) -> Result<()> {
        let parent = self.output.parent().unwrap();

        if !parent.exists() {
            fs::create_dir(parent)?;
        }

        ensure!(parent.is_dir(), "Output folder is not a directory!");

        let input_file = fs::read(&self.input)?;
        let input_file = str::from_utf8(&input_file)?.to_string();
        let input_file = self.insert_colors(input_file);

        File::create(&self.output)?.write_all(input_file.as_bytes())?;
        Ok(())
    }

    pub fn insert_colors(&self, input: String) -> String {
        let Colorscheme(theme) = &self.theme;
        let mut input = input.clone();

        for (k, Color(v)) in theme.iter() {
            // replace {k} -> v
            let to_replace = format!("{{{}}}", k);
            input = input.replace(&to_replace, v);
        }

        input
    }
}
