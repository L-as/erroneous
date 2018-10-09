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

fn to_number(input: char) -> Option<u8> {
	match input {
		'0'..='9' => Some((input as u32 - '0' as u32) as u8),
		_ => None,
	}
}

#[derive(Debug, Display, From, Error)]
enum MainError {
	#[display(fmt = "Data could not be parsed")]
	Parse(ParseError),
	#[display(fmt = "Parsed data '{}' is invalid", _0)]
	InvalidData(char),
}

fn main() -> Result<(), MainError> {
	let parsed = parse("fuaxna")?;
	assert!(parsed == 'a');
	assert!(to_number(parsed).ok_or(MainError::InvalidData(parsed))? == 2);
	Ok(())
}
