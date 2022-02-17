use crate::cursor::Cursor;
pub use crate::token::Token;
use error::Error;

mod automatons;
mod cursor;
mod token;
mod traits;

type Result<T> = std::result::Result<T, Error>;

pub struct Lexer;

impl Lexer {
	pub fn tokenize<'a, I>(input: I) -> Result<impl Iterator<Item = Token>>
	where
		I: Iterator<Item = char>,
	{
		let mut buffer: Vec<Token> = vec![];
		let mut cursor = Cursor::new(input);

		loop {
			let token = cursor.next_token()?;
			if token.is_eof() {
				break;
			}
			buffer.push(token);
		}
		Ok(buffer.into_iter())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::token::*;

	#[test]
	fn hello_world() {
		let input = "
			function main() {
				print(\"hello world\");
			}
		";

		let output = Lexer::tokenize(input.chars());

		let expected = vec![
			Token::new(TokenKind::Keyword(Keyword::Function)),
			Token::new(TokenKind::Identifier("main".to_string())),
			Token::new(TokenKind::Symbol(Symbol::Group(GroupSymbols::LParent))),
			Token::new(TokenKind::Symbol(Symbol::Group(GroupSymbols::RParent))),
			Token::new(TokenKind::Symbol(Symbol::Group(GroupSymbols::LBrace))),
			Token::new(TokenKind::Identifier("print".to_string())),
			Token::new(TokenKind::Symbol(Symbol::Group(GroupSymbols::LParent))),
			Token::new(TokenKind::Literal(Literal::Str {
				value: "hello world".to_string(),
			})),
			Token::new(TokenKind::Symbol(Symbol::Group(GroupSymbols::RParent))),
			Token::new(TokenKind::Symbol(Symbol::Punctuation(
				PunctuationSymbols::Semicolon,
			))),
			Token::new(TokenKind::Symbol(Symbol::Group(GroupSymbols::RBrace))),
		];

		assert_eq!(output.unwrap().collect::<Vec<Token>>(), expected);
	}
}
