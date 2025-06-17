use arbitrary::{Arbitrary, MaxRecursionReached, Result, Unstructured};

use crate::{ArbitraryAs, AsWrap};

impl<'a, OkT, OkAs, ET, EAs> ArbitraryAs<'a, Result<OkT, ET>> for Result<OkAs, EAs>
where
    OkAs: ArbitraryAs<'a, OkT>,
    EAs: ArbitraryAs<'a, ET>,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<Result<OkT, ET>> {
        Ok(Result::<AsWrap<OkT, OkAs>, AsWrap<ET, EAs>>::arbitrary(u)?
            .map(AsWrap::into_inner)
            .map_err(AsWrap::into_inner))
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<Result<OkT, ET>> {
        Ok(
            Result::<AsWrap<OkT, OkAs>, AsWrap<ET, EAs>>::arbitrary_take_rest(u)?
                .map(AsWrap::into_inner)
                .map_err(AsWrap::into_inner),
        )
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        Self::try_size_hint_as(depth).unwrap_or_default()
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        Result::<AsWrap<OkT, OkAs>, AsWrap<ET, EAs>>::try_size_hint(depth)
    }
}
