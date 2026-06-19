#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use anyhow::Result;
use clap::Parser;

mod cli_args;

use crate::cli_args::Args;

fn main() -> Result<()> {
    let args = Args::parse();

    beatricec::compile(&args.source_path, args.output)
}
