use arbitrary::{Arbitrary, MaxRecursionReached, Result, Unstructured};

use crate::{ArbitraryAs, AsWrap};

impl<'a, T, As> ArbitraryAs<'a, Vec<T>> for Vec<As>
where
    As: ArbitraryAs<'a, T>,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<Vec<T>> {
        u.arbitrary_iter::<AsWrap<T, As>>()?
            .map(|r| r.map(AsWrap::into_inner))
            .collect()
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<Vec<T>> {
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
        Vec::<AsWrap<T, As>>::try_size_hint(depth)
    }
}
