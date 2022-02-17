use bytecode::Bytecode;
use error::Error;
use lexer::Lexer;
use parser::Parser;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub struct Interpreter;

impl Interpreter {
	pub fn run<'a>(input: &'a str) -> Result<Output> {
		Interpreter::interpret(Bytecode::generate(Parser::parse(Lexer::tokenize(
			input.chars(),
		)?)?)?)
	}

	fn interpret(_bytecode: Bytecode) -> Result<Output> {
		todo!()
	}
}

#[derive(Debug, PartialEq)]
pub struct Output {
	output: StdOut,
	last_state: State,
}

#[derive(Debug, PartialEq)]
pub struct StdOut {
	output: String,
}

#[derive(Debug, PartialEq)]
pub struct State;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn hello_world() {
		let input = "
			function main() {
				print(\"hello world\")
			}
		";

		let output = Interpreter::run(input);

		let expected = Output {
			last_state: State {},
			output: StdOut {
				output: String::from("hello world"),
			},
		};

		assert_eq!(output.unwrap(), expected);
	}

	#[test]
	fn basic_function() {
		let input = "
			function f() {
			    x = 1;
			    y = 2;
				return (x + 1) * y;
			}
			
			function main() {
			    f();
			}
		";

		let output = Interpreter::run(input);

		let expected = Output {
			last_state: State {},
			output: StdOut {
				output: String::new(),
			},
		};

		assert_eq!(output.unwrap(), expected);
	}
}
