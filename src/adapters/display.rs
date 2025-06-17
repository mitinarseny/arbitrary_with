use core::{fmt::Display, marker::PhantomData};
use std::str::FromStr;

use arbitrary::{Arbitrary, Error, MaxRecursionReached, Result, Unstructured};

use crate::ArbitraryAs;

pub struct DisplayFromStr<T: ?Sized>(PhantomData<T>);

impl<'a, T, U> ArbitraryAs<'a, T> for DisplayFromStr<U>
where
    U: Arbitrary<'a> + Display,
    T: FromStr,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<T> {
        T::from_str(&U::arbitrary(u)?.to_string()).map_err(|_| Error::IncorrectFormat)
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<T> {
        T::from_str(&U::arbitrary_take_rest(u)?.to_string()).map_err(|_| Error::IncorrectFormat)
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        U::size_hint(depth)
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        U::try_size_hint(depth)
    }
}
