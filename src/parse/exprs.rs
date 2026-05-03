use crate::{
    ast::{BinOp, Expr, ExprKind, LogicOp, UnOp},
    diagnostics::Diag,
    spans::{BytePos, Span},
    symbols::Symbol,
    tokens::{TokenKind, TokenType},
};

use super::Parser;

impl Parser<'_, '_, '_> {
    /// Parses and returns an [`Expr`].
    pub fn parse_expr(&mut self) -> Expr {
        self.parse_expr_infix(0)
    }

    /// Parses and returns an infix [`Expr`].
    fn parse_expr_infix(&mut self, min_precedence: u8) -> Expr {
        let start_pos = self.start_pos();
        let mut lhs = self.parse_expr_unary();

        while let Some(op) = InfixOp::from_token_type(self.peek()) {
            let precedence = op.precedence();

            if precedence < min_precedence {
                break;
            }

            self.bump();
            let rhs = self.parse_expr_infix(precedence + 1);
            lhs = self.make_expr(op.make_expr_kind(lhs, rhs), start_pos);
        }

        lhs
    }

    /// Parses and returns a unary [`Expr`].
    fn parse_expr_unary(&mut self) -> Expr {
        let start_pos = self.start_pos();

        let op = match self.peek() {
            TokenType::Minus => UnOp::Minus,
            TokenType::Bang => UnOp::Not,
            _ => return self.parse_expr_call(),
        };

        self.bump();
        let rhs = self.parse_expr_unary();
        self.make_expr(ExprKind::Unary(op, Box::new(rhs)), start_pos)
    }

    /// Parses and returns a call [`Expr`].
    fn parse_expr_call(&mut self) -> Expr {
        let start_pos = self.start_pos();
        let mut lhs = self.parse_expr_primary();

        loop {
            let kind = match self.peek() {
                TokenType::OpenParen => {
                    let args = self.parse_args();
                    ExprKind::Call(Box::new(lhs), args)
                }
                TokenType::Dot => {
                    self.bump();
                    let ident = self.parse_ident();
                    ExprKind::Property(Box::new(lhs), ident)
                }
                _ => break lhs,
            };

            lhs = self.make_expr(kind, start_pos);
        }
    }

    /// Parses and returns a primary [`Expr`].
    fn parse_expr_primary(&mut self) -> Expr {
        let start_pos = self.start_pos();

        let kind = match self.next_token.kind() {
            TokenKind::Literal(literal) => {
                self.bump();
                ExprKind::Literal(literal)
            }
            TokenKind::Ident(symbol) => {
                self.bump();
                ExprKind::Variable(symbol)
            }
            TokenKind::OpenParen => {
                self.bump();
                let expr = self.parse_expr();
                self.expect(TokenType::CloseParen);
                ExprKind::Paren(Box::new(expr))
            }
            TokenKind::Super => {
                self.bump();
                self.expect(TokenType::Dot);
                let ident = self.parse_ident();
                ExprKind::Super(ident)
            }
            TokenKind::This => {
                self.bump();
                ExprKind::This
            }
            kind => {
                let span = self.next_token.span();
                return self.error_expr(Diag::ExpectedExpr(kind), span);
            }
        };

        self.make_expr(kind, start_pos)
    }

    /// Parses and returns a boxed slice of argument [`Expr`]s.
    fn parse_args(&mut self) -> Box<[Expr]> {
        debug_assert_eq!(
            self.peek(),
            TokenType::OpenParen,
            "parsed arguments without opening parenthesis"
        );

        self.bump();

        if self.eat(TokenType::CloseParen) {
            return Box::new([]);
        }

        let mut args = vec![self.parse_expr()];

        while self.eat(TokenType::Comma) {
            args.push(self.parse_expr());
        }

        self.expect(TokenType::CloseParen);
        args.into_boxed_slice()
    }

    /// Returns a new [`Expr`] from an [`ExprKind`] and a start [`BytePos`].
    fn make_expr(&self, kind: ExprKind, start_pos: BytePos) -> Expr {
        let span = self.span_from(start_pos);
        Expr { kind, span }
    }

    /// Reports a [`Diag`] at a [`Span`] and returns a new synthetic [`Expr`]
    /// for error recovery.
    fn error_expr(&mut self, diag: Diag, span: Span) -> Expr {
        self.report(diag, span);

        Expr {
            kind: ExprKind::Variable(Symbol::ERROR),
            span,
        }
    }
}

/// An infix operator.
#[derive(Clone, Copy)]
enum InfixOp {
    /// A [`BinOp`].
    Binary(BinOp),

    /// A [`LogicOp`].
    Logic(LogicOp),
}

impl InfixOp {
    /// Creates a new `InfixOp` from a [`TokenType`]. This function returns
    /// [`None`] if the [`TokenType`] does not correspond to an `InfixOp`.
    const fn from_token_type(token_type: TokenType) -> Option<Self> {
        let op = match token_type {
            TokenType::Minus => Self::Binary(BinOp::Subtract),
            TokenType::Plus => Self::Binary(BinOp::Add),
            TokenType::Slash => Self::Binary(BinOp::Divide),
            TokenType::Star => Self::Binary(BinOp::Multiply),
            TokenType::BangEquals => Self::Binary(BinOp::NotEqual),
            TokenType::EqualsEquals => Self::Binary(BinOp::Equal),
            TokenType::Greater => Self::Binary(BinOp::Greater),
            TokenType::GreaterEquals => Self::Binary(BinOp::GreaterEqual),
            TokenType::Less => Self::Binary(BinOp::Less),
            TokenType::LessEquals => Self::Binary(BinOp::LessEqual),
            TokenType::And => Self::Logic(LogicOp::And),
            TokenType::Or => Self::Logic(LogicOp::Or),
            _ => return None,
        };

        Some(op)
    }

    /// Returns the `InfixOp`'s precedence level.
    const fn precedence(self) -> u8 {
        let precedence = match self {
            Self::Binary(BinOp::Add | BinOp::Subtract) => Precedence::Sum,
            Self::Binary(BinOp::Multiply | BinOp::Divide) => Precedence::Term,
            Self::Binary(BinOp::Equal | BinOp::NotEqual) => Precedence::Equality,
            Self::Binary(BinOp::Greater | BinOp::GreaterEqual | BinOp::Less | BinOp::LessEqual) => {
                Precedence::Comparison
            }
            Self::Logic(LogicOp::And) => Precedence::And,
            Self::Logic(LogicOp::Or) => Precedence::Or,
        };

        precedence as u8
    }

    /// Returns a new [`ExprKind`] from the `InfixOp` and operand [`Expr`]s.
    fn make_expr_kind(self, lhs: Expr, rhs: Expr) -> ExprKind {
        match self {
            Self::Binary(op) => ExprKind::Binary(op, Box::new(lhs), Box::new(rhs)),
            Self::Logic(op) => ExprKind::Logic(op, Box::new(lhs), Box::new(rhs)),
        }
    }
}

/// An [`InfixOp`]'s precedence level.
#[derive(Clone, Copy)]
#[repr(u8)]
enum Precedence {
    /// A logical or.
    Or,

    /// A logical and.
    And,

    /// An equality test.
    Equality,

    /// A comparison.
    Comparison,

    /// An addition or subtraction.
    Sum,

    /// A multiplication or division.
    Term,
}
