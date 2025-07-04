use std::collections::BinaryHeap;

use arbitrary::{Arbitrary, MaxRecursionReached, Result, Unstructured};

use crate::{ArbitraryAs, AsWrap, UnstructuredExt};

impl<'a, T, As> ArbitraryAs<'a, BinaryHeap<T>> for BinaryHeap<As>
where
    As: ArbitraryAs<'a, T>,
    T: Ord,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<BinaryHeap<T>> {
        u.arbitrary_iter_as::<T, As>()?.collect()
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<BinaryHeap<T>> {
        u.arbitrary_take_rest_iter_as::<T, As>()?.collect()
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        Self::try_size_hint_as(depth).unwrap_or_default()
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        BinaryHeap::<AsWrap<T, As>>::try_size_hint(depth)
    }
}
