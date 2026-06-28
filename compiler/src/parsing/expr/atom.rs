use chumsky::{Parser, primitive::choice, select};

use crate::{
    ast::{expression::Expression, ty::Type},
    lexing::token::Token,
    parsing::{
        expr::{
            addr_of::addr_of_expr, array_access::array_access, cast::cast,
            create_array::create_array, create_struct::create_struct, deref_expr,
            field_access::field_access, function_call::function_call_expr, invert::invert,
        },
        ident::ident,
    },
};

pub fn atom_expr<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    let base = select! {
        Token::Number(value) => Expression::Number(value),
        Token::StringLiteral(string) => Expression::StringLiteral(string),
        Token::I32Number(value) => Expression::TypedNumber { value, ty: Type::I32 },
        Token::U32Number(value) => Expression::TypedNumber { value, ty: Type::U32 },
        Token::BoolLiteral(value) => Expression::Bool(value)
    }
    .or(ident().map(Expression::Ident));

    let primary = choice((
        array_access(expr.clone()),
        create_array(expr.clone()),
        invert(expr.clone()),
        field_access(),
        function_call_expr(expr.clone()),
        create_struct(expr.clone()),
        base,
        deref_expr(expr.clone()),
        addr_of_expr(expr),
    ));

    cast(primary)
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{expression::Expression, ty::Type},
        lexing::token::Token,
        parsing::{
            expr::{atom::atom_expr, expr},
            test_ident, test_parse, test_tokens,
        },
    };

    #[test]
    fn parses_base_expr() {
        let number_tokens = test_tokens![Token::Number(42)];
        let i32_number_tokens = test_tokens![Token::I32Number(42)];
        let u32_number_tokens = test_tokens![Token::U32Number(42)];
        let bool_tokens = test_tokens![Token::BoolLiteral(true)];
        let ident_tokens = test_tokens![Token::Ident("x".into())];
        let string_tokens = test_tokens![Token::StringLiteral("hello".into())];

        assert_eq!(
            test_parse(atom_expr(expr()), &number_tokens),
            Expression::Number(42)
        );
        assert_eq!(
            test_parse(atom_expr(expr()), &i32_number_tokens),
            Expression::TypedNumber {
                value: 42,
                ty: Type::I32,
            }
        );
        assert_eq!(
            test_parse(atom_expr(expr()), &bool_tokens),
            Expression::Bool(true)
        );
        assert_eq!(
            test_parse(atom_expr(expr()), &u32_number_tokens),
            Expression::TypedNumber {
                value: 42,
                ty: Type::U32,
            }
        );
        assert_eq!(
            test_parse(atom_expr(expr()), &ident_tokens),
            Expression::Ident(test_ident("x"))
        );
        assert_eq!(
            test_parse(atom_expr(expr()), &string_tokens),
            Expression::StringLiteral("hello".into())
        );
    }
}
