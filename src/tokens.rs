use std::fmt::{self, Display, Formatter};

/// Defines the set of [`TokenKind`]s.
macro_rules! define_token_kinds {
    {$(($name:ident$(($field:ty))?, $doc:literal, $desc:literal)),* $(,)?} => {
        /// A [`Token`]'s kind.
        #[derive(Clone, Copy, Debug)]
        pub enum TokenKind {$(
            #[doc = $doc]
            $name$(($field))?
        ),*}

        impl TokenKind {
            /// Returns the `TokenKind`'s [`TokenType`].
            const fn token_type(self) -> TokenType {
                match self {$(
                    Self::$name { .. } => TokenType::$name
                ),*}
            }
        }

        /// A [`TokenKind`]'s discriminant type.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum TokenType {$(
            #[doc = $doc]
            $name
        ),*}

        impl TokenType {
            /// Returns a description of the `TokenType`.
            const fn description(self) -> &'static str {
                match self {$(
                    Self::$name => $desc
                ),*}
            }
        }
    };
}

define_token_kinds! {
    (Eof, "An end of source code marker.", "end of file"),
    (Literal(Literal), "A [`Literal`].", "a literal"),
    (OpenParen, "An opening parenthesis (`(`).", "an opening '('"),
    (CloseParen, "A closing parenthesis (`)`).", "a closing ')'"),
    (OpenBrace, "An opening brace (`{`).", "an opening '{'"),
    (CloseBrace, "A closing brace (`}`).", "a closing '}'"),
    (Comma, "A comma (`,`).", "','"),
    (Dot, "A dot (`.`).", "'.'"),
    (Minus, "A minus sign (`-`).", "'-'"),
    (Plus, "A plus sign (`+`).", "'+'"),
    (Semi, "A semicolon (`;`).", "';'"),
    (Slash, "A forward slash (`/`).", "'/'"),
    (Star, "An asterisk (`*`).", "'*'"),
    (Bang, "An exclamation mark (`!`).", "'!'"),
    (BangEquals, "An exclamation mark and equals sign (`!=`).", "'!='"),
    (Equals, "An equals sign (`=`).", "'='"),
    (EqualsEquals, "A double equals sign (`==`).", "'=='"),
    (Greater, "A greater than symbol (`>`).", "'>'"),
    (GreaterEquals, "A greater than symbol and equals sign (`>=`).", "'>='"),
    (Less, "A less than symbol (`<`).", "'<'"),
    (LessEquals, "A less than symbol and equals sign (`<=`).", "'<='"),
}

/// A lexical element of source code.
#[derive(Debug)]
pub struct Token {
    /// The [`TokenKind`].
    kind: TokenKind,
}

impl Token {
    /// Creates a new `Token` from its [`TokenKind`].
    pub const fn new(kind: TokenKind) -> Self {
        Self { kind }
    }

    /// Returns the `Token`'s [`TokenType`].
    pub const fn token_type(&self) -> TokenType {
        self.kind.token_type()
    }
}

/// A value which can be represented with a single [`Token`].
#[derive(Clone, Copy, Debug)]
pub enum Literal {
    /// A number.
    Number(f64),
}

impl Literal {
    /// Returns the `Literal`'s type name.
    const fn type_name(self) -> &'static str {
        match self {
            Self::Number(_) => "number",
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(literal) => {
                let type_name = literal.type_name();
                write!(f, "{type_name} '{literal}'")
            }
            _ => Display::fmt(&self.token_type(), f),
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => Display::fmt(value, f),
        }
    }
}
