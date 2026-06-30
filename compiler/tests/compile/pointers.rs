use super::assert_returns_42;

#[test]
fn compiles_address_of_local_value() {
    assert_returns_42(
        "
        fn main() -> i32 {
            let value: i32 = 42;
            let ptr: *i32 = &value;

            return *ptr;
        }
        ",
    );
}

#[test]
fn compiles_is_nullptr_intrinsic_for_non_null_pointer() {
    assert_returns_42(
        "
        fn main() -> i32 {
            let value: i32 = 42;

            if _is_nullptr(&value) {
                return 1;
            }

            return 42;
        }
        ",
    );
}

#[test]
fn compiles_deref_address_of_local_value() {
    assert_returns_42(
        "
        fn main() -> i32 {
            let value: i32 = 42;

            return *&value;
        }
        ",
    );
}
