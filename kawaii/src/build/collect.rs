use std::env::current_dir;

use anyhow::{Result, bail};
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
            let entry = entry?;

            if !entry.file_type().is_file()
                || !entry
                    .clone()
                    .into_path()
                    .extension()
                    .is_some_and(|ext| ext == "bt")
            {
                continue;
            }

            sources.push(entry.into_path());
        }

        self.advance_to(super::KawaiiBuildState::Compile { sources });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::build::{KawaiiBuild, KawaiiBuildState};
    use crate::test_utils::{project, temp_test_dir, with_current_dir};

    #[test]
    fn fails_when_source_directory_is_missing() {
        let dir = temp_test_dir();

        let result = with_current_dir(&dir, || {
            let mut build = KawaiiBuild::new(project());

            build.collect()
        });

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Source directory not found"
        );

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn only_tracks_source_files() {
        let dir = temp_test_dir();
        fs::create_dir_all(dir.join("src/lib")).unwrap();
        fs::write(dir.join("src/main.bt"), "").unwrap();
        fs::write(dir.join("src/lib/value.bt"), "").unwrap();

        with_current_dir(&dir, || {
            let mut build = KawaiiBuild::new(project());

            build.collect().unwrap();

            let KawaiiBuildState::Compile { sources } = build.state else {
                panic!("expected compile state after collect");
            };

            let mut sources = sources;
            sources.sort();

            assert_eq!(
                sources,
                vec![dir.join("src/lib/value.bt"), dir.join("src/main.bt")]
            );
        });

        fs::remove_dir_all(dir).unwrap();
    }
}
