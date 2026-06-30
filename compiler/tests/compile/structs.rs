use super::assert_returns_42;

#[test]
fn compiles_struct_field_access_to_executable() {
    assert_returns_42(
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
}

#[test]
fn compiles_function_with_struct_declared_later() {
    assert_returns_42(
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
}
