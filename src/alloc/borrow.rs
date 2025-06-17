use std::borrow::Cow;

use arbitrary::{Arbitrary, MaxRecursionReached, Result, Unstructured};

use crate::{ArbitraryAs, AsWrap};

impl<'a, T, As> ArbitraryAs<'a, Cow<'a, T>> for Cow<'a, As>
where
    As: ToOwned,
    T: Clone,
    As::Owned: ArbitraryAs<'a, T>,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<Cow<'a, T>> {
        <As::Owned>::arbitrary_as(u).map(Cow::Owned)
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<Cow<'a, T>> {
        <As::Owned>::arbitrary_take_rest_as(u).map(Cow::Owned)
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        Self::try_size_hint_as(depth).unwrap_or_default()
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        Cow::<'a, AsWrap<T, As::Owned>>::try_size_hint(depth)
    }
}
