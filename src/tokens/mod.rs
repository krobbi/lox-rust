mod render;

use crate::{spans::Span, symbols::Symbol};

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
    (Ident(Symbol), "An identifier.", "an identifier"),
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
    (And, "An `and` keyword.", "keyword 'and'"),
    (Class, "A `class` keyword.", "keyword 'class'"),
    (Else, "An `else` keyword.", "keyword 'else'"),
    (For, "A `for` keyword.", "keyword 'for'"),
    (Fun, "A `fun` keyword.", "keyword 'fun'"),
    (If, "An `if` keyword.", "keyword 'if'"),
    (Or, "An `or` keyword.", "keyword 'or'"),
    (Print, "A `print` keyword.", "keyword 'print'"),
    (Return, "A `return` keyword.", "keyword 'return'"),
    (Super, "A `super` keyword.", "keyword 'super'"),
    (This, "A `this` keyword.", "keyword 'this'"),
    (Var, "A `var` keyword.", "keyword 'var'"),
    (While, "A `while` keyword.", "keyword 'while'"),
}

/// A lexical element of source code.
#[derive(Debug)]
pub struct Token {
    /// The [`TokenKind`].
    kind: TokenKind,

    /// The [`Span`].
    span: Span,
}

impl Token {
    /// Creates a new `Token` from its [`TokenKind`] and [`Span`].
    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Returns the `Token`'s [`TokenType`].
    pub const fn token_type(&self) -> TokenType {
        self.kind.token_type()
    }
}

/// A value which can be represented with a single [`Token`].
#[derive(Clone, Copy, Debug)]
pub enum Literal {
    /// A nil value.
    Nil,

    /// A Boolean value.
    Bool(bool),

    /// A number.
    Number(f64),

    /// A string.
    String(Symbol),
}
