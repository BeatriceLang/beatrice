use std::fs;

use anyhow::{Context, Result};

#[derive(serde::Deserialize, Debug)]
pub struct ProjectInfo {
    pub name: String,

    #[serde(default)]
    pub freestanding: bool,
}

impl ProjectInfo {
    pub fn from_project_toml() -> Result<Self> {
        let project_toml = fs::read_to_string("Kawaii.toml")
            .context("Failed to read project info (Kawaii.toml)")?;

        toml::from_str(&project_toml).context("Failed to parse project info")
    }
}
