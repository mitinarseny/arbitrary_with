use arbitrary::{MaxRecursionReached, Result, Unstructured, size_hint};

use crate::ArbitraryAs;

macro_rules! impl_arbitrary_as_for_tuple {
    () => {};
    ($lastt:ident:$lasta:ident $($ts:ident:$as:ident)*) => {
        impl_arbitrary_as_for_tuple!($($ts:$as)*);

        impl<'a, $lastt, $lasta, $($ts, $as),*> ArbitraryAs<'a, ($($ts,)* $lastt,)> for ($($as,)* $lasta,)
        where $(
            $as: ArbitraryAs<'a, $ts>,)*
            $lasta: ArbitraryAs<'a, $lastt>,
        {
            #[inline]
            fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<($($ts,)* $lastt,)> {
                Ok(($($as::arbitrary_as(u)?,)* $lasta::arbitrary_as(u)?,))
            }

            #[allow(unused_mut)]
            #[inline]
            fn arbitrary_take_rest_as(mut u: Unstructured<'a>) -> Result<($($ts,)* $lastt,)> {
                Ok(($(
                    $as::arbitrary_as(&mut u)?,)*
                    $lasta::arbitrary_take_rest_as(u)?,
                ))
            }

            #[inline]
            fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
                Self::try_size_hint_as(depth).unwrap_or_default()
            }

            #[inline]
            fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
                Ok(size_hint::and_all(&[$(
                    $as::try_size_hint_as(depth)?,)*
                    $lasta::try_size_hint_as(depth)?,
                ]))
            }
        }
    };
}
impl_arbitrary_as_for_tuple!(
    T25:As25 T24:As24 T23:As23 T22:As22 T21:As21 T20:As20
    T19:As19 T18:As18 T17:As17 T16:As16 T15:As15 T14:As14 T13:As13 T12:As12 T11:As11 T10:As10
    T9:As9 T8:As8 T7:As7 T6:As6 T5:As5 T4:As4 T3:As3 T2:As2 T1:As1 T0:As0
);
