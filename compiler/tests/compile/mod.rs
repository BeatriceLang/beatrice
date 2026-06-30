use std::{fs, path::PathBuf};

use tempfile::{NamedTempFile, tempdir};

macro_rules! shell_cmd {
    ($($cmd:tt)*) => {{
        let sh = Box::leak(Box::new(xshell::Shell::new().unwrap()));
        xshell::cmd!(sh, $($cmd)*)
    }};
}

fn compile_and_run_output(source_code: &str) -> std::process::Output {
    let source = random_file();
    let object = random_file();
    let executable = random_file();

    fs::write(&source, source_code).unwrap();

    compile_to_object(&source, &object);

    link_executable(&[object], &executable);

    shell_cmd!("{executable}").ignore_status().output().unwrap()
}

fn compile_and_run(source_code: &str) -> Option<i32> {
    compile_and_run_output(source_code).status.code()
}

fn compile_to_object(source: &PathBuf, object: &PathBuf) {
    beatrice_compiler::compile(source, object.clone()).unwrap();
}

fn link_executable(objects: &[PathBuf], executable: &PathBuf) {
    shell_cmd!("cc {objects...} -o {executable}").run().unwrap();
}

fn compile_objects_and_run(
    sources: &[(&str, &str)],
    objects_to_link: &[&str],
) -> std::process::Output {
    let dir = tempdir().unwrap();
    let executable = random_file();

    for (name, source_code) in sources {
        let path = dir.path().join(name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, source_code).unwrap();
    }

    let objects = objects_to_link
        .iter()
        .map(|name| {
            let source = dir.path().join(format!("{name}.bt"));
            let object = dir.path().join(format!("{name}.o"));

            compile_to_object(&source, &object);

            object
        })
        .collect::<Vec<_>>();

    link_executable(&objects, &executable);

    shell_cmd!("{executable}").ignore_status().output().unwrap()
}

fn random_file() -> PathBuf {
    NamedTempFile::new()
        .unwrap()
        .into_temp_path()
        .keep()
        .unwrap()
}

mod array_access;
mod basics;
mod control_flow;
mod functions;
mod imports;
mod pointers;
mod structs;
