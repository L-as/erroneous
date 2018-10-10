#[macro_use]
extern crate derive_more;
extern crate erroneous;

use erroneous::Error;

#[derive(Debug, Display, Error)]
#[display(fmt = "SomeError")]
struct SomeError;

#[derive(Debug, Display, Error)]
#[display(fmt = "E")]
enum E<Custom: Error> {
	A,
	Custom(#[error(source)] Custom),
}

#[test]
fn main() {
	let _ = E::A::<SomeError>;
	let b = E::Custom(SomeError);
	b.source().unwrap();
}
