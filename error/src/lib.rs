#[derive(Debug, PartialEq)]
pub enum Error {
	LexerError(LexerError),
}

impl Default for Error {
	fn default() -> Self {
		Self::LexerError(LexerError::default())
	}
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
	UnexpectedToken,
}

impl Default for LexerError {
	fn default() -> Self {
		LexerError::UnexpectedToken
	}
}

impl std::error::Error for LexerError {}

impl std::fmt::Display for LexerError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
