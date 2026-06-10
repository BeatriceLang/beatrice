use std::{
    env, fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn temp_test_dir() -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = env::temp_dir().join(format!("beatrice-test-{}-{suffix}", std::process::id()));

    fs::create_dir(&dir).unwrap();

    dir
}

#[test]
fn compiles_return_42_to_executable() {
    let dir = temp_test_dir();
    let source = dir.join("return_42.bt");
    let object = dir.join("return_42.o");
    let executable = dir.join("return_42");

    fs::write(
        &source,
        "
        fn main() -> i32 {
            return 42;
        }
        ",
    )
    .unwrap();

    let compiler_output = Command::new(env!("CARGO_BIN_EXE_beatrice"))
        .arg(&source)
        .arg("-o")
        .arg(&object)
        .output()
        .unwrap();

    assert!(
        compiler_output.status.success(),
        "compiler failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&compiler_output.stdout),
        String::from_utf8_lossy(&compiler_output.stderr)
    );

    assert!(object.exists(), "compiler did not create object file");

    let linker_output = Command::new("cc")
        .arg(&object)
        .arg("-o")
        .arg(&executable)
        .output()
        .unwrap();

    assert!(
        linker_output.status.success(),
        "linker failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&linker_output.stdout),
        String::from_utf8_lossy(&linker_output.stderr)
    );

    let status = Command::new(&executable).status().unwrap();

    assert_eq!(status.code(), Some(42));

    fs::remove_dir_all(dir).unwrap();
}
