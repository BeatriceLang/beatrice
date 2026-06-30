use super::assert_returns_42;

#[test]
fn compiles_while_loop() {
    assert_returns_42(
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
}

#[test]
fn compiles_while_loop_with_initially_false_condition() {
    assert_returns_42(
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
}

#[test]
fn compiles_var_assignment_return() {
    assert_returns_42(
        "
        fn main() -> i32 {
            var value: i32 = 1;
            value = 42;
            return value;
        }
        ",
    );
}

#[test]
fn compiles_var_assignment_from_previous_value() {
    assert_returns_42(
        "
        fn main() -> i32 {
            var value: i32 = 40;
            value = value + 2;
            return value;
        }
        ",
    );
}
