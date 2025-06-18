use std::collections::LinkedList;

use arbitrary::{Arbitrary, MaxRecursionReached, Result, Unstructured};

use crate::{ArbitraryAs, AsWrap, UnstructuredExt};

impl<'a, T, As> ArbitraryAs<'a, LinkedList<T>> for LinkedList<As>
where
    As: ArbitraryAs<'a, T>,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<LinkedList<T>> {
        u.arbitrary_iter_as::<T, As>()?.collect()
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<LinkedList<T>> {
        u.arbitrary_take_rest_iter_as::<T, As>()?.collect()
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        Self::try_size_hint_as(depth).unwrap_or_default()
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        LinkedList::<AsWrap<T, As>>::try_size_hint(depth)
    }
}
