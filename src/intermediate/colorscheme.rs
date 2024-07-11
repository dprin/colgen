use std::collections::HashMap;

use anyhow::{Error, Result};

use crate::template::{Color, Colorscheme};

#[derive(Debug)]
pub(crate) struct ColorschemeIntermediate {
    pub(crate) settings: SettingsIntermediate,
    pub(crate) colors: HashMap<String, Color>,
}

#[derive(Debug, Default)]
pub(crate) struct SettingsIntermediate {
    pub(crate) inherit: Vec<String>,
    pub(crate) rename: HashMap<String, String>,
}

impl ColorschemeIntermediate {
    pub(crate) fn compile(&self, current_state: &HashMap<String, Colorscheme>) -> Colorscheme {
        let mut colorscheme = Colorscheme::new();

        colorscheme
            .inherit_all(&self.settings.inherit, current_state)
            .rename_all(&self.settings.rename)
            .insert(&self.colors);

        colorscheme
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
    #[derive(Clone)]
    struct Intermediate<'a> {
        name: &'a String,
        dependencies: Vec<String>,
    }

    impl PartialEq for Intermediate<'_> {
        fn eq(&self, other: &Self) -> bool {
            self.dependencies.len() == other.dependencies.len()
        }
    }

    impl Eq for Intermediate<'_> {}

    impl PartialOrd for Intermediate<'_> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Intermediate<'_> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.dependencies.len().cmp(&other.dependencies.len())
        }
    }

    let mut order: Vec<String> = Vec::with_capacity(colorschemes.len());
    let mut pending_colors: Vec<Intermediate> = colorschemes
        .iter()
        .map(|(name, colorscheme)| Intermediate {
            name,
            dependencies: colorscheme.settings.inherit.clone(),
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
