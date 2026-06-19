use core::fmt;
use std::{env::current_dir, path::PathBuf};

use anyhow::{Result, bail};
use clap::builder::PathBufValueParser;
use tap::Tap;
use walkdir::WalkDir;

use crate::build::KawaiiBuild;

impl KawaiiBuild {
    pub(super) fn collect(&mut self) -> Result<()> {
        let source_dir = current_dir().unwrap().join("src");
        let mut sources = vec![];

        if !source_dir.exists() {
            bail!("Source directory not found")
        }

        for entry in WalkDir::new(source_dir) {
            let entry_path = entry?.into_path();
            sources.push(entry_path);
        }

        self.advance_to(super::KawaiiBuildState::Compile { sources });

        Ok(())
    }
}
