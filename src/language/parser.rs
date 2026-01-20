use crate::language::token::Token;

type TokenResult = Result<Token, String>;

#[derive(Debug, Clone)]
pub struct Parser {
	tokens: Vec<Token>,
	pos: i32,
	cur_token: Option<Token>
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		let empty = tokens.is_empty();
		let mut new_parser = Self {
			tokens,
			pos: -1,
			cur_token: None
		};

		if !empty {
			new_parser.advance().unwrap();
		}
		new_parser
	}

	pub fn advance(&mut self) -> TokenResult {
		self.pos += 1;

		if self.pos < self.tokens.len() as i32 {
			self.cur_token = Some(self.tokens[self.pos as usize]);
			Ok(self.tokens[self.pos as usize])
		} else {
			self.cur_token = None;
			Err("Expected more tokens. Got [EOF].".to_string())
		}
	}

	pub fn parse_expression(&mut self) -> TokenResult {
		todo!()
	}
}