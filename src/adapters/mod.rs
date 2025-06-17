mod display;
mod from_into;

pub use self::{display::*, from_into::*};

use core::{hash::Hash, marker::PhantomData};

use arbitrary::{Arbitrary, MaxRecursionReached, Result, Unstructured};
use impl_tools::autoimpl;

use crate::ArbitraryAs;

pub struct As<T: ?Sized>(PhantomData<T>);

impl<T: ?Sized> As<T> {
    #[inline]
    pub fn arbitrary<'a, U>(u: &mut Unstructured<'a>) -> Result<U>
    where
        T: ArbitraryAs<'a, U>,
    {
        T::arbitrary_as(u)
    }
}

/// Helper to implement [`Arbitrary`] trait for adapters
#[autoimpl(Clone where T: Clone)]
#[autoimpl(Copy where T: Copy)]
#[autoimpl(PartialOrd where T: PartialOrd)]
#[autoimpl(Ord where T: Ord)]
#[autoimpl(PartialEq where T: PartialEq)]
#[autoimpl(Eq where T: Eq)]
#[autoimpl(Hash where T: Hash)]
#[repr(transparent)]
pub struct AsWrap<T, As>
where
    As: ?Sized,
{
    value: T,
    _phantom: PhantomData<As>,
}

impl<T, As> AsWrap<T, As>
where
    As: ?Sized,
{
    // Wrap given value
    #[inline]
    pub const fn new(value: T) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }

    pub fn from_slice(slice: &[T]) -> &[Self] {
        unsafe { slice.as_ptr().cast::<&[Self]>().read() }
    }

    /// Unwrap inner value
    #[inline]
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<'a, T, As> Arbitrary<'a> for AsWrap<T, As>
where
    As: ArbitraryAs<'a, T> + ?Sized,
{
    #[inline]
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        As::arbitrary_as(u).map(Self::new)
    }

    #[inline]
    fn arbitrary_take_rest(u: Unstructured<'a>) -> Result<Self> {
        As::arbitrary_take_rest_as(u).map(Self::new)
    }

    #[inline]
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        As::size_hint_as(depth)
    }

    #[inline]
    fn try_size_hint(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        As::try_size_hint_as(depth)
    }
}

pub struct Same;

impl<'a, T> ArbitraryAs<'a, T> for Same
where
    T: Arbitrary<'a>,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<T> {
        T::arbitrary(u)
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<T> {
        T::arbitrary_take_rest(u)
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        T::size_hint(depth)
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        T::try_size_hint(depth)
    }
}
