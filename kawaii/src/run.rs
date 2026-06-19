use std::process::{Command, exit};

use anyhow::{Result, anyhow};

use crate::{build::build, project_info::ProjectInfo, project_layout::ProjectLayout};

#[derive(clap::Args, Debug)]
pub struct RunArgs {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    inner_args: Vec<String>,
}

pub fn run(args: RunArgs) -> Result<()> {
    let project_info = ProjectInfo::from_project_toml()?;

    build(project_info.clone())?;

    let output = ProjectLayout::current()?.artifact(&project_info);

    exit(
        Command::new(output)
            .args(args.inner_args)
            .status()?
            .code()
            .ok_or_else(|| anyhow!("Failed to get exit code of program"))?,
    );
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use crate::args::{Args, Command};

    #[test]
    fn parses_trailing_program_args_after_separator() {
        let args = Args::parse_from(["kawaii", "run", "--", "hello", "-x", "--flag"]);

        let Command::Run(args) = args.command else {
            panic!("expected run command");
        };

        assert_eq!(args.inner_args, ["hello", "-x", "--flag"]);
    }

    #[test]
    fn parses_hyphenated_program_args_after_separator() {
        let args = Args::parse_from(["kawaii", "run", "--", "-x", "--flag"]);

        let Command::Run(args) = args.command else {
            panic!("expected run command");
        };

        assert_eq!(args.inner_args, ["-x", "--flag"]);
    }
}
