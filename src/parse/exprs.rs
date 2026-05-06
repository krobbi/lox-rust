use crate::{
    ast::{BinOp, Expr, ExprKind, Ident, LogicOp, UnOp},
    diagnostics::Diag,
    spans::BytePos,
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

            let next_min_precedence = match op.associativity() {
                Associativity::Left => precedence + 1,
                Associativity::Right => precedence,
            };

            self.bump();
            let rhs = self.parse_expr_infix(next_min_precedence);
            lhs = self.make_infix_expr(op, lhs, rhs, start_pos);
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
            TokenKind::Ident(name) => {
                self.bump();
                ExprKind::Variable(name)
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
                self.report(Diag::ExpectedExpr(kind), self.next_token.span());
                ExprKind::Variable(Symbol::ERROR)
            }
        };

        self.make_expr(kind, start_pos)
    }

    /// Parses and returns a boxed slice of argument [`Expr`]s.
    fn parse_args(&mut self) -> Box<[Expr]> {
        self.bump_assert(TokenType::OpenParen);

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

    /// Returns a new infix [`Expr`] from an [`InfixOp`], operand [`Expr`]s, and
    /// a start [`BytePos`].
    fn make_infix_expr(&mut self, op: InfixOp, lhs: Expr, rhs: Expr, start_pos: BytePos) -> Expr {
        let kind = match op {
            InfixOp::Assign => return self.make_assign_expr(lhs, rhs, start_pos),
            InfixOp::Binary(op) => ExprKind::Binary(op, Box::new(lhs), Box::new(rhs)),
            InfixOp::Logic(op) => ExprKind::Logic(op, Box::new(lhs), Box::new(rhs)),
        };

        self.make_expr(kind, start_pos)
    }

    /// Returns a new assignment [`Expr`] from operand [`Expr`]s and a start
    /// [`BytePos`].
    fn make_assign_expr(&mut self, lhs: Expr, rhs: Expr, start_pos: BytePos) -> Expr {
        let kind = match lhs.kind {
            ExprKind::Variable(name) => ExprKind::AssignVar(
                Ident {
                    name,
                    span: lhs.span,
                },
                Box::new(rhs),
            ),
            ExprKind::Property(instance, name) => {
                ExprKind::AssignField(instance, name, Box::new(rhs))
            }
            _ => {
                self.report_recovered(Diag::InvalidAssign, lhs.span);
                ExprKind::Variable(Symbol::ERROR)
            }
        };

        self.make_expr(kind, start_pos)
    }

    /// Returns a new [`Expr`] from an [`ExprKind`] and a start [`BytePos`].
    fn make_expr(&self, kind: ExprKind, start_pos: BytePos) -> Expr {
        let span = self.span_from(start_pos);
        Expr { kind, span }
    }
}

/// An infix operator.
#[derive(Clone, Copy)]
enum InfixOp {
    /// An assignment operator.
    Assign,

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
            TokenType::Equals => Self::Assign,
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
            Self::Assign => Precedence::Assign,
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

    /// Returns the `InfixOp`'s [`Associativity`].
    const fn associativity(self) -> Associativity {
        match self {
            Self::Assign => Associativity::Right,
            Self::Binary(_) | Self::Logic(_) => Associativity::Left,
        }
    }
}

/// An [`InfixOp`]'s precedence level.
#[derive(Clone, Copy)]
#[repr(u8)]
enum Precedence {
    /// An assignment.
    Assign,

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

/// An [`InfixOp`]'s associativity.
enum Associativity {
    /// Left to right.
    Left,

    /// Right to left.
    Right,
}
