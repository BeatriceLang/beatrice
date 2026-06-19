use std::{
    env::current_dir,
    path::{self, PathBuf},
};

use anyhow::{Context, Result, bail};
use xshell::{Shell, cmd};

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

            let sh = Shell::new()?;
            if cmd!(sh, "beatricec {source} -o {object}").run().is_err() {
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
