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

    fn advance(&mut self) -> TokenResult {
        let current = self.current();
        self.pos += 1;

        current
    }

    #[allow(unused)]
    fn current(&self) -> TokenResult {
        if self.pos < self.tokens.len() as i32 {
            Ok(self.tokens[self.pos as usize].clone())
        } else {
            Err("Expected more tokens. Got [EOF]. (Current)".to_string())
        }
    }

    pub fn peek(&self) -> Option<Token> {
        if self.pos + 1 < self.tokens.len() as i32 {
            Some(self.tokens[(self.pos + 1) as usize].clone())
        } else {
            None
        }
    }

    fn expect_and_consume(&mut self, kind: TokenKind) -> Result<Token, String> {
        if let Ok(next) = self.current() {
            if next.kind != kind {
                Err(format!("Expected `{kind:?}`, got `{:?}`", next.kind))
            } else {
                self.advance()?;
                Ok(next)
            }
        } else {
            Err(format!("Expected `{kind:?}`, got [EOF]."))
        }
    }

    pub fn parse(&mut self) -> NodeResult {
        self.skip_new_lines();

        match self.current()?.kind {
            TokenKind::LET => self.parse_let(),
            TokenKind::LBRACE => self.parse_block(),
            TokenKind::FUNC => self.parse_function_def(),
            TokenKind::RETURN => self.parse_return(),
            TokenKind::BREAK => self.simple_parse_keyword(Node::BreakStatement),
            TokenKind::CONTINUE => self.simple_parse_keyword(Node::ContinueStatement),
            TokenKind::WHILE => self.parse_while(),
            TokenKind::FOR => self.parse_for(),
            TokenKind::IF => self.parse_if(),

            _ => {
                let expr = self.parse_expression()?;

                // FUNCTION CALL
                if let Ok(next) = self.current()
                    && next.kind == TokenKind::LPAREN
                {
                    return self.parse_function_call(expr);
                }

                Ok(expr)
            }
        }
    }
}

