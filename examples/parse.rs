#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate erroneous;

#[derive(Debug, Display, Error)]
enum ParseError {
	#[display(fmt = "Found an unexpected 'a' in the input")]
	UnexpectedA,
	#[display(fmt = "Found an unexpected 'b' in the input")]
	UnexpectedB,
	#[display(fmt = "Input was empty")]
	Empty,
}

fn parse(input: &str) -> Result<char, ParseError> {
	let mut last = None;
	for c in input.chars() {
		match c {
			'a' => return Err(ParseError::UnexpectedA),
			'b' => return Err(ParseError::UnexpectedB),
			c => last = Some(c),
		}
	}
	last.ok_or(ParseError::Empty)
}

#[derive(Debug, Display, Error)]
#[display(
	fmt = "Input '{}' could not be converted to a character",
	"_0"
)]
struct ToNumberError(char);

#[derive(Debug, Display, From, Error)]
#[display(fmt = "Fatal error encountered")]
enum MainError {
	Parse(ParseError),
	ToNumber(ToNumberError),
}

fn main() -> Result<(), MainError> {
	assert!(parse("fuaxna")? == 'a');
	Ok(())
}
