use std::{
    env::current_dir,
    hash::{DefaultHasher, Hash, Hasher},
    path::{self, Path, PathBuf},
};

use anyhow::{Result, bail};
use beatrice_compiler::compile;

use crate::build::{KawaiiBuild, KawaiiBuildState};

impl KawaiiBuild {
    pub(super) fn compile(&mut self) -> Result<()> {
        let KawaiiBuildState::Compile { sources } = &self.state else {
            panic!("Unexpected kawaii build state");
        };

        let mut objects = vec![];
        let mut failed = false;

        for source in sources {
            let object = object_file_for(source);

            if let Err(err) = compile(source, object.clone()) {
                eprintln!("Failed to compile {source:?}: {err:#}");
                failed = true;
            }

            objects.push(object);
        }

        if failed {
            bail!("Compile failed");
        }

        self.advance_to(KawaiiBuildState::Link { objects });

        Ok(())
    }
}

fn object_file_for(source: &Path) -> PathBuf {
    let object_dir = current_dir().unwrap().join("target").join("objects");
    let source_name = source.file_name().unwrap().to_str().unwrap();
    let hash = hash_path(source);

    object_dir.join(format!("{source_name}-{hash:016x}"))
}

fn hash_path(path: &Path) -> u64 {
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::build::{
        KawaiiBuild, KawaiiBuildState,
        test_support::{project, temp_test_dir, with_current_dir},
    };

    #[test]
    fn writes_objects_under_target_with_object_extension() {
        let dir = temp_test_dir();
        fs::create_dir_all(dir.join("src")).unwrap();
        fs::write(
            dir.join("src/main.bt"),
            r#"
            fn main() -> i32 {
                return 42;
            }
            "#,
        )
        .unwrap();

        with_current_dir(&dir, || {
            let mut build = KawaiiBuild {
                state: KawaiiBuildState::Compile {
                    sources: vec![dir.join("src/main.bt")],
                },
                project: project(),
            };

            build.compile().unwrap();

            let KawaiiBuildState::Link { objects } = build.state else {
                panic!("expected link state after compile");
            };

            assert_eq!(objects, vec![dir.join("target/main.o")]);
            assert!(dir.join("target/main.o").exists());
        });

        fs::remove_dir_all(dir).unwrap();
    }
}
