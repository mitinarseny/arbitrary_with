use core::hash::Hash;
use std::collections::HashSet;

use arbitrary::{Arbitrary, MaxRecursionReached, Result, Unstructured};

use crate::{ArbitraryAs, AsWrap};

impl<'a, T, As> ArbitraryAs<'a, HashSet<T>> for HashSet<As>
where
    As: ArbitraryAs<'a, T>,
    T: Eq + Hash,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<HashSet<T>> {
        u.arbitrary_iter::<AsWrap<T, As>>()?
            .map(|r| r.map(AsWrap::into_inner))
            .collect()
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<HashSet<T>> {
        u.arbitrary_take_rest_iter::<AsWrap<T, As>>()?
            .map(|r| r.map(AsWrap::into_inner))
            .collect()
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        Self::try_size_hint_as(depth).unwrap_or_default()
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        HashSet::<AsWrap<T, As>>::try_size_hint(depth)
    }
}
