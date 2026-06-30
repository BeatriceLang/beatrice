use super::{assert_returns_42, assert_returns_number, compile_and_run_output};

#[test]
fn compiles_return_function_call_to_executable() {
    assert_returns_42(
        "
        fn test() -> i32 {
            return 42;
        }

        fn main() -> i32 {
            return test();
        }
        ",
    );
}

#[test]
fn compiles_function_params_as_idents() {
    assert_returns_42(
        "
        fn add(lhs: i32, rhs: i32) -> i32 {
            return lhs + rhs;
        }

        fn main() -> i32 {
            return add(40, 2);
        }
        ",
    );
}

#[test]
fn compiles_recursive_fibonacci_to_executable() {
    assert_returns_number(
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
        55,
    );
}

#[test]
fn compiles_let_value_return() {
    assert_returns_42(
        "
        fn main() -> i32 {
            let hello: i32 = 42;
            return hello;
        }
        ",
    );
}

#[test]
fn compiles_puts_hello_world() {
    let output = compile_and_run_output(
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
fn compiles_return_without_value() {
    let output = compile_and_run_output(
        r#"
        extern fn puts(value: string) -> i32;

        fn say_hello() {
            puts("before return");
            return;
            puts("after return");
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
    assert_eq!(String::from_utf8_lossy(&output.stdout), "before return\n");
}
