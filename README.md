% erroneous - Helper for defining and using errors

[![Documentation](https://img.shields.io/readthedocs/pip.svg)](docs.rs/erroneous)

`erroneous` is a crate with two features:
- Its `Error` trait
- Its `Error` derive

# The `Error` trait
The `Error` trait is a supertrait of `std::error::Error`.
It is automatically implemented for all implementors of `std::error::Error`,
with some restrictions.
Particularly, it is `Send + Sync + 'static`. This means you have
more freedom when dealing with them, and can in addition downcast dyn Error
to concrete types.

# The `Error` derive
This feature just implements `std::error::Error` (and thus also `erroneous::Error`) for you,
You can annotate a field in your input as `#[error(source)]` to make the `cause` method
return that field (and in the future the `source` method).

# Guidelines

## Scope
Your error type should not be able to represent errors that can
never happen, unless of course for backward compatibility.
This means you shouldn't just use a single error type for your entire crate,
because not all functions can return all errors.

## Information
Your error type should provide two types of information: developer-oriented
information and end-user-oriented information.

You should have descriptive error messages, that include important (but not obvious)
information. This means in a parsing function, your error probably shouldn't return
the input parsed.

In addition, your message should help your user fix the problem.
However, this also applies not just to the end-user, but also to the **developer**.

The developer should have a way to programmatically handle errors, and that doesn't
mean that the developer should use an English parsing library to parse your error
message. No, it means that you should provide clear error types (e.g. through enum variants)
that are documented.

Let's say you're writing a file system, and you have an `open_file` function.
If the file does not exist, it should return an error that indicates that the
file does not exist, then the program can e.g. create that file.

# Examples

Check the examples subdirectory.

# Backtraces

Backtraces are not an essential part of errors. You should be able to tell
what happened and why without needing a backtrace, you have to remember:
an end-user of your program should also be able to tell why an error is happening,
and shouldn't have to turn on RUST_BACKTRACE and delve into the depths of your program.

However, you may have inadequately designed your error handling, especially in those
cases where the error shouldn't actually be happening and is a bug, and for that reason,
it is important when debugging, but should absolutely not be required in other cases.

This is why it's desirable for backtraces to be available in `erroneous`, however,
I am waiting until the new Error trait is stabilized, since that will provide it.

# License

`erroneous` is licensed under the terms of the MIT License or the Apache License
2.0, at your choosing.
