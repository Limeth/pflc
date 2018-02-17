use nom::simple_errors::Context;
use nom::Err::Error;
use nom::ErrorKind;
use nom::types::CompleteStr;
use nom::digit;

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

named!(alphasep<CompleteStr, char>, peek!(one_of!(" \t\n,;:.<>{}[]()+-%*/=^?\"'")));

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

named!(parse_i32<CompleteStr, i32>,
  flat_map!(
    recognize!(
      tuple!(
        opt!(alt!(tag!("+") | tag!("-"))),
        digit,
        opt!(tuple!(
          alt!(tag!("e") | tag!("E")),
          opt!(alt!(tag!("+") | tag!("-"))),
          digit
          )
        ),
        opt!(tag!("i32"))
      )
    ),
    parse_to!(i32)
  )
);

#[test]
fn test_parse_i32() {
    check_parser!(parse_i32; "10 rest" => " rest", 10i32);
    check_parser!(parse_i32; "10_i32 rest" => " rest", 10i32);
}
// }}}

// {{{ f32
named!(parse_f32<CompleteStr, f32>,
  flat_map!(
    recognize!(
      tuple!(
        opt!(alt!(tag!("+") | tag!("-"))),
        alt!(
          delimited!(digit, tag!("."), opt!(digit))
          | delimited!(opt!(digit), tag!("."), digit)
        ),
        opt!(tuple!(
          alt!(tag!("e") | tag!("E")),
          opt!(alt!(tag!("+") | tag!("-"))),
          digit
          )
        ),
        opt!(tag!("f32"))
      )
    ),
    parse_to!(f32)
  )
);

#[test]
fn test_parse_f32() {
    check_parser!(parse_f32; "10 rest" => " rest", 10f32);
    check_parser!(parse_f32; "10_f32 rest" => " rest", 10f32);
}
// }}}

// }}}
