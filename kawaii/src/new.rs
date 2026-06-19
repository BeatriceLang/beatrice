use std::{env::current_dir, fs};

use anyhow::Result;

const GITIGNORE: &str = "/target";
const MAIN: &str = r"fn main() -> i32 {
    return 0;
}
";

#[derive(clap::Args, Debug)]
pub struct NewArgs {
    pub(super) name: String,
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
    fs::write(kawaii_toml, format!("name = \"{project_name}\"\n"))?;

    println!("Created new kawaii project {project_name}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{NewArgs, new};
    use crate::test_utils::{temp_test_dir, with_current_dir};

    #[test]
    fn creates_project_scaffold() {
        let dir = temp_test_dir();

        with_current_dir(&dir, || {
            new(NewArgs {
                name: "hello".to_string(),
            })
            .unwrap();

            let project_dir = dir.join("hello");

            assert!(project_dir.is_dir());
            assert!(project_dir.join("src").is_dir());
            assert!(project_dir.join("target").is_dir());
            assert_eq!(
                fs::read_to_string(project_dir.join(".gitignore")).unwrap(),
                "/target"
            );
            assert_eq!(
                fs::read_to_string(project_dir.join("src/main.bt")).unwrap(),
                r"fn main() -> i32 {
    return 0;
}
"
            );
            assert_eq!(
                fs::read_to_string(project_dir.join("Kawaii.toml")).unwrap(),
                "name = \"hello\"\n"
            );
        });

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn fails_when_project_directory_already_exists() {
        let dir = temp_test_dir();
        fs::create_dir(dir.join("hello")).unwrap();

        let result = with_current_dir(&dir, || {
            new(NewArgs {
                name: "hello".to_string(),
            })
        });

        assert!(result.is_err());

        fs::remove_dir_all(dir).unwrap();
    }
}
