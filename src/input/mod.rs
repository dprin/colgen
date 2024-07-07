use std::fmt::Display;

mod colorscheme;
pub(crate) mod config;
mod template;

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
