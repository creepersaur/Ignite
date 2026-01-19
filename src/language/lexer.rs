use crate::language::token::{Token, TokenKind::{self, *}, TokenRange};

const PUNCTUATION: &str = "!@#$%^&*()_+[]{}|;,./<>?\n";

#[derive(Debug)]
pub struct Lexer {
    chars: Vec<char>,
    pos: i32,
    cur_char: Option<char>,
}

impl Lexer {
    pub fn new(text: &str) -> Self {
        let mut new_lexer = Self {
            chars: text.chars().collect(),
            pos: -1,
            cur_char: None,
        };

        new_lexer.advance();
        new_lexer
    }

    pub fn advance(&mut self) {
        self.pos += 1;

        if self.pos < self.chars.len() as i32 {
            self.cur_char = Some(self.chars[self.pos as usize])
        } else {
            self.cur_char = None
        }
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
		let mut instr = None;

        loop {
            if self.cur_char.is_none() {
                break;
            }

            // Skip tabs and spaces
            while let Some(c) = self.cur_char {
                if c == '\t' || c == ' ' {
                    self.advance();
                } else {
                    break;
                }
            }

            let start_pos = self.pos;
            let mut current_token = String::new();

            while let Some(c) = self.cur_char {
                if instr.is_none() && (c == '\t' || c == ' ') {
                    break;
                }

                if instr.is_none() && PUNCTUATION.contains(c) {
                    if !current_token.is_empty() {
                        tokens.push(Self::identify(&current_token, start_pos));
                    }

                    let mut buf = [0u8; 4];
                    let s = c.encode_utf8(&mut buf);
                    tokens.push(Self::identify(s, self.pos));

                    current_token.clear();
                    break;
                } else {
                    current_token.push(c);

					if c == '"' || c == '\'' {
						if let Some(s) = instr {
							if s == c {
								instr = None;
							}
						} else {
							instr = Some(c);
						}
					}
                }

                self.advance();
            }

            if !current_token.is_empty() {
                tokens.push(Self::identify(&current_token, start_pos));
            }

            self.advance();
        }

        tokens
    }

    pub fn identify(text: &str, start: i32) -> Token {
        let start = start as usize;
        let end = start + text.len();

        let kind = match text {
            "\n" => NEWLINE,

			// Keywords
            "let" => LET,
			"func" => FUNC, 
            "return" => RETURN,
			"for" => FOR,

            // Punctuation
            "(" => LPAREN, // Parenthesis ()
            ")" => RPAREN,
            "[" => LBRACK, // Brackets []
            "]" => RBRACK,
            "{" => LBRACE, // Braces {}
            "}" => RBRACE,
            "+" => PLUS,
            "-" => MINUS,
            "*" => MUL,
            "/" => DIV,
            "%" => MOD,
            "^" => POW,
            "$" => DOLLAR,
            "#" => HASH,
            "@" => AT,
            "!" => BANG,
            "=" => EQUAL,
            ">" => GR,
            "<" => LT,
            ">=" => GE,
            "<=" => LE,
            ":" => COLON,
            ";" => SEMI,
            "?" => QUESTION,
            "~" => TILDA,
            "`" => BACKTICK,
            "|" => PIPE,

            _ => Self::identify_other(text),
        };

        Token::new(kind, TokenRange { start, end })
    }

	pub fn identify_other(text: &str) -> TokenKind {
		if text.parse::<i32>().is_ok() {
			return IntLiteral
		} else if text.parse::<f32>().is_ok() {
			return FloatLiteral
		} else if text == "true" || text == "false" {
			return BooleanLiteral
		} else if text.starts_with('"') && text.ends_with('"') {
			return StringLiteral
		} else if text.starts_with('\'') && text.ends_with('\'') {
			return CharLiteral
		}

		Identifier
	}
}
