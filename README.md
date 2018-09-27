# erroneous - Minimalistic helper for using errors

`erroneous` is an opinionated (albeit minimalistic) library that helps you with using errors.

Check the [documentation](docs.rs/erroneous)

`erroneous`'s most important item is its `Error` trait which you can **not** implement manually.
It is a supertrait of the standard library's error trait, but with extra guarantees.
That means it's `Send`, `Sync`, and also `'static`. This means you can pass it around
without any restrictions, and also downcast `dyn Error` to concrete types.

## How to use

Errors in Rust exist primarily for use with the `Result` generic struct.
You have to keep this in mind when designing your errors.

### What errors should my functions return?

You may be tempted to just return a `Box<Error>` or `failure::Error`, but that
is in my opinion the wrong way to go about it. It means that users of the API
have **no idea whatsoever** about why it can fail, they can only inspect
the actual failure.

This is what I would call bad design.

Your functions should return an error type that contains exactly the errors
that function can cause, and no more (unless for backward compatibility).

If you are making a parser, and you have a `parse` function, that function should
return a `ParseError` enum that describes the possible errors. It is also
possible to use a `ParseError` and `ParseErrorKind` pair, but that is less ergonomic.

You should not do that unless needed.

An example of this may be:
```rust
use std::error::Error as StdError;

pub enum ParseError {
	UnexpectedNumber(u64),
}

// NB: trait implementations omitted
```

If you have a function called `parse_from_file` which takes a path as argument,
that function should **not** return a `ParseError` too, which forces you
to include the possibility of an IO error in your `ParseError` error type,
even though the `parse` function can't generate IO errors!

This is again, bad design.

You should instead make **another** (yes I know it's tedious) error type
that can be either an IO error or a parsing error.

Thankfully, there's an easy way in this case:

```rust
use std::io::Error as IoError;
use std::path::Path;
use either::Either;

pub fn parse_from_file(path: Path) -> Result<Something, Either<IoError, ParseError>> {
	...
}
```

That's right, you can use Either! You **can** chain this ad infinitum, but you should
probably just make another enum if your function is so complex that it can return
that many types of errors.

In this case you can use the `derive_more` crate.
Then you do:

```rust
#[derive(From)]
pub enum ParseFileError {
	Parse(ParseError),
	Io(IoError),
}
```

This means `From` is automatically implemented for each of the error types,
which means you can also save characters in the definition, since you can
just use `?` alone, instead of `map_err` stuff.

### What information should my error provide?

Errors in addition to encoding the possibility of failure, also need to provide
*information* to help one fix the cause of the error.
A good example is reading a file with `fs::read`:
It tells you why it couldn't read it, if it errs, but it
doesn't tell you what path you provided it, and this makes sense.

Sure, you may think that it's inconvenient and insensible that it doesn't provide this
information, but what if you just want the error without the path you supplied it?

Perhaps you're implementing a virtual filesystem where each path corresponds
to an actual path on the filesystem, then it would make no sense to show the user
the actual path read, if the user can't use that information.

A good rule is to only supply information that the user wouldn't be able to know.

### How should I define my errors?

I would recommend using a non-exhaustive enum (either with the `#[nonexhaustive]`
attribute or using a special `#[doc(hidden)] __Nonexhaustive` member.

How each variant is defined depends on what it needs to Display itself,
and also if it's caused by another error.

If it's caused by another error, it should contain that other error.
If it needs some extra information, e.g. a path, the variant
should also contain that.

You may want to use the `derive_more` crate to automatically implement `From` automatically.

You should then implement `std::error::Error` and its cause method (and that method only).

#### What should my Display and Debug implementations be?

Your debug implementation should probably just be the automatically derived one,
but your display implementation should describe the error **without** displaying
the cause.

This is so that each error doesn't use a different style for displaying its
causes, and so that the end-user can customise how to print it. (each on a different line?
indentation? etc. etc.)

#### How should I generate errors inside my code?

There aren't any helpers for this, since there isn't any catch-all approach.
What you should do is just use `Result::map_err` for errors that have a cause,
and just do `return Err(MyError(some_var))` for those that don't have a cause.

#### Examples

Check the examples subdirectory.

## Backtraces

Backtraces are IMHO not an essential part of errors. You should be able to tell
what happened and why without needing a backtrace, you have to remember:
an end-user of your program should also be able to tell why an error is happening,
and shouldn't have to turn on RUST_BACKTRACE and delve into the depths of your program.

However, you may not have adequately designed your error handling, especially in those
cases where the error shouldn't actually be happening and is a bug, and for that reason,
it is important when debugging, but should absolutely not be required in other cases.

This is why it's desirable for backtraces to be available in `erroneous`, however,
I am waiting until the new Error trait is stabilized, since that will provide it.

## License

`erroneous` is licensed under the terms of the MIT License or the Apache License
2.0, at your choosing.
