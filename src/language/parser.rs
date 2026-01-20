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
            pos: 0,
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
    fn current(&self) -> TokenResult {
        if self.pos < self.tokens.len() as i32 {
            Ok(self.tokens[self.pos as usize])
        } else {
            Err("Current expected more tokens. Got [EOF].".to_string())
        }
    }

    fn peek(&self) -> Option<Token> {
        if self.pos + 1 < self.tokens.len() as i32 {
            Some(self.tokens[(self.pos + 1) as usize])
        } else {
            None
        }
    }

    fn expect(&self, kind: TokenKind) -> Result<(), String> {
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

    fn expect_and_consume(&mut self, kind: TokenKind) -> Result<Token, String> {
        if let Some(next) = self.peek() {
            if next.kind != kind {
                Err(format!("Expected `{kind:?}`, got `{:?}`", next.kind))
            } else {
                Ok(self.advance()?)
            }
        } else {
            Err(format!("Expected `{kind:?}`, got [EOF]."))
        }
    }

    pub fn parse(&mut self) -> NodeResult {
		self.skip_new_lines();
		
        match self.current()?.kind {
            TokenKind::LET => self.parse_let(),
            // TokenKind::LBRACE => self.parse_block(),

            _ => self.parse_expression(),
        }
    }
}

// EXPRESSIONS
impl Parser {
    pub fn parse_add_sub(&mut self) -> NodeResult {
        let mut left = self.parse_mul_div()?;

        while let Some(next) = self.peek() {
            if !matches!(next.kind, TokenKind::PLUS | TokenKind::MINUS) {
                break;
            }

            self.skip_new_lines();
            let op = self.advance()?.kind;
			self.advance()?;
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

            self.skip_new_lines();
            let op = self.advance()?.kind;
			self.advance()?;
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
        self.skip_new_lines();

        if let Ok(next) = self.current()
            && matches!(next.kind, TokenKind::PLUS | TokenKind::MINUS)
        {
			let op = self.current()?.kind;
			self.advance()?;

            return Ok(Node::UnaryOp {
                op,
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

            self.skip_new_lines();
            let op = self.advance()?.kind;
			self.advance()?;
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
        self.skip_new_lines();
        let current = self.current()?;

        let node = match current.kind {
            TokenKind::IntLiteral(x) => Ok(Node::IntLiteral(x)),
            TokenKind::FloatLiteral(x) => Ok(Node::FloatLiteral(x)),
            TokenKind::BooleanLiteral(x) => Ok(Node::BooleanLiteral(x)),
            TokenKind::StringLiteral => Ok(Node::StringLiteral(
                current.get_text(&self.source).to_string(),
            )),
            TokenKind::CharLiteral => Ok(Node::CharLiteral(current.get_text(&self.source).into())),

            TokenKind::LPAREN => {
                let expr = self.parse_expression()?;

                self.skip_new_lines();
                self.expect(TokenKind::RPAREN)?;
                self.advance()?;

                Ok(expr)
            }

            other => Err(format!("Got unexpected token `{other:?}` while parsing primary.")),
        };

		node
    }

    pub fn skip_new_lines(&mut self) {
		if let Ok(next) = self.current() {
			if matches!(next.kind, TokenKind::NEWLINE) {
                self.advance().unwrap();
			}
		}
        while let Some(next) = self.peek() {
            if matches!(next.kind, TokenKind::NEWLINE) {
                self.advance().unwrap();
            } else {
				break;
			}
        }
    }

    pub fn parse_expression(&mut self) -> NodeResult {
        self.parse_add_sub()
    }
}

// STATEMENTS
impl Parser {
    fn parse_let(&mut self) -> NodeResult {
        let name = self.expect_and_consume(TokenKind::Identifier)?;
        self.expect_and_consume(TokenKind::EQUAL)?;
		self.advance()?;

        Ok(Node::LetStatement {
            name: name.get_text(&self.source).to_string(),
            value: Box::new(self.parse_expression()?),
        })
    }

	// fn parse_block(&mut self) -> NodeResult {
	// 	self.advance()?;
	// 	self.skip_new_lines();

	// 	let mut body = vec![];
	// 	while let Some(next) = self.peek() {
	// 		self.skip_new_lines();
	// 		println!("{next:?}");
	// 		if next.kind == TokenKind::RBRACE {
	// 			break;
	// 		}
	// 		body.push(self.parse()?);
	// 	}

	// 	self.expect_and_consume(TokenKind::RBRACE)?;

	// 	Ok(Node::Block { body })
	// }
}
