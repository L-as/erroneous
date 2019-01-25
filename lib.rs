extern crate erroneous_derive;
pub use erroneous_derive::Error;

use std::error::Error as StdError;

/// A "replacement" trait for [`std::error::Error`][StdError].
/// You should use this as a bound instead of that one,
/// since this one has more guarantees, although it is entirely
/// logically identical to `std::error::Error + Send + Sync + 'static`.
///
/// To "implement" this you should use the derive.
///
/// [StdError]: https://doc.rust-lang.org/std/error/trait.Error.html
pub trait Error: StdError + Send + Sync + 'static {
	/// Iterate over the entire chain of errors. Includes `self`.
	///
	/// Result implements `Iterator<Item = &(dyn std::error::Error + 'static)>`.
	fn iter<'a>(&'a self) -> Iter<'a>;
}

/// Returned by [Error::iter](trait.Error.html#tymethod.iter)
pub struct Iter<'a> {
	error: Option<&'a (dyn StdError + 'static)>,
}

impl<'a> Iterator for Iter<'a> {
	type Item = &'a (dyn StdError + 'static);

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(error) = self.error {
			self.error = error.source();
			Some(error)
		} else {
			None
		}
	}
}

impl<T: StdError + Send + Sync + 'static> Error for T {
	fn iter<'a>(&'a self) -> Iter<'a> {
		Iter { error: Some(self) }
	}
}
