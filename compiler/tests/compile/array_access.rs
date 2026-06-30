use super::compile_and_run;

#[test]
fn compiles_array_access_on_local_array_to_executable() {
    let code = compile_and_run(
        "
        fn main() -> i32 {
            let values: [i32; 3] = [1, 42, 3];

            return values[1];
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_array_access_on_array_literal_to_executable() {
    let code = compile_and_run(
        "
        fn main() -> i32 {
            return [1, 42, 3][1];
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_array_access_with_expression_index_to_executable() {
    let code = compile_and_run(
        "
        fn main() -> i32 {
            let values: [i32; 4] = [1, 2, 42, 4];

            return values[1 + 1];
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_array_access_with_runtime_index_to_executable() {
    let code = compile_and_run(
        "
        fn pick(index: i32) -> i32 {
            let values: [i32; 4] = [1, 2, 42, 4];

            return values[index];
        }

        fn main() -> i32 {
            return pick(2);
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_nested_array_access_to_executable() {
    let code = compile_and_run(
        "
        fn main() -> i32 {
            let matrix: [[i32; 2]; 2] = [[1, 2], [42, 4]];
            let row: [i32; 2] = matrix[1];

            return row[0];
        }
        ",
    );

    assert_eq!(code, Some(42));
}

#[test]
fn compiles_array_access_with_nested_index_to_executable() {
    let code = compile_and_run(
        "
        fn main() -> i32 {
            let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 42, 6]];

            return matrix[1][1];
        }
        ",
    );

    assert_eq!(code, Some(42));
}
