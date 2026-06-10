use std::path::PathBuf;

use clap::builder::PathBufValueParser;

#[derive(clap::Parser, Debug)]
pub struct Args {
    pub input: PathBuf,
}
