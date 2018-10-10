#[allow(unused_imports)]
#[macro_use]
extern crate erroneous_derive;
pub use erroneous_derive::*;

use std::error::Error as StdError;

/// A "replacement" trait for std::error::Error.
/// You should use this as a bound instead of that one,
/// since this one has more guarantees.
pub trait Error: StdError + Send + Sync + 'static {
	fn iter<'a>(&'a self) -> Iter<'a>;

	fn source(&self) -> Option<&(dyn StdError + 'static)> {
		StdError::source(self)
	}
}

pub struct Iter<'a> {
	error: &'a dyn StdError,
}

impl<'a> Iterator for Iter<'a> {
	type Item = &'a(dyn StdError + 'static);

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(source) = self.error.source() {
			self.error = source;
			Some(source)
		} else {
			None
		}
	}
}

impl<T: StdError + Send + Sync + 'static> Error for T {
	fn iter<'a>(&'a self) -> Iter<'a> {
		Iter { error: self }
	}
}
