use crate::{new::NewArgs, run::RunArgs};

#[derive(clap::Parser, Debug)]
#[command(about = "Beatrice's package manager and build system")]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    Build,
    New(NewArgs),
    Run(RunArgs),
}
