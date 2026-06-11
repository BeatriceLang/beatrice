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

fn compile_and_run(test_name: &str, source_code: &str) -> Option<i32> {
    let dir = temp_test_dir();
    let source = dir.join(format!("{test_name}.bt"));
    let object = dir.join(format!("{test_name}.o"));
    let executable = dir.join(test_name);

    fs::write(&source, source_code).unwrap();

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
    let code = status.code();

    fs::remove_dir_all(dir).unwrap();

    code
}

#[test]
fn compiles_return_42_to_executable() {
    let code = compile_and_run(
        "return_42",
        "
        fn main() -> i32 {
            return 42;
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_return_math_op_to_executable() {
    let code = compile_and_run(
        "return_math_op",
        "
        fn main() -> i32 {
            return 40 + 2;
        }
        ",
    );

    assert_eq!(code, Some(42));
}
