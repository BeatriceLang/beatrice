use std::{
    fs,
    path::PathBuf,
    process::Command,
};

fn temp_test_dir() -> tempfile::TempDir {
    tempfile::tempdir().expect("failed to create temp test dir")
}

fn compile_and_run_output(test_name: &str, source_code: &str) -> std::process::Output {
    let dir = temp_test_dir();
    let source = dir.path().join(format!("{test_name}.bt"));
    let object = dir.path().join(format!("{test_name}.o"));
    let executable = dir.path().join(test_name);

    fs::write(&source, source_code).unwrap();

    compile_to_object(&source, &object);

    link_executable(&[object], &executable);

    let output = Command::new(&executable).output().unwrap();

    output
}

fn compile_and_run(test_name: &str, source_code: &str) -> Option<i32> {
    compile_and_run_output(test_name, source_code).status.code()
}

fn compile_to_object(source: &PathBuf, object: &PathBuf) {
    let compile_result = beatrice_compiler::compile(source, object.clone());

    assert!(
        compile_result.is_ok(),
        "compiler failed\nerror:\n{:#}",
        compile_result.unwrap_err()
    );

    assert!(object.exists(), "compiler did not create object file");
}

fn link_executable(objects: &[PathBuf], executable: &PathBuf) {
    let mut command = Command::new("cc");
    command.args(objects).arg("-o").arg(executable);

    let linker_output = command.output().unwrap();

    assert!(
        linker_output.status.success(),
        "linker failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&linker_output.stdout),
        String::from_utf8_lossy(&linker_output.stderr)
    );
}

fn compile_objects_and_run(
    test_name: &str,
    sources: &[(&str, &str)],
    objects_to_link: &[&str],
) -> std::process::Output {
    let dir = temp_test_dir();
    let executable = dir.path().join(test_name);

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

    let output = Command::new(&executable).output().unwrap();

    output
}

mod array_access;
mod basics;
mod control_flow;
mod functions;
mod imports;
mod pointers;
mod structs;
