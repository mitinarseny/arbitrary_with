use std::collections::VecDeque;

use arbitrary::{Arbitrary, MaxRecursionReached, Result, Unstructured};

use crate::{ArbitraryAs, AsWrap, UnstructuredExt};

impl<'a, T, As> ArbitraryAs<'a, VecDeque<T>> for VecDeque<As>
where
    As: ArbitraryAs<'a, T>,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<VecDeque<T>> {
        u.arbitrary_iter_as::<T, As>()?.collect()
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<VecDeque<T>> {
        u.arbitrary_take_rest_iter_as::<T, As>()?.collect()
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        Self::try_size_hint_as(depth).unwrap_or_default()
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        VecDeque::<AsWrap<T, As>>::try_size_hint(depth)
    }
}
