use error::Error;
use parser::AST;

type Result<T> = std::result::Result<T, Error>;

pub struct Bytecode;

impl Bytecode {
	pub fn generate(_input: AST) -> Result<Bytecode> {
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
