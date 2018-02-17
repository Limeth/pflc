enum Boolean {
    True,
    False,
}

enum Literal<'a> {
    Boolean {
        string: &'a str,
        value: Boolean,
    },
    FloatingPoint(&'a str),
}
