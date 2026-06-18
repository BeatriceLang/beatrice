use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
pub struct Args {
    pub source_path: PathBuf,

    #[arg(short, long, default_value = "out.o")]
    pub output: PathBuf,
}
