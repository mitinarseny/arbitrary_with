mod adapters;
mod alloc;
mod core;
mod std;
pub use self::adapters::*;

use arbitrary::{MaxRecursionReached, Result, Unstructured};

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
