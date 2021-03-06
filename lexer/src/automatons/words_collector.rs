use error::Error;
use crate::token::Token;
use crate::traits::{Collector, Cursor};

pub struct WordsCollector;

#[derive(Debug, PartialEq)]
pub enum State {
	Initial,
	Collecting,
	StopCollecting,
}

impl<'a, T, I> Collector<'a, T, I, Token> for WordsCollector
where
	I: Iterator<Item = char>,
	T: Cursor<I>,
{
	type State = State;
	type Error = Error;

	fn collect(cursor: &mut T) -> Result<Token, Self::Error> {
		let mut state = State::Initial;
		let mut buffer = String::new();

		while state != State::StopCollecting {
			match cursor.current() {
				Some(c) => match c {
					'_' | 'A'..='Z' | 'a'..='z' => match state {
						State::Initial | State::Collecting => {
							if state == State::Initial {
								state = State::Collecting;
							}
							cursor.push_buffer(&mut buffer, c);
						}
						_ => {}
					},
					'0'..='9' => match state {
						State::Collecting => cursor.push_buffer(&mut buffer, c),
						_ => return Err(Self::Error::default()),
					},
					_ => match state {
						State::Initial => {
							cursor.consume();
							return Err(Self::Error::default());
						}
						_ => state = State::StopCollecting,
					},
				},
				_ => state = State::StopCollecting,
			}
		}
		Ok(Token::get(buffer))
	}
}
