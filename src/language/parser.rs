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
        let current = self.current();
        self.pos += 1;

        current
    }

    #[allow(unused)]
    fn current(&self) -> TokenResult {
        if self.pos < self.tokens.len() as i32 {
            Ok(self.tokens[self.pos as usize].clone())
        } else {
            Err("Current expected more tokens. Got [EOF].".to_string())
        }
    }

    pub fn peek(&self) -> Option<Token> {
        if self.pos + 1 < self.tokens.len() as i32 {
            Some(self.tokens[(self.pos + 1) as usize].clone())
        } else {
            None
        }
    }

    fn expect(&self, kind: TokenKind) -> Result<(), String> {
        if let Ok(next) = self.current() {
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
        let next = self.current()?;
        if next.kind != kind {
            Err(format!("Expected `{kind:?}`, got `{:?}`", next.kind))
        } else {
            self.advance()?;
            Ok(next)
        }
    }

    pub fn parse(&mut self) -> NodeResult {
        self.skip_new_lines();

        match self.current()?.kind {
            TokenKind::LET => self.parse_let(),
            TokenKind::LBRACE => self.parse_block(),

            _ => self.parse_expression(),
        }
    }
}

// EXPRESSIONS
impl Parser {
    pub fn parse_add_sub(&mut self) -> NodeResult {
        let mut left = self.parse_mul_div()?;

        while let Ok(next) = self.current() {
            if !matches!(next.kind, TokenKind::PLUS | TokenKind::MINUS) {
                break;
            }

            self.skip_new_lines();
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

        while let Ok(next) = self.current() {
            if !matches!(
                next.kind,
                TokenKind::STAR | TokenKind::SLASH | TokenKind::MOD
            ) {
                break;
            }

            self.skip_new_lines();
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
        self.skip_new_lines();

        if let Ok(next) = self.current()
            && matches!(next.kind, TokenKind::PLUS | TokenKind::MINUS)
        {
            let op = self.advance()?.kind;

            return Ok(Node::UnaryOp {
                op,
                right: Box::new(self.parse_exponent()?),
            });
        }

        Ok(self.parse_exponent()?)
    }

    pub fn parse_exponent(&mut self) -> NodeResult {
        let mut left = self.parse_primary()?;

        while let Ok(next) = self.current() {
            if !matches!(next.kind, TokenKind::POW) {
                break;
            }

            self.skip_new_lines();
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
        self.skip_new_lines();
        let current = self.current()?;

        let node = match current.kind {
            TokenKind::IntLiteral(x) => Ok(Node::IntLiteral(x)),
            TokenKind::FloatLiteral(x) => Ok(Node::FloatLiteral(x)),
            TokenKind::BooleanLiteral(x) => Ok(Node::BooleanLiteral(x)),
            TokenKind::StringLiteral(_) => Ok(Node::StringLiteral({
                let text = current.get_text(&self.source);
                text[1..text.len() - 1].to_string()
            })),
            TokenKind::CharLiteral(_) => Ok(Node::CharLiteral({
                let text = current.get_text(&self.source);
                text[1..text.len() - 1].into()
            })),

            TokenKind::LPAREN => {
                self.advance()?;
                self.skip_new_lines();

                let expr = if let Some(next) = self.current().ok() {
                    if next.kind == TokenKind::RPAREN {
                        Node::Null
                    } else {
                        self.parse_expression()?
                    }
                } else {
                    return Err("Unexpected end of input inside parentheses".to_string());
                };

                self.expect_and_consume(TokenKind::RPAREN)?;
                Ok(expr)
            }

            TokenKind::FUNC => {
                self.advance()?;
                let func_name = self.parse_declaration_name()?;
                Ok(Node::FuncDeclaration {
                    name: func_name.to_string(),
                    params: Box::new(Node::Null),
                    return_type: Box::new(Node::Null),
                    block: Box::new(Node::Block { body: vec![] }),
                })
            }

            other => Err(format!(
                "Got unexpected token `{other:?}` while parsing primary."
            )),
        };

        node
    }

    pub fn parse_declaration_name(&mut self) -> Result<String, String> {
        let current = self.current()?;

        // getting the actual name
        if let TokenKind::Identifier(name) = &current.kind {
            let name = name.clone();
            self.advance();
            Ok(name)
        } else {
            Err(format!("Expected an identifier, found {:?}", current.kind))
        }
    }

    pub fn skip_new_lines(&mut self) {
        // if let Ok(next) = self.current() {
        //     if matches!(next.kind, TokenKind::NEWLINE) {
        //         self.advance().unwrap();
        //     }
        // }
        while let Ok(next) = self.current() {
            if matches!(next.kind, TokenKind::NEWLINE) {
                self.advance();
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
        self.advance()?;
        let name = self.expect_and_consume_identifier()?;
        self.expect_and_consume(TokenKind::EQUAL)?;

        Ok(Node::LetStatement {
            name: name,
            value: Box::new(self.parse_expression()?),
        })
    }

    pub fn expect_and_consume_identifier(&mut self) -> Result<String, String> {
        let current = self.current()?;

        if let TokenKind::Identifier(name) = &current.kind {
            let name = name.clone();
            let _ = self.advance();
            Ok(name)
        } else {
            Err(format!("Expected identifier, found {:?}", current.kind))
        }
    }

    fn parse_block(&mut self) -> NodeResult {
        self.expect_and_consume(TokenKind::LBRACE)?;

        let mut body = vec![];

        loop {
            self.skip_new_lines();

            let next = match self.current() {
                Ok(tok) => tok,
                Err(_) => return Err("Unexpected end of input inside block".to_string()),
            };

            if next.kind == TokenKind::RBRACE {
                break;
            }

            body.push(self.parse()?);
        }

        self.expect_and_consume(TokenKind::RBRACE)?;

        Ok(Node::Block { body })
    }
}
