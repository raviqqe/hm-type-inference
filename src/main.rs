mod ast;
mod infer;
mod types;

use ast::*;
use infer::infer;
use types::*;

fn main() {
    for (expression, expected_type) in &[
        (num(42), Type::Number),
        (let_("x", num(42), var("x")), Type::Number),
        (
            let_("f", lambda("x", num(42)), app(var("f"), num(42))),
            Type::Number,
        ),
        (
            let_("f", lambda("x", var("x")), app(var("f"), num(42))),
            Type::Number,
        ),
        (
            let_(
                "f",
                lambda("x", num(42)),
                let_("y", app(var("f"), num(42)), var("f")),
            ),
            Type::Function(Type::Number.into(), Type::Number.into()),
        ),
        (
            let_(
                "f",
                lambda("x", lambda("x", num(42))),
                app(app(var("f"), num(42)), num(42)),
            ),
            Type::Number,
        ),
    ] {
        let (_, type_) = infer(&Default::default(), expression).unwrap();

        println!("{} : {}", expression, type_);
        assert_eq!(&type_, expected_type);
    }
}
