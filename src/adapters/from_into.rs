use core::marker::PhantomData;

use arbitrary::{Arbitrary, MaxRecursionReached, Result, Unstructured};

use crate::ArbitraryAs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FromInto<T>(PhantomData<T>);

impl<'a, T, U> ArbitraryAs<'a, T> for FromInto<U>
where
    U: Into<T> + Arbitrary<'a>,
{
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<T> {
        U::arbitrary(u).map(Into::into)
    }

    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<T> {
        U::arbitrary_take_rest(u).map(Into::into)
    }

    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        let _ = depth;
        (0, None)
    }

    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        Ok(Self::size_hint_as(depth))
    }
}
