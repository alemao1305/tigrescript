use super::ast::{Expr, Literal, BinOp};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Some(Token::Int(n)) => {
                self.advance();
                Ok(Expr::Literal(Literal::Int(*n)))
            }
            // ... outros casos
        }
    }
}