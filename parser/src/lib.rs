use error::Error;
use lexer::Token;

type Result<T> = std::result::Result<T, Error>;

pub struct AST;

pub struct Parser;

impl Parser {
	pub fn parse<I: Iterator<Item = Token>>(_input: I) -> Result<AST> {
		todo!()
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
