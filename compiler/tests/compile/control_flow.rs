use super::compile_and_run;

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
