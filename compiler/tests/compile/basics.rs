use super::{assert_returns_42, assert_returns_number};

#[test]
fn compiles_return_42_to_executable() {
    assert_returns_42(
        "
        fn main() -> i32 {
            return 42;
        }
        ",
    );
}

#[test]
fn compiles_return_binary_op_to_executable() {
    assert_returns_42(
        "
        fn main() -> i32 {
            return 40 + 2;
        }
        ",
    );
}

#[test]
fn compiles_i32_number_suffix_to_executable() {
    assert_returns_42(
        "
        fn main() -> i32 {
            return 40i32 + 2i32;
        }
        ",
    );
}

#[test]
fn compiles_u32_number_suffix_to_executable() {
    assert_returns_42(
        "
        fn main() -> i32 {
            if 40u32 + 2u32 == 42u32 {
                return 42;
            }

            return 1;
        }
        ",
    );
}

#[test]
fn compiles_bool_cast_to_i32_to_executable() {
    assert_returns_number(
        "
        fn main() -> i32 {
            return true as i32;
        }
        ",
        1,
    );
}

#[test]
fn compiles_invert_to_executable() {
    assert_returns_42(
        "
        fn main() -> i32 {
            if !false {
                if !!true {
                    return 42;
                }
            }

            return 1;
        }
        ",
    );
}

#[test]
fn compiles_unsigned_greater_than_to_executable() {
    assert_returns_42(
        "
        fn main() -> i32 {
            if 4294967295u32 > 1u32 {
                return 42;
            }

            return 1;
        }
        ",
    );
}

#[test]
fn compiles_unsigned_less_than_to_executable() {
    assert_returns_42(
        "
        fn main() -> i32 {
            if 1u32 < 4294967295u32 {
                return 42;
            }

            return 1;
        }
        ",
    );
}

#[test]
fn compiles_unsigned_divide_to_executable() {
    assert_returns_42(
        "
        fn main() -> i32 {
            if 4294967295u32 / 2u32 > 100u32 {
                return 42;
            }

            return 1;
        }
        ",
    );
}

#[test]
fn compiles_constant_literal_to_executable() {
    assert_returns_42(
        "
        const answer: i32 = 42;

        fn main() -> i32 {
            return answer;
        }
        ",
    );
}

#[test]
#[ignore = "TODO: enable once const initializers use const-eval instead of runtime codegen"]
fn compiles_constant_binary_op_to_executable() {
    assert_returns_42(
        "
        const answer: i32 = 40 + 2;

        fn main() -> i32 {
            return answer;
        }
        ",
    );
}