// EXPRESSIONS
impl Parser {
    fn parse_add_sub(&mut self) -> NodeResult {
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

    fn parse_mul_div(&mut self) -> NodeResult {
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

    fn parse_unary(&mut self) -> NodeResult {
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

    fn parse_exponent(&mut self) -> NodeResult {
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

    fn parse_primary(&mut self) -> NodeResult {
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

            TokenKind::Identifier => Ok(Node::Variable(current.get_text(&self.source).to_string())),

            // COLLECTIONS
            TokenKind::LPAREN => {
                self.advance()?;
                self.skip_new_lines();

                let expr = if self.current().is_ok() {
                    self.parse_expression()
                } else {
                    return Err("Unexpected end of input inside parentheses".to_string());
                };

                // DON'T PARSE THE `RPAREN` BECAUSE WE ALREADY ADVANCE LATER
                expr
            }
            TokenKind::LBRACK => self.parse_list(),

            other => Err(format!(
                "Got unexpected token `{other:?}` while parsing primary."
            )),
        };

        self.advance()?;

        node
    }

    fn skip_new_lines(&mut self) {
        while let Ok(next) = self.current() {
            if matches!(next.kind, TokenKind::NEWLINE) {
                self.advance().unwrap();
            } else {
                break;
            }
        }
    }

    fn parse_list(&mut self) -> NodeResult {
        self.advance()?;

        let mut values = vec![];

        loop {
            self.skip_new_lines();

            if let Ok(next) = self.current() {
                if next.kind == TokenKind::RBRACK {
                    break;
                }
            } else {
                return Err(format!(
                    "Unexpected end of input [EOF] while parsing list. Expected `]`."
                ));
            }

            values.push(self.parse_expression()?);

            if let Ok(next) = self.current()
                && next.kind == TokenKind::COMMA
            {
                self.advance()?;
            } else {
                break;
            }
        }

        // DONT PARSE THE `RBRACK` BECAUSE WE ALREADY ADVANCE LATER

        Ok(Node::ListNode(values))
    }

    fn parse_expression(&mut self) -> NodeResult {
        let expr = self.parse_logical();

        self.skip_new_lines();

        expr
    }

    fn parse_logical(&mut self) -> NodeResult {
        self.parse_or()
    }

    fn parse_or(&mut self) -> NodeResult {
        let mut left = self.parse_and()?;

        while let Ok(next) = self.current() {
            if next.kind != TokenKind::OR {
                break;
            }

            let op = self.advance()?.kind;
            let right = self.parse_and()?;

            left = Node::BinOp {
                left: Box::new(left),
                right: Box::new(right),
                op,
            };
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> NodeResult {
        let mut left = self.parse_equality()?;

        while let Ok(next) = self.current() {
            if next.kind != TokenKind::AND {
                break;
            }

            let op = self.advance()?.kind;
            let right = self.parse_equality()?;

            left = Node::BinOp {
                left: Box::new(left),
                right: Box::new(right),
                op,
            };
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> NodeResult {
        let mut left = self.parse_comparison()?;

        while let Ok(next) = self.current() {
            if !matches!(next.kind, TokenKind::EQUAL | TokenKind::NE) {
                break;
            }

            let op = self.advance()?.kind;
            let right = self.parse_comparison()?;

            left = Node::BinOp {
                left: Box::new(left),
                right: Box::new(right),
                op,
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> NodeResult {
        let mut left = self.parse_add_sub()?;

        while let Ok(next) = self.current() {
            if !matches!(
                next.kind,
                TokenKind::LT | TokenKind::LE | TokenKind::GR | TokenKind::GE
            ) {
                break;
            }

            let op = self.advance()?.kind;
            let right = self.parse_add_sub()?;

            left = Node::BinOp {
                left: Box::new(left),
                right: Box::new(right),
                op,
            };
        }

        Ok(left)
    }

    fn parse_function_call(&mut self, expr: Node) -> NodeResult {
        self.advance()?; // consume LPAREN

        let mut args = vec![];

        loop {
            self.skip_new_lines();

            let next = match self.current() {
                Ok(x) => x,
                Err(_) => {
                    return {
                        Err(format!(
                            "Unexpected end of input in function call. Expected `)`."
                        ))
                    };
                }
            };

            if next.kind == TokenKind::RPAREN {
                break;
            }

            args.push(self.parse_expression()?);

            if let Ok(next) = self.current()
                && next.kind == TokenKind::COMMA
            {
                self.advance()?;
                continue;
            } else {
                break;
            }
        }

        self.expect_and_consume(TokenKind::RPAREN)?;

        Ok(Node::FunctionCall {
            target: Box::new(expr),
            args,
        })
    }
}

// STATEMENTS
impl Parser {
    fn parse_let(&mut self) -> NodeResult {
        self.advance()?;

        let name = self
            .expect_and_consume(TokenKind::Identifier)?
            .get_text(&self.source)
            .to_string();

        self.expect_and_consume(TokenKind::EQUAL)?;

        Ok(Node::LetStatement {
            name: name,
            value: Box::new(self.parse_expression()?),
        })
    }

    fn parse_block(&mut self) -> NodeResult {
        self.expect_and_consume(TokenKind::LBRACE)?;

        let mut body = vec![];

        loop {
            self.skip_new_lines();

            let next = match self.current() {
                Ok(tok) => tok,
                Err(_) => return Err("Unexpected end of input inside block.".to_string()),
            };

            if next.kind == TokenKind::RBRACE {
                break;
            }

            body.push(self.parse()?);
        }

        self.expect_and_consume(TokenKind::RBRACE)?;

        Ok(Node::Block { body })
    }

    fn parse_function_def(&mut self) -> NodeResult {
        self.advance()?;
        self.skip_new_lines();

        let name = self
            .expect_and_consume(TokenKind::Identifier)?
            .get_text(&self.source)
            .to_string();

        self.skip_new_lines();
        self.expect_and_consume(TokenKind::LPAREN)?;

        let mut args = vec![];

        loop {
            self.skip_new_lines();

            if let Ok(next) = self.current() {
                if next.kind == TokenKind::RPAREN {
                    break;
                }
            } else {
                return Err(format!(
                    "Unexpected end of input while parsing function arguments."
                ));
            }

            let arg_name = self
                .expect_and_consume(TokenKind::Identifier)?
                .get_text(&self.source)
                .to_string();

            let arg_type = if let Ok(next) = self.current()
                && next.kind == TokenKind::COLON
            {
                self.expect_and_consume(TokenKind::COLON)?;
                Some(
                    self.expect_and_consume(TokenKind::Identifier)?
                        .get_text(&self.source)
                        .to_string(),
                )
            } else {
                None
            };

            args.push((arg_name, arg_type));

            self.skip_new_lines();
            if let Ok(next) = self.current()
                && next.kind == TokenKind::COMMA
            {
                self.advance()?;
            } else {
                break;
            }
        }

        self.expect_and_consume(TokenKind::RPAREN)?;

        let return_type = if let Ok(next) = self.current()
            && next.kind == TokenKind::ARROW
        {
            self.advance()?;
            Some(
                self.expect_and_consume(TokenKind::Identifier)?
                    .get_text(&self.source)
                    .to_string(),
            )
        } else {
            None
        };

        Ok(Node::FunctionDefinition {
            name,
            args,
            return_type,
            block: Box::new(self.parse_block()?),
        })
    }

    fn parse_return(&mut self) -> NodeResult {
        self.advance()?;

        if let Ok(next) = self.current()
            && !matches!(next.kind, TokenKind::NEWLINE | TokenKind::SEMI)
        {
            Ok(Node::ReturnStatement(Some(Box::new(
                self.parse_expression()?,
            ))))
        } else {
            Ok(Node::ReturnStatement(None))
        }
    }

    fn parse_if(&mut self) -> NodeResult {
        self.advance()?;

        let condition = self.parse_expression()?;
        let main_block = self.parse_block()?;
        let mut elifs = vec![];
        let mut else_block = None;

        loop {
            if let Ok(next) = self.current()
                && next.kind == TokenKind::ELSE
            {
                self.advance()?;

                if let Ok(next) = self.current()
                    && next.kind == TokenKind::IF
                {
                    self.advance()?;

                    let elif_condition = self.parse_expression()?;
                    let elif_block = self.parse_block()?;

                    elifs.push((elif_condition, elif_block));
                } else {
                    else_block = Some(Box::new(self.parse_block()?));
                }
            } else {
                break;
            }
        }

        Ok(Node::IfStatement {
            condition: Box::new(condition),
            block: Box::new(main_block),
            elifs,
            else_block,
        })
    }

    /// Just advance and return whatever you want.
    fn simple_parse_keyword(&mut self, node: Node) -> NodeResult {
        self.advance()?;
        Ok(node)
    }

    fn parse_while(&mut self) -> NodeResult {
        self.advance()?;

        Ok(Node::WhileLoop {
            condition: Box::new(self.parse_expression()?),
            block: Box::new(self.parse_block()?),
        })
    }

    fn parse_for(&mut self) -> NodeResult {
        self.advance()?;

        /*
        for x = 1, 10 {

        }
         */

        let var_name = self
            .expect_and_consume(TokenKind::Identifier)?
            .get_text(&self.source)
            .to_string();

        let for_handle_type = self.advance()?;

        if for_handle_type.kind == TokenKind::EQUAL {
            let start = self.parse_expression()?;
            self.expect_and_consume(TokenKind::COMMA)?;
            let end = self.parse_expression()?;

            return Ok(Node::RangedForLoop {
                var_name,
                start: Box::new(start),
                end: Box::new(end),

                // optional step parameter
                step: if let Ok(next) = self.current()
                    && next.kind == TokenKind::COMMA
                {
                    Some(Box::new(self.parse_expression()?))
                } else {
                    None
                },
            });
        } else if for_handle_type.kind == TokenKind::COLON {
            let iterable = self.parse_expression()?;

            return Ok(Node::IterableForLoop {
                var_name,
                iterable: Box::new(iterable),
            });
        } else {
            Err("Expected equals (=) or colon (:) after variable name.".to_string())
        }
    }
}
