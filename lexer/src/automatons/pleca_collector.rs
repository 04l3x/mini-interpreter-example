use crate::token::Token;
use crate::traits::{Collector, Cursor};
use error::Error;

pub struct PlecaCollector;

#[derive(Debug, PartialEq)]
pub enum State {
	Initial,
	Pleca,
	Or,
	Finished,
}

impl<'a, T, I> Collector<'a, T, I, Token> for PlecaCollector
where
	I: Iterator<Item = char>,
	T: Cursor<I>,
{
	type State = State;
	type Error = Error;

	fn collect(cursor: &mut T) -> Result<Token, Self::Error> {
		let mut state = State::Initial;
		let mut buffer = String::new();

		while state != State::Finished {
			match state {
				State::Initial => match cursor.current() {
					Some(c) => match c {
						'|' => {
							state = State::Pleca;
							cursor.push_buffer(&mut buffer, c);
						}
						_ => {}
					},
					_ => {}
				},

				State::Pleca => match cursor.current() {
					Some(c) => match c {
						'|' => {
							state = State::Or;
							cursor.push_buffer(&mut buffer, c);
						}
						_ => return Err(Self::Error::default()),
					},
					_ => return Err(Self::Error::default()),
				},

				State::Or => state = State::Finished,
				_ => {}
			}
		}
		Ok(Token::get(buffer))
	}
}
