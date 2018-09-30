# erroneous - Minimalistic helper for using errors

`erroneous` is an opinionated (albeit minimalistic) library that helps you with using errors.

Check the [documentation](docs.rs/erroneous)

`erroneous`'s most important item is its `Error` trait.
It is a supertrait of the standard library's error trait, but with extra guarantees.
That means it's `Send`, `Sync`, and also `'static`. This means you can pass it around
without any restrictions, and also downcast `dyn Error` to concrete types.

## How to use

### What errors should my functions return?

A function should precisely specify how it can err.
This is best accomplished by having an enum representing the types
of errors that can occur, optionally non-exhaustive.

An example for a parser is:
```rust
#[macro_use]
extern crate erroneous;
#[macro_use]
extern crate derive_more;

#[derive(Debug, Display, Error)]
pub enum ParseError {
	#[display(fmt = "Unexpected number {} found in input", "_0")]
	UnexpectedNumber(u64),
}

pub fn parse(input: Input) -> Result<Output, ParseError> {
	...
}
```

If you want to return errors that are caused by other errors, you can do this:

```rust#[macro_use]
extern crate erroneous;
#[macro_use]
extern crate derive_more;

use parse::{ParseError, parse};

#[derive(Debug, Display, From, Error)]
pub enum UtilizeError {
	#[display(fmt = "Input could not be parsed")]
	ParseError(#[error(cause)] ParseError),
	#[display(fmt = "Input was incorrect")]
	BadInput,
}

pub fn utilize(input: Input) -> Result<(), UtilizeError> {
	let output = parse(input)?;
	...
}
```

### What information should my error provide?

Errors in addition to encoding the possibility of failure, also need to provide
*information* to help one fix the cause of the error.

However it should not provide information that is unnecessary and obvious,
e.g. a parsing function should not say "Parsing of <provided input> failed",
since the caller already *knows* that.
They know what function they're calling and what input they've provided.

#### What should my error `Display`?

Your error should display why it occurred (per the rules above), but it
should not display the cause, if any, since the logging facility
will want to format causes itself.

#### How should I generate errors inside my code?

There aren't any helpers for this, since there isn't any catch-all approach.
What you should do is just use `Result::map_err` for errors that have a cause,
and just do `return Err(MyError(some_var))` for those that don't have a cause.

#### Examples

Check the examples subdirectory.

## Backtraces

Backtraces are not an essential part of errors. You should be able to tell
what happened and why without needing a backtrace, you have to remember:
an end-user of your program should also be able to tell why an error is happening,
and shouldn't have to turn on RUST_BACKTRACE and delve into the depths of your program.

However, you may have inadequately designed your error handling, especially in those
cases where the error shouldn't actually be happening and is a bug, and for that reason,
it is important when debugging, but should absolutely not be required in other cases.

This is why it's desirable for backtraces to be available in `erroneous`, however,
I am waiting until the new Error trait is stabilized, since that will provide it.

## License

`erroneous` is licensed under the terms of the MIT License or the Apache License
2.0, at your choosing.
