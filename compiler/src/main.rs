use anyhow::Result;
use clap::Parser;

mod cli_args;

use crate::cli_args::Args;

fn main() -> Result<()> {
    let args = Args::parse();

    beatrice_compiler::compile(&args.source_path, args.output)
}
