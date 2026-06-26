use std::{
    env, fs, io,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn temp_test_dir() -> PathBuf {
    for attempt in 0..100 {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = env::temp_dir().join(format!(
            "beatrice-test-{}-{suffix}-{attempt}",
            std::process::id()
        ));

        match fs::create_dir(&dir) {
            Ok(()) => return dir,
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(err) => panic!("failed to create temp test dir: {err}"),
        }
    }

    panic!("failed to create unique temp test dir");
}

fn compile_and_run_output(test_name: &str, source_code: &str) -> std::process::Output {
    let dir = temp_test_dir();
    let source = dir.join(format!("{test_name}.bt"));
    let object = dir.join(format!("{test_name}.o"));
    let executable = dir.join(test_name);

    fs::write(&source, source_code).unwrap();

    compile_to_object(&source, &object);

    link_executable(&[object], &executable);

    let output = Command::new(&executable).output().unwrap();

    fs::remove_dir_all(dir).unwrap();

    output
}

fn compile_and_run(test_name: &str, source_code: &str) -> Option<i32> {
    compile_and_run_output(test_name, source_code).status.code()
}

fn compile_to_object(source: &PathBuf, object: &PathBuf) {
    let compiler_output = Command::new(env!("CARGO_BIN_EXE_beatricec"))
        .arg(source)
        .arg("-o")
        .arg(object)
        .output()
        .unwrap();

    assert!(
        compiler_output.status.success(),
        "compiler failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&compiler_output.stdout),
        String::from_utf8_lossy(&compiler_output.stderr)
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
    let executable = dir.join(test_name);

    for (name, source_code) in sources {
        let path = dir.join(name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, source_code).unwrap();
    }

    let objects = objects_to_link
        .iter()
        .map(|name| {
            let source = dir.join(format!("{name}.bt"));
            let object = dir.join(format!("{name}.o"));

            compile_to_object(&source, &object);

            object
        })
        .collect::<Vec<_>>();

    link_executable(&objects, &executable);

    let output = Command::new(&executable).output().unwrap();

    fs::remove_dir_all(dir).unwrap();

    output
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
fn compiles_return_binary_op_to_executable() {
    let code = compile_and_run(
        "return_binary_op",
        "
        fn main() -> i32 {
            return 40 + 2;
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_i32_number_suffix_to_executable() {
    let code = compile_and_run(
        "i32_number_suffix",
        "
        fn main() -> i32 {
            return 40i32 + 2i32;
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_u32_number_suffix_to_executable() {
    let code = compile_and_run(
        "u32_number_suffix",
        "
        fn main() -> i32 {
            if 40u32 + 2u32 == 42u32 {
                return 42;
            }

            return 1;
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_unsigned_greater_than_to_executable() {
    let code = compile_and_run(
        "unsigned_greater_than",
        "
        fn main() -> i32 {
            if 4294967295u32 > 1u32 {
                return 42;
            }

            return 1;
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_unsigned_less_than_to_executable() {
    let code = compile_and_run(
        "unsigned_less_than",
        "
        fn main() -> i32 {
            if 1u32 < 4294967295u32 {
                return 42;
            }

            return 1;
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_unsigned_divide_to_executable() {
    let code = compile_and_run(
        "unsigned_divide",
        "
        fn main() -> i32 {
            let half: u32 = 4294967295u32 / 2u32;

            if half > 100u32 {
                return 42;
            }

            return 1;
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_constant_literal_to_executable() {
    let code = compile_and_run(
        "constant_literal",
        "
        const answer: i32 = 42

        fn main() -> i32 {
            return answer;
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
#[ignore = "TODO: enable once const initializers use const-eval instead of runtime codegen"]
fn compiles_constant_binary_op_to_executable() {
    let code = compile_and_run(
        "constant_binary_op",
        "
        const answer: i32 = 40 + 2

        fn main() -> i32 {
            return answer;
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_return_function_call_to_executable() {
    let code = compile_and_run(
        "return_function_call",
        "
        fn test() -> i32 {
            return 42;
        }

        fn main() -> i32 {
            return test();
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_function_params_as_idents() {
    let code = compile_and_run(
        "function_params_as_idents",
        "
        fn add(lhs: i32, rhs: i32) -> i32 {
            return lhs + rhs;
        }

        fn main() -> i32 {
            return add(40, 2);
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_recursive_fibonacci_to_executable() {
    let code = compile_and_run(
        "recursive_fibonacci",
        "
        fn fib(n: i32) -> i32 {
            if n < 2 {
                return n;
            }

            return fib(n - 1) + fib(n - 2);
        }

        fn main() -> i32 {
            return fib(10);
        }
        ",
    );

    assert_eq!(code, Some(55));
}

#[test]
fn compiles_let_value_return() {
    let code = compile_and_run(
        "let_return",
        "
        fn main() -> i32 {
            let hello: i32 = 40;
            return hello;
        }
        ",
    );

    assert_eq!(code, Some(40));
}

#[test]
fn compiles_struct_field_access_to_executable() {
    let code = compile_and_run(
        "struct_field_access",
        "
        struct Point {
            x: i32,
            y: i32,
        }

        fn main() -> i32 {
            let point: Point = Point {
                x: 40,
                y: 2,
            };

            return point.x + point.y;
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_address_of_local_value() {
    let code = compile_and_run(
        "address_of_local_value",
        "
        fn main() -> i32 {
            let value: i32 = 42;
            let ptr: *i32 = &value;

            return *ptr;
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_deref_address_of_local_value() {
    let code = compile_and_run(
        "deref_address_of_local_value",
        "
        fn main() -> i32 {
            let value: i32 = 42;

            return *&value;
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_var_assignment_return() {
    let code = compile_and_run(
        "var_assignment_return",
        "
        fn main() -> i32 {
            var value: i32 = 1;
            value = 42;
            return value;
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_var_assignment_from_previous_value() {
    let code = compile_and_run(
        "var_assignment_from_previous_value",
        "
        fn main() -> i32 {
            var value: i32 = 40;
            value = value + 2;
            return value;
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_while_loop() {
    let code = compile_and_run(
        "while_loop",
        "
        fn main() -> i32 {
            var value: i32 = 0;

            while value < 42 {
                value = value + 1;
            }

            return value;
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_while_loop_with_initially_false_condition() {
    let code = compile_and_run(
        "while_loop_initially_false",
        "
        fn main() -> i32 {
            var value: i32 = 42;

            while value < 42 {
                value = 0;
            }

            return value;
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_puts_hello_world() {
    let output = compile_and_run_output(
        "puts_hello_world",
        r#"
        extern fn puts(value: string) -> i32;

        fn main() -> i32 {
            puts("Hello world!");
            return 0;
        }
        "#,
    );

    assert!(
        output.status.success(),
        "executable failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello world!\n");
}

#[test]
fn compiles_function_without_return_value() {
    let output = compile_and_run_output(
        "function_without_return_value",
        r#"
        extern fn puts(value: string) -> i32;

        fn say_hello() {
            puts("Hello void!");
        }

        fn main() -> i32 {
            say_hello();
            return 0;
        }
        "#,
    );

    assert!(
        output.status.success(),
        "executable failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello void!\n");
}

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
