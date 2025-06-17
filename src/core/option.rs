use arbitrary::{Arbitrary, MaxRecursionReached, Result, Unstructured};

use crate::{ArbitraryAs, AsWrap};

impl<'a, T, As> ArbitraryAs<'a, Option<T>> for Option<As>
where
    As: ArbitraryAs<'a, T>,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<Option<T>> {
        Ok(Option::<AsWrap<T, As>>::arbitrary(u)?.map(AsWrap::into_inner))
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<Option<T>> {
        Ok(Option::<AsWrap<T, As>>::arbitrary_take_rest(u)?.map(AsWrap::into_inner))
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        Self::try_size_hint_as(depth).unwrap_or_default()
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        Option::<AsWrap<T, As>>::try_size_hint(depth)
    }
}
