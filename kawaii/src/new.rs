use std::{env::current_dir, fs};

use anyhow::Result;

const GITIGNORE: &str = "/target";
const MAIN: &str = r#"
fn main() -> i32 {
    return 0;
}
"#;

#[derive(clap::Args, Debug)]
pub struct NewArgs {
    name: String,
}

pub fn new(args: NewArgs) -> Result<()> {
    let project_name = args.name;

    let dir = current_dir()?.join(&project_name);

    fs::create_dir(&dir)?;

    let src = dir.join("src");
    let target = dir.join("target");

    fs::create_dir(&src)?;
    fs::create_dir(&target)?;

    let gitignore = dir.join(".gitignore");
    let main = src.join("main.bt");
    let kawaii_toml = dir.join("Kawaii.toml");

    fs::write(gitignore, GITIGNORE)?;
    fs::write(main, MAIN)?;
    fs::write(
        kawaii_toml,
        format!(
            r#"
                name = "{project_name}"
            "#
        ),
    )?;

    Ok(())
}
