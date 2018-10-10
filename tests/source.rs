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

#[derive(Debug, Display, Error)]
#[display(fmt = "C")]
struct C(#[error(source)] B);

#[test]
fn main() {
	let e = C(B(A));
	let e = e.source().unwrap();
	assert!(e.is::<B>());
	let e = e.source().unwrap();
	assert!(e.is::<A>());
	assert!(e.source().is_none());
}
