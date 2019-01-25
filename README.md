# erroneous - Helper for defining and using errors

[![Documentation](https://img.shields.io/readthedocs/pip.svg)](docs.rs/erroneous)

`erroneous` is a crate with two features:
- Its `Error` trait
- Its `Error` derive

# Example
```rust
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
```

# The `Error` trait
The `Error` trait is a supertrait of `std::error::Error`.
It is automatically implemented for all implementors of `std::error::Error`,
with some restrictions, particularly, it is `Send + Sync + 'static`. This means you have
more freedom when dealing with them, and can in addition downcast dyn Error
to concrete types.
In addition, the trait also includes a helper method to iterate the chain of errors called `iter`.

# The `Error` derive
This feature just implements `std::error::Error` (and thus also `erroneous::Error`) for you,
You can annotate a field in your input as `#[error(source)]` to make the `source` method
return that field.

# License

`erroneous` is licensed under the terms of the MIT License or the Apache License
2.0, at your choosing.
