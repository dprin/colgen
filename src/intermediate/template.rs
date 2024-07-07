use std::path::{Path, PathBuf};

pub(crate) struct TemplateIntermediate {
    pub(crate) theme: String,
    pub(crate) output: PathBuf,
}

impl TemplateIntermediate {
    pub fn new(filename: &String, output: &Path) -> Self {
        Self {
            theme: "default".to_string(),
            output: output.join(filename),
        }
    }
}
