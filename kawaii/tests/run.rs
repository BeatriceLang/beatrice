use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

fn temp_test_dir() -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = env::temp_dir().join(format!("kawaii-run-test-{}-{suffix}", std::process::id()));

    fs::create_dir(&dir).unwrap();

    dir
}

fn write_project(dir: &Path, project_toml: &str, sources: &[(&str, &str)]) {
    fs::write(dir.join("Kawaii.toml"), project_toml).unwrap();

    for (path, source) in sources {
        let path = dir.join(path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, source).unwrap();
    }
}

fn kawaii_run(dir: &Path) -> Output {
    kawaii_run_with_args(dir, &[])
}

fn kawaii_run_with_args(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_kawaii"))
        .arg("run")
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap()
}

#[test]
fn returns_program_exit_code() {
    let dir = temp_test_dir();
    write_project(
        &dir,
        r#"
        name = "hello"
        "#,
        &[(
            "src/main.bt",
            r#"
            fn main() -> i32 {
                return 42;
            }
            "#,
        )],
    );

    let output = kawaii_run(&dir);

    assert!(
        output.status.code() == Some(42),
        "kawaii run did not return the program's exit code\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(dir.join("target").join("hello").exists());

    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn accepts_hyphenated_program_args_after_separator() {
    let dir = temp_test_dir();
    write_project(
        &dir,
        r#"
        name = "args"
        "#,
        &[(
            "src/main.bt",
            r"
            fn main() -> i32 {
                return 0;
            }
            ",
        )],
    );

    let output = kawaii_run_with_args(&dir, &["--", "-x", "--flag"]);

    assert!(
        output.status.success(),
        "kawaii run rejected hyphenated program args\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn forwards_program_stdout() {
    let dir = temp_test_dir();
    write_project(
        &dir,
        r#"
        name = "prints"
        "#,
        &[(
            "src/main.bt",
            r#"
            extern fn puts(value: string) -> i32;

            fn main() -> i32 {
                puts("Hello run!");
                return 0;
            }
            "#,
        )],
    );

    let output = kawaii_run(&dir);

    assert!(
        output.status.success(),
        "kawaii run failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("Hello run!\n"),
        "kawaii run did not forward program stdout\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn fails_when_project_info_is_missing() {
    let dir = temp_test_dir();

    let output = kawaii_run(&dir);

    assert!(
        !output.status.success(),
        "kawaii run succeeded without Kawaii.toml"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("Failed to read project info"),
        "unexpected stderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    fs::remove_dir_all(dir).unwrap();
}
