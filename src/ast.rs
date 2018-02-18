#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Ident(pub String);

impl Ident {
    pub fn new(name: String) -> Self {
        let mut chars = name.chars();

        assert!(chars.next().expect("An identifier must not be an empty string.").is_alphabetic());

        for character in chars {
            assert!(character.is_alphanumeric());
        }

        Self(name)
    }
}

pub mod lexer {
    use std::str::FromStr;

    enum Integer {
        // Undetermined(String),
        i32(i32),
        // i64(i64),
    }

    enum Float {
        // Undetermined(String),
        f32(f32),
        // f64(f64),
    }

    #[derive(Debug, PartialEq, Eq)]
    enum Literal {
        Boolean(bool),
        Float(String),
        Integer(String),
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug, PartialEq, Eq)]
    pub enum Primitive {
        bool,
        i32,
        f32,
    }

    impl FromStr for Primitive {
        type Err = String;

        fn from_str(other: &str) -> Result<Self, Self::Err> {
            Ok(match other {
                "bool" => Primitive::bool,
                "i32" => Primitive::i32,
                "f32" => Primitive::f32,
                _ => return Err(format!("No primitive type '{}' found.", other)),
            })
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct TypeFunction {
        pub args: Vec<Type>,
        pub return_ty: Box<Type>,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct TypeCustom {
        pub name: String,
        pub generics: Vec<String>,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Type {
        Primitive(Primitive),
        Function(TypeFunction),
        Custom(TypeCustom),
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct FnHeader {
        pub ident: Ident,
        pub ty: TypeFunction,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Variable {
        pub ident: Ident,
        pub ty: Type,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct FnCall {
        pub ident: Ident,
        pub args: Vec<(Ident, Ident)>,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Expr {
        Literal(Literal),
        Variable(Ident),
        FnCall(FnCall),
        Double(String), // FIXME Just for testing
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct FnDecl {
        pub ident: Ident,
        pub args: Vec<Variable>,
        pub return_ty: Type,
        pub body: Expr,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Item {
        FnDecl(FnDecl),
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Root {
        pub items: Vec<Item>,
    }
}
