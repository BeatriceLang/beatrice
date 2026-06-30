use super::compile_and_run;

#[test]
fn compiles_address_of_local_value() {
    let code = compile_and_run(
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
fn compiles_is_nullptr_intrinsic_for_non_null_pointer() {
    let code = compile_and_run(
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

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_deref_address_of_local_value() {
    let code = compile_and_run(
        "
        fn main() -> i32 {
            let value: i32 = 42;

            return *&value;
        }
        ",
    );

    assert_eq!(code, Some(42));
}
