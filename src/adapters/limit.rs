use std::{
    collections::{BTreeSet, BinaryHeap, HashSet, VecDeque},
    hash::Hash,
    marker::PhantomData,
};

use arbitrary::{Arbitrary, MaxRecursionReached, Result, Unstructured};

use crate::{ArbitraryAs, AsWrap, Same, UnstructuredExt};

pub struct LimitLen<const MAX: usize, As: ?Sized = Same>(PhantomData<As>);

impl<'a, T, As, const MAX: usize> ArbitraryAs<'a, Vec<T>> for LimitLen<MAX, As>
where
    As: ArbitraryAs<'a, T>,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<Vec<T>> {
        u.arbitrary_iter_as::<T, As>()?.take(MAX).collect()
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<Vec<T>> {
        u.arbitrary_take_rest_iter_as::<T, As>()?
            .take(MAX)
            .collect()
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        Vec::<AsWrap<T, As>>::size_hint(depth)
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        Vec::<AsWrap<T, As>>::try_size_hint(depth)
    }
}

impl<'a, T, As, const MAX: usize> ArbitraryAs<'a, Box<[T]>> for LimitLen<MAX, As>
where
    As: ArbitraryAs<'a, T>,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<Box<[T]>> {
        u.arbitrary_iter_as::<T, As>()?.take(MAX).collect()
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<Box<[T]>> {
        u.arbitrary_take_rest_iter_as::<T, As>()?
            .take(MAX)
            .collect()
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        Box::<[AsWrap<T, As>]>::size_hint(depth)
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        Box::<[AsWrap<T, As>]>::try_size_hint(depth)
    }
}

impl<'a, T, As, const MAX: usize> ArbitraryAs<'a, VecDeque<T>> for LimitLen<MAX, As>
where
    As: ArbitraryAs<'a, T>,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<VecDeque<T>> {
        u.arbitrary_iter_as::<T, As>()?.take(MAX).collect()
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<VecDeque<T>> {
        u.arbitrary_take_rest_iter_as::<T, As>()?
            .take(MAX)
            .collect()
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        VecDeque::<AsWrap<T, As>>::size_hint(depth)
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        VecDeque::<AsWrap<T, As>>::try_size_hint(depth)
    }
}

impl<'a, T, As, const MAX: usize> ArbitraryAs<'a, BTreeSet<T>> for LimitLen<MAX, As>
where
    As: ArbitraryAs<'a, T>,
    T: Ord,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<BTreeSet<T>> {
        u.arbitrary_iter_as::<T, As>()?.take(MAX).collect()
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<BTreeSet<T>> {
        u.arbitrary_take_rest_iter_as::<T, As>()?
            .take(MAX)
            .collect()
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        BTreeSet::<AsWrap<T, As>>::size_hint(depth)
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        BTreeSet::<AsWrap<T, As>>::try_size_hint(depth)
    }
}

impl<'a, T, As, const MAX: usize> ArbitraryAs<'a, BinaryHeap<T>> for LimitLen<MAX, As>
where
    As: ArbitraryAs<'a, T>,
    T: Ord,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<BinaryHeap<T>> {
        u.arbitrary_iter_as::<T, As>()?.take(MAX).collect()
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<BinaryHeap<T>> {
        u.arbitrary_take_rest_iter_as::<T, As>()?
            .take(MAX)
            .collect()
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        BinaryHeap::<AsWrap<T, As>>::size_hint(depth)
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        BinaryHeap::<AsWrap<T, As>>::try_size_hint(depth)
    }
}

impl<'a, T, As, const MAX: usize> ArbitraryAs<'a, HashSet<T>> for LimitLen<MAX, As>
where
    As: ArbitraryAs<'a, T>,
    T: Hash + Eq,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<HashSet<T>> {
        u.arbitrary_iter_as::<T, As>()?.take(MAX).collect()
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<HashSet<T>> {
        u.arbitrary_take_rest_iter_as::<T, As>()?
            .take(MAX)
            .collect()
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        HashSet::<AsWrap<T, As>>::size_hint(depth)
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        HashSet::<AsWrap<T, As>>::try_size_hint(depth)
    }
}

#[rustversion::since(1.87)]
#[allow(clippy::incompatible_msrv)]
const _: () = {
    /// Copied from https://github.com/rust-fuzz/arbitrary/blob/334351140a0dcea63bc28dd5c57fab19762f6b1d/src/foreign/core/str.rs#L6-L22
    fn arbitrary_str<'a>(u: &mut Unstructured<'a>, size: usize) -> Result<&'a str> {
        match str::from_utf8(u.peek_bytes(size).unwrap()) {
            Ok(s) => {
                u.bytes(size).unwrap();
                Ok(s)
            }
            Err(e) => {
                let i = e.valid_up_to();
                let valid = u.bytes(i).unwrap();
                let s = unsafe {
                    debug_assert!(str::from_utf8(valid).is_ok());
                    str::from_utf8_unchecked(valid)
                };
                Ok(s)
            }
        }
    }

    impl<'a, const MAX: usize> ArbitraryAs<'a, &'a str> for LimitLen<MAX> {
        #[inline]
        fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<&'a str> {
            let size = u.arbitrary_len::<u8>()?.min(MAX);
            arbitrary_str(u, size)
        }

        #[inline]
        fn arbitrary_take_rest_as(mut u: Unstructured<'a>) -> Result<&'a str> {
            let size = u.len().min(MAX);
            arbitrary_str(&mut u, size)
        }

        #[inline]
        fn size_hint_as(_depth: usize) -> (usize, Option<usize>) {
            (0, None)
        }
    }

    impl<'a, const MAX: usize> ArbitraryAs<'a, String> for LimitLen<MAX> {
        #[inline]
        fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<String> {
            <Self as ArbitraryAs<'a, &'a str>>::arbitrary_as(u).map(Into::into)
        }

        #[inline]
        fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<String> {
            <Self as ArbitraryAs<'a, &'a str>>::arbitrary_take_rest_as(u).map(Into::into)
        }

        #[inline]
        fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
            <Self as ArbitraryAs<'a, &'a str>>::size_hint_as(depth)
        }
    }
};

#[rustversion::before(1.87)]
const _: () = {
    impl<'a, const MAX: usize> ArbitraryAs<'a, String> for LimitLen<MAX> {
        #[inline]
        fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<String> {
            let mut s = String::arbitrary(u)?;
            s.truncate(MAX);
            Ok(s)
        }

        #[inline]
        fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<String> {
            let mut s = String::arbitrary_take_rest(u)?;
            s.truncate(MAX);
            Ok(s)
        }

        #[inline]
        fn size_hint_as(_depth: usize) -> (usize, Option<usize>) {
            (0, None)
        }
    }
};

#[cfg(test)]
mod tests {
    use arbitrary::Unstructured;

    use crate::{ArbitraryAs, LimitLen};

    #[test]
    fn limit_str() {
        const MAX_LEN: usize = 3;
        let u = Unstructured::new(&[0xff; 100]);

        let s: String = LimitLen::<MAX_LEN>::arbitrary_take_rest_as(u).unwrap();
        assert!(s.len() < MAX_LEN);
    }
}
