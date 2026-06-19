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
    let dir = env::temp_dir().join(format!("kawaii-test-{}-{suffix}", std::process::id()));

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

fn kawaii_build(dir: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_kawaii"))
        .arg("build")
        .current_dir(dir)
        .output()
        .unwrap()
}

#[test]
fn builds_basic_project_into_named_executable() {
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

    let build_output = kawaii_build(&dir);

    assert!(
        build_output.status.success(),
        "kawaii build failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build_output.stdout),
        String::from_utf8_lossy(&build_output.stderr)
    );

    let executable = dir.join("target").join("hello");
    assert!(executable.exists(), "kawaii did not create the executable");

    let run_output = Command::new(executable).output().unwrap();
    assert_eq!(run_output.status.code(), Some(42));

    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn builds_project_with_imported_source() {
    let dir = temp_test_dir();
    write_project(
        &dir,
        r#"
        name = "imports"
        "#,
        &[
            (
                "src/main.bt",
                r#"
                import "lib/value.bt";

                fn main() -> i32 {
                    return value();
                }
                "#,
            ),
            (
                "src/lib/value.bt",
                r#"
                fn value() -> i32 {
                    return 42;
                }
                "#,
            ),
        ],
    );

    let build_output = kawaii_build(&dir);

    assert!(
        build_output.status.success(),
        "kawaii build failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build_output.stdout),
        String::from_utf8_lossy(&build_output.stderr)
    );

    let run_output = Command::new(dir.join("target").join("imports"))
        .output()
        .unwrap();
    assert_eq!(run_output.status.code(), Some(42));

    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn passes_configured_link_args_to_linker() {
    let dir = temp_test_dir();
    write_project(
        &dir,
        r#"
        name = "link-args"

        [build]
        link-args = ["-Wl,-Map,target/link.map"]
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

    let build_output = kawaii_build(&dir);

    assert!(
        build_output.status.success(),
        "kawaii build failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build_output.stdout),
        String::from_utf8_lossy(&build_output.stderr)
    );
    assert!(dir.join("target").join("link-args").exists());
    assert!(
        dir.join("target").join("link.map").exists(),
        "linker did not produce the configured map file"
    );

    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn fails_when_source_directory_is_missing() {
    let dir = temp_test_dir();
    fs::write(
        dir.join("Kawaii.toml"),
        r#"
        name = "missing-src"
        "#,
    )
    .unwrap();

    let build_output = kawaii_build(&dir);

    assert!(
        !build_output.status.success(),
        "kawaii build succeeded without a src directory"
    );
    assert!(
        String::from_utf8_lossy(&build_output.stderr).contains("Failed to collect source files"),
        "unexpected stderr:\n{}",
        String::from_utf8_lossy(&build_output.stderr)
    );

    fs::remove_dir_all(dir).unwrap();
}
