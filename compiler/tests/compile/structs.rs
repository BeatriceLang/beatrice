use super::compile_and_run;

#[test]
fn compiles_struct_field_access_to_executable() {
    let code = compile_and_run(
        "
        struct Point {
            x: i32,
            y: i32,
        }

        fn main() -> i32 {
            let point: Point = new Point {
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
fn compiles_function_with_struct_declared_later() {
    let code = compile_and_run(
        "
        fn x(point: Point) -> i32 {
            return point.x;
        }

        struct Point {
            x: i32,
        }

        fn main() -> i32 {
            let point: Point = new Point {
                x: 42,
            };

            return x(point);
        }
        ",
    );

    assert_eq!(code, Some(42));
}
