use crate::language::{
    nodes::Node,
    token::{Token, TokenKind},
};

type TokenResult = Result<Token, String>;
type NodeResult = Result<Node, String>;

#[derive(Debug, Clone)]
pub struct Parser {
    source: String,
    tokens: Vec<Token>,
    pos: i32,
}

impl Parser {
    pub fn new(source: String, tokens: Vec<Token>) -> Self {
        Self {
            source,
            tokens,
            pos: -1,
        }
    }

    pub fn advance(&mut self) -> TokenResult {
        self.pos += 1;

        if self.pos < self.tokens.len() as i32 {
            Ok(self.tokens[self.pos as usize])
        } else {
            Err("Expected more tokens. Got [EOF].".to_string())
        }
    }

	#[allow(unused)]
    pub fn current(&self) -> TokenResult {
        if self.pos < self.tokens.len() as i32 {
            Ok(self.tokens[self.pos as usize])
        } else {
            Err("Current expected more tokens. Got [EOF].".to_string())
        }
    }

    pub fn peek(&self) -> Option<Token> {
        if self.pos + 1 < self.tokens.len() as i32 {
            Some(self.tokens[(self.pos + 1) as usize])
        } else {
            None
        }
    }

    pub fn expect(&self, kind: TokenKind) -> Result<(), String> {
		if let Some(next) = self.peek() {
			if next.kind != kind {
				Err(format!("Expected `{kind:?}`, got `{:?}`", next.kind))
			} else {
				Ok(())
			}
		} else {
			Err(format!("Expected `{kind:?}`, got [EOF]"))
		}
    }

    pub fn parse_add_sub(&mut self) -> NodeResult {
        let mut left = self.parse_mul_div()?;

        while let Some(next) = self.peek() {
            if !matches!(next.kind, TokenKind::PLUS | TokenKind::MINUS) {
                break;
            }

            let op = self.advance()?.kind;
            let right = self.parse_mul_div()?;

            left = Node::BinOp {
                left: Box::new(left),
                right: Box::new(right),
                op,
            };
        }

        Ok(left)
    }

    pub fn parse_mul_div(&mut self) -> NodeResult {
        let mut left = self.parse_unary()?;

        while let Some(next) = self.peek() {
            if !matches!(
                next.kind,
                TokenKind::STAR | TokenKind::SLASH | TokenKind::MOD
            ) {
                break;
            }

            let op = self.advance()?.kind;
            let right = self.parse_unary()?;

            left = Node::BinOp {
                left: Box::new(left),
                right: Box::new(right),
                op,
            };
        }

        Ok(left)
    }

    pub fn parse_unary(&mut self) -> NodeResult {
        if let Some(next) = self.peek()
            && matches!(next.kind, TokenKind::PLUS | TokenKind::MINUS)
        {
            return Ok(Node::UnaryOp {
                op: self.advance()?.kind,
                right: Box::new(self.parse_exponent()?),
            });
        }

        Ok(self.parse_exponent()?)
    }

    pub fn parse_exponent(&mut self) -> NodeResult {
        let mut left = self.parse_primary()?;

        while let Some(next) = self.peek() {
            if !matches!(next.kind, TokenKind::POW) {
                break;
            }

            let op = self.advance()?.kind;
            let right = self.parse_primary()?;

            left = Node::BinOp {
                left: Box::new(left),
                right: Box::new(right),
                op,
            };
        }

        Ok(left)
    }

    pub fn parse_primary(&mut self) -> NodeResult {
        let current = self.advance()?;

        match current.kind {
            TokenKind::IntLiteral(x) => Ok(Node::IntLiteral(x)),
            TokenKind::FloatLiteral(x) => Ok(Node::FloatLiteral(x)),
            TokenKind::BooleanLiteral(x) => Ok(Node::BooleanLiteral(x)),
            TokenKind::StringLiteral => Ok(Node::StringLiteral(
                current.get_text(&self.source).to_string(),
            )),
            TokenKind::CharLiteral => Ok(Node::CharLiteral(current.get_text(&self.source).into())),

			TokenKind::LPAREN => {
				let expr = self.parse_expression()?;

				self.expect(TokenKind::RPAREN)?;
				self.advance()?;

				Ok(expr)
			}

            _ => Err("Got unexpected token while parsing primary.".to_string()),
        }
    }

    pub fn parse_expression(&mut self) -> NodeResult {
        self.parse_add_sub()
    }
}
