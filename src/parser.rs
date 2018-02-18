use nom::simple_errors::Context;
use nom::Err::Error;
use nom::ErrorKind;
use nom::types::CompleteStr;
use nom::{digit, space};
use ast::lexer;

macro_rules! check_parser {
    ($parser_fn:ident; $input:expr => $rest:expr, $output:expr) => {
        assert_eq!($parser_fn(CompleteStr($input)), Ok((CompleteStr($rest), $output)));
    }
}

macro_rules! check_parser_err {
    ($parser_fn:ident; $input:expr => $rest:expr, $kind:expr) => {
        if let Err(Error(Context::Code(code, kind))) = $parser_fn(CompleteStr($input)) {
            assert_eq!(code, CompleteStr($rest));
            assert_eq!(kind, $kind);
        } else {
            panic!("Parsing `{}` using `{}` doesn't produce an error, but should.", stringify!($input), stringify!($parser_fn));
        }
    }
}

// use syntax::parser::lexer as rust_lexer;

// fn ident_start(c: char) -> bool {
//     rust_lexer::ident_start(Some(c))
// }

// fn ident_continue(c: char) -> bool {
//     rust_lexer::ident_continue(Some(c))
// }

named!(alphasep<CompleteStr, CompleteStr>,
    alt!(
        recognize!(one_of!(" \t\r\n,;:.<>{}[]()+-%*/=^?\"'"))
        | recognize!(tuple!(char!('\n'), char!('\r')))
    )
);

named!(parse_comment<CompleteStr, CompleteStr>,
    alt!(
        preceded!(tag!("//"), take_until!("\n")) |
        delimited!(tag!("/*"), take_until!("*/"), tag!("*/"))
    )
);

#[test]
fn test_parse_comment() {
    check_parser!(parse_comment; "// one-line\nrest" => "\nrest", CompleteStr(" one-line"));
    check_parser!(parse_comment; "/* multi\nline */ rest" => " rest", CompleteStr(" multi\nline "));
}

named!(parse_identifier<CompleteStr, CompleteStr>,
    recognize!(
        do_parse!(
            one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
            >> many0!(
                one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")
            )
            >> ()
        )
    )
);

#[test]
fn test_parse_identifier() {
    check_parser!(parse_identifier; "helloWorld42 rest" => " rest", CompleteStr("helloWorld42"));
    check_parser_err!(parse_identifier; "0helloWorld42 rest" => "0helloWorld42 rest", ErrorKind::OneOf);
}

named!(parse_type_primitive<CompleteStr, lexer::Primitive>,
    flat_map!(
        alt!(
            tag!("bool")
            | tag!("i32")
            | tag!("f32")
        ),
        parse_to!(lexer::Primitive)
    )
);

#[test]
fn test_parse_type_primitive() {
    check_parser!(parse_type_primitive; "bool rest" => " rest", lexer::Primitive::bool);
    check_parser!(parse_type_primitive; "i32 rest" => " rest", lexer::Primitive::i32);
    check_parser!(parse_type_primitive; "f32 rest" => " rest", lexer::Primitive::f32);
}

named!(parse_type<CompleteStr, lexer::Type>,
    alt!(
        do_parse!(
            x: parse_type_primitive
            >> (lexer::Type::Primitive(x))
        )
    )
);

#[test]
fn test_parse_type() {
    check_parser!(parse_type; "bool rest" => " rest", lexer::Type::Primitive(lexer::Primitive::bool));
    check_parser!(parse_type; "i32 rest" => " rest", lexer::Type::Primitive(lexer::Primitive::i32));
    check_parser!(parse_type; "f32 rest" => " rest", lexer::Type::Primitive(lexer::Primitive::f32));
}

// {{{ Literals

// {{{ bool
named!(parse_boolean<CompleteStr, bool>,
  flat_map!(
    alt!(tag!("true") | tag!("false")),
    parse_to!(bool)
  )
);

#[test]
fn test_parse_boolean() {
    check_parser!(parse_boolean; "true" => "", true);
    check_parser!(parse_boolean; "false" => "", false);
}
// }}}

// {{{ i32
// macro_rules! parse_i32_base_variant_e {
//     ($prefix:expr, $digits:expr) => {
//       tuple!(
//         opt!(alt!(tag!("+") | tag!("-"))),
//         tag!($prefix),
//         many1!(is_a!($digits)),
//         opt!(tuple!(
//           alt!(tag!("e") | tag!("E")),
//           opt!(alt!(tag!("+") | tag!("-"))),
//             many1!(is_a!($digits)),
//           )
//         )
//       )
//     };

//     ($digits:expr) => {
//       tuple!(
//         opt!(alt!(tag!("+") | tag!("-"))),
//         many1!(is_a!($digits)),
//         opt!(tuple!(
//           alt!(tag!("e") | tag!("E")),
//           opt!(alt!(tag!("+") | tag!("-"))),
//             many1!(is_a!($digits)),
//           )
//         )
//       )
//     }
// }

// macro_rules! parse_i32_base_variant {
//     ($prefix:expr, $digits:expr) => {
//       tuple!(
//         opt!(alt!(tag!("+") | tag!("-"))),
//         tag!($prefix),
//         many1!(is_a!($digits)),
//       )
//     };

//     ($digits:expr) => {
//       tuple!(
//         opt!(alt!(tag!("+") | tag!("-"))),
//         many1!(is_a!($digits)),
//       )
//     }
// }

// named!(parse_i32<CompleteStr, i32>,
//   flat_map!(
//     recognize!(
//       tuple!(
//         alt!(
//           parse_i32_base_variant!("0b", "01")
//           | parse_i32_base_variant_e!("0123456789")
//           | parse_i32_base_variant!("0o", "01234567")
//           | parse_i32_base_variant_e!("0x", "0123456789abcdef")
//         ),
//         opt!(tag!("i32"))
//       )
//     ),
//     parse_to!(i32)
//   )
// );

// named!(parse_i32<CompleteStr, i32>,
//   flat_map!(
//     recognize!(
//       tuple!(
//         opt!(alt!(tag!("+") | tag!("-"))),
//         digit,
//         opt!(tuple!(
//           alt!(tag!("e") | tag!("E")),
//           opt!(alt!(tag!("+") | tag!("-"))),
//           digit
//           )
//         ),
//         opt!(tag!("i32"))
//       )
//     ),
//     parse_to!(i32)
//   )
// );

named!(parse_i32<CompleteStr, i32>,
  flat_map!(
    digit,
    parse_to!(i32)
  )
);

#[test]
fn test_parse_i32() {
    check_parser!(parse_i32; "10 rest" => " rest", 10i32);
}
// }}}

// {{{ f32
// named!(parse_f32<CompleteStr, f32>,
//     do_parse!(
//       flat_map!(
//         recognize!(
//           tuple!(
//             opt!(alt!(tag!("+") | tag!("-"))),
//             alt!(
//               delimited!(digit, tag!("."), opt!(digit))
//               | delimited!(opt!(digit), tag!("."), digit)
//             ),
//             opt!(tuple!(
//               alt!(tag!("e") | tag!("E")),
//               opt!(alt!(tag!("+") | tag!("-"))),
//               digit
//               )
//             )
//           )
//         ),
//         parse_to!(f32)
//       )
//     )
// );

// #[test]
// fn test_parse_f32() {
//     check_parser!(parse_f32; "10 rest" => " rest", 10f32);
//     check_parser!(parse_f32; "10_f32 rest" => " rest", 10f32);
// }
// }}}

// }}}

named!(parse_variable<CompleteStr, lexer::Variable>,
    do_parse!(
        ident: parse_identifier
        >> opt!(space)
        >> char!(':')
        >> opt!(space)
        >> ty: parse_type
        >> (lexer::Variable {
            ident,
            ty,
        })
    )
);

named!(parse_variables<CompleteStr, Vec<lexer::Variable>>,
    do_parse!(
        // TODO
    )
);

named!(parse_expr<CompleteStr, lexer::Variable>,
    do_parse!(
        // TODO
    )
)

named!(parse_fn_decl<CompleteStr, lexer::FnDecl>,
    do_parse!(
        tag!("fn")
        >> many1!(space)
        >> ident: parse_identifier
        >> many0!(space)
        >> char!('(')
        >> many0!(space)
        >> args: separated_list!(
            tuple!(many0!(space), char!(','), many0!(space)),
            parse_variable // TODO replace with parse_variables
        )
        >> many0!(space)
        >> char!(')')
        >> tag!("->")
        >> many0!(space)
        >> return_ty: parse_type
        >> many0!(space)
        >> char!('=')
        >> many0!(space)
        >> body: parse_expr
        >> many0!(space)
        >> char!(';')
        >> (lexer::FnDecl {
            ident,
            args,
            return_ty,
            body,
        })
    )
);

named!(parse_root<CompleteStr, lexer::Root>,
    many0!(
        alt!(
            do_parse!(
                x: parse_fn_decl
                >> (lexer::FnDecl(x))
            )
        )
    )
);

pub fn parse(source: &str) -> lexer::Root {
    parse_root(CompleteStr(source))
}
