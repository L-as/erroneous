extern crate derive_more;
extern crate erroneous;

use derive_more::Display;
use erroneous::Error;

#[derive(Debug, Display, Error)]
#[allow(dead_code)]
enum EmptyError {}
