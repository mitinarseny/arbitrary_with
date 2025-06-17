#![doc = include_str!("../README.md")]
//! ## Example
//! ```rust
//! # use arbitrary::{Arbitrary, Error, Unstructured};
//! # use arbitrary_with::{As, DisplayFromStr};
//! #[derive(Arbitrary)]
//! struct A {
//!     #[arbitrary(with = As::<Option<DisplayFromStr<i32>>>::arbitrary)]
//!     n: Option<String>,
//! };
//!
//! # fn main() -> Result<(), Error> {
//! # let mut u = Unstructured::new(&[1, 2, 3]);
//! let a: A = u.arbitrary()?;
//! if let Some(n) = a.n {
//!     let _i: i32 = n.parse().unwrap();
//! }
//! # Ok(())
//! # }
//! ```

mod adapters;
mod alloc;
mod core;
mod std;
pub use self::adapters::*;

pub use arbitrary::{self, *};

pub trait ArbitraryAs<'a, T> {
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<T>;

    #[inline]
    fn arbitrary_take_rest_as(mut u: Unstructured<'a>) -> Result<T> {
        Self::arbitrary_as(&mut u)
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        let _ = depth;
        (0, None)
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        Ok(Self::size_hint_as(depth))
    }
}
