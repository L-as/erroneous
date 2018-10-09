#[allow(unused_imports)]
#[macro_use]
extern crate erroneous_derive;
pub use erroneous_derive::*;

use std::error::Error as StdError;

/// A "replacement" trait for std::error::Error.
/// You should use this as a bound instead of that one,
/// since this one has more guarantees.
pub trait Error: StdError + Send + Sync + Sized + 'static {
	fn iter<'a>(&'a self) -> Iter<'a> {
		Iter { error: self }
	}

	fn cause(&self) -> Option<&StdError> {
		StdError::cause(self)
	}
}

pub struct Iter<'a> {
	error: &'a dyn StdError,
}

impl<'a> Iterator for Iter<'a> {
	type Item = &'a dyn StdError;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(cause) = self.error.cause() {
			self.error = cause;
			Some(cause)
		} else {
			None
		}
	}
}

impl<T: StdError + Send + Sync + 'static> Error for T {}
