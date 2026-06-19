use std::{env::current_dir, path::PathBuf};

use anyhow::{Result, bail};
use beatricec::compile;

use crate::build::{KawaiiBuild, KawaiiBuildState};

impl KawaiiBuild {
    pub(super) fn compile(&mut self) -> Result<()> {
        let KawaiiBuildState::Compile { sources } = &self.state else {
            panic!("Unexpected kawaii build state");
        };

        let mut objects = vec![];
        let mut failed = false;

        for source in sources {
            let object = object_file_for(source.clone());

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

fn object_file_for(source: PathBuf) -> PathBuf {
    let target_dir = current_dir().unwrap().join("target");
    let source_name = source.file_name().unwrap();

    target_dir.join(source_name)
}
