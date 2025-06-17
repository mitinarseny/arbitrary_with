use core::array;

use arbitrary::{MaxRecursionReached, Result, Unstructured, size_hint};

use crate::ArbitraryAs;

impl<'a, T, As, const N: usize> ArbitraryAs<'a, [T; N]> for [As; N]
where
    As: ArbitraryAs<'a, T>,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<[T; N]> {
        // TODO: replace with [`core::array::try_from_fn`](https://github.com/rust-lang/rust/issues/89379) when stabilized
        array_util::try_from_fn(|_i| As::arbitrary_as(u))
    }

    #[inline]
    fn arbitrary_take_rest_as(mut u: Unstructured<'a>) -> Result<[T; N]> {
        let mut array = Self::arbitrary_as(&mut u)?;
        if let Some(last) = array.last_mut() {
            *last = As::arbitrary_take_rest_as(u)?;
        }
        Ok(array)
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        Self::try_size_hint_as(depth).unwrap_or_default()
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        let hint = As::try_size_hint_as(depth)?;
        Ok(size_hint::and_all(&array::from_fn::<_, N, _>(|_| hint)))
    }
}
