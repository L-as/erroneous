#[macro_use]
extern crate derive_more;
extern crate erroneous;

use erroneous::Error;

#[derive(Debug, Display, Error)]
#[display(fmt = "A")]
struct A;

#[derive(Debug, Display, Error)]
#[display(fmt = "B")]
struct B(#[error(source)] A);

#[test]
fn main() {
	let b = B(A);
	let _ = b.cause().unwrap();
}
