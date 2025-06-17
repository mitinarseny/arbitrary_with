#![doc = include_str!("../README.md")]
//! ## Example
//! ```rust
//! # use arbitrary_with::{Arbitrary, As, DisplayFromStr, Error, Unstructured};
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

pub trait UnstructuredExt<'a> {
    fn arbitrary_as<T, As>(&mut self) -> Result<T>
    where
        As: ArbitraryAs<'a, T>;

    fn arbitrary_len_as<ElementType, ElementAs>(&mut self) -> Result<usize>
    where
        ElementAs: ArbitraryAs<'a, ElementType>;

    fn arbitrary_iter_as<ElementType, ElementAs>(
        &mut self,
    ) -> Result<impl Iterator<Item = Result<ElementType>>>
    where
        ElementAs: ArbitraryAs<'a, ElementType>;

    fn arbitrary_take_rest_iter_as<ElementType, ElementAs>(
        self,
    ) -> Result<impl Iterator<Item = Result<ElementType>>>
    where
        ElementAs: ArbitraryAs<'a, ElementType>;
}

impl<'a> UnstructuredExt<'a> for Unstructured<'a> {
    #[inline]
    fn arbitrary_as<T, As>(&mut self) -> Result<T>
    where
        As: ArbitraryAs<'a, T>,
    {
        self.arbitrary::<AsWrap<T, As>>().map(AsWrap::into_inner)
    }

    #[inline]
    fn arbitrary_len_as<ElementType, ElementAs>(&mut self) -> Result<usize>
    where
        ElementAs: ArbitraryAs<'a, ElementType>,
    {
        self.arbitrary_len::<AsWrap<ElementType, ElementAs>>()
    }

    #[inline]
    fn arbitrary_iter_as<ElementType, ElementAs>(
        &mut self,
    ) -> Result<impl Iterator<Item = Result<ElementType>>>
    where
        ElementAs: ArbitraryAs<'a, ElementType>,
    {
        Ok(self
            .arbitrary_iter::<AsWrap<ElementType, ElementAs>>()?
            .map(|r| r.map(AsWrap::into_inner)))
    }

    #[inline]
    fn arbitrary_take_rest_iter_as<ElementType, ElementAs>(
        self,
    ) -> Result<impl Iterator<Item = Result<ElementType>>>
    where
        ElementAs: ArbitraryAs<'a, ElementType>,
    {
        Ok(self
            .arbitrary_take_rest_iter::<AsWrap<ElementType, ElementAs>>()?
            .map(|r| r.map(AsWrap::into_inner)))
    }
}
