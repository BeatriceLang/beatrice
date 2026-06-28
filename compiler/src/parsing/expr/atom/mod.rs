use chumsky::{Parser, primitive::choice, select};

use crate::{
    ast::{expression::Expression, ty::Type},
    lexing::token::Token,
    parsing::{
        expr::atom::{
            addr_of::addr_of, array_access::array_access, cast::cast, create_array::create_array,
            create_struct::create_struct, deref::deref, field_access::field_access,
            function_call::function_call, invert::invert,
        },
        ident::ident,
    },
};

mod addr_of;
mod array_access;
mod cast;
mod create_array;
mod create_struct;
mod deref;
mod field_access;
mod function_call;
mod invert;

pub fn atom<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    let base = select! {
        Token::Number(value) => Expression::Number(value),
        Token::StringLiteral(string) => Expression::StringLiteral(string),
        Token::I32Number(value) => Expression::TypedNumber { value, ty: Type::I32 },
        Token::U32Number(value) => Expression::TypedNumber { value, ty: Type::U32 },
        Token::BoolLiteral(value) => Expression::Bool(value)
    };

    let atom = choice((
        array_access(expr.clone()),
        create_array(expr.clone()),
        invert(expr.clone()),
        field_access(),
        function_call(expr.clone()),
        create_struct(expr.clone()),
        base,
        ident().map(Expression::Ident),
        deref(expr.clone()),
        addr_of(expr),
    ));

    cast(atom)
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{expression::Expression, ty::Type},
        lexing::token::Token,
        parsing::{
            expr::{atom::atom, expr},
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
            test_parse(atom(expr()), &number_tokens),
            Expression::Number(42)
        );
        assert_eq!(
            test_parse(atom(expr()), &i32_number_tokens),
            Expression::TypedNumber {
                value: 42,
                ty: Type::I32,
            }
        );
        assert_eq!(
            test_parse(atom(expr()), &bool_tokens),
            Expression::Bool(true)
        );
        assert_eq!(
            test_parse(atom(expr()), &u32_number_tokens),
            Expression::TypedNumber {
                value: 42,
                ty: Type::U32,
            }
        );
        assert_eq!(
            test_parse(atom(expr()), &ident_tokens),
            Expression::Ident(test_ident("x"))
        );
        assert_eq!(
            test_parse(atom(expr()), &string_tokens),
            Expression::StringLiteral("hello".into())
        );
    }
}
