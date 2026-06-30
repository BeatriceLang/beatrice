use super::compile_objects_and_run;

#[test]
fn compiles_imported_function_call_to_executable() {
    let output = compile_objects_and_run(
        "imported_function_call",
        &[
            (
                "main.bt",
                r#"
                import "imported.bt";

                fn main() -> i32 {
                    return imported_value();
                }
                "#,
            ),
            (
                "imported.bt",
                r#"
                fn imported_value() -> i32 {
                    return 42;
                }
                "#,
            ),
        ],
        &["main", "imported"],
    );

    assert_eq!(output.status.code(), Some(42));
}

#[test]
fn resolves_imports_relative_to_importing_file() {
    let output = compile_objects_and_run(
        "relative_imports",
        &[
            (
                "src/main.bt",
                r#"
                import "lib/first.bt";

                fn main() -> i32 {
                    return nested_value();
                }
                "#,
            ),
            (
                "src/lib/first.bt",
                r#"
                import "second.bt";
                "#,
            ),
            (
                "src/lib/second.bt",
                r#"
                fn nested_value() -> i32 {
                    return 42;
                }
                "#,
            ),
        ],
        &["src/main", "src/lib/second"],
    );

    assert_eq!(output.status.code(), Some(42));
}

#[test]
fn compiles_nested_imported_function_call_to_executable() {
    let output = compile_objects_and_run(
        "nested_imported_function_call",
        &[
            (
                "main.bt",
                r#"
                import "first.bt";

                fn main() -> i32 {
                    return nested_value();
                }
                "#,
            ),
            (
                "first.bt",
                r#"
                import "second.bt";
                "#,
            ),
            (
                "second.bt",
                r#"
                fn nested_value() -> i32 {
                    return 42;
                }
                "#,
            ),
        ],
        &["main", "second"],
    );

    assert_eq!(output.status.code(), Some(42));
}

#[test]
fn compiles_imported_extern_function_call_to_executable() {
    let output = compile_objects_and_run(
        "imported_extern_function_call",
        &[
            (
                "main.bt",
                r#"
                import "stdio.bt";

                fn main() -> i32 {
                    puts("Hello import!");
                    return 0;
                }
                "#,
            ),
            (
                "stdio.bt",
                r#"
                extern fn puts(value: string) -> i32;
                "#,
            ),
        ],
        &["main"],
    );

    assert!(
        output.status.success(),
        "executable failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello import!\n");
}
