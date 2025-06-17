use core::cell::Cell;
use std::cell::{RefCell, UnsafeCell};

use arbitrary::{MaxRecursionReached, Result, Unstructured};

use crate::ArbitraryAs;

impl<'a, T, As> ArbitraryAs<'a, Cell<T>> for Cell<As>
where
    As: ArbitraryAs<'a, T> + ?Sized,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<Cell<T>> {
        As::arbitrary_as(u).map(Cell::new)
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<Cell<T>> {
        As::arbitrary_take_rest_as(u).map(Cell::new)
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        As::try_size_hint_as(depth).unwrap_or_default()
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        Ok(As::size_hint_as(depth))
    }
}

impl<'a, T, As> ArbitraryAs<'a, RefCell<T>> for RefCell<As>
where
    As: ArbitraryAs<'a, T> + ?Sized,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<RefCell<T>> {
        As::arbitrary_as(u).map(RefCell::new)
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<RefCell<T>> {
        As::arbitrary_take_rest_as(u).map(RefCell::new)
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        As::try_size_hint_as(depth).unwrap_or_default()
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        Ok(As::size_hint_as(depth))
    }
}

impl<'a, T, As> ArbitraryAs<'a, UnsafeCell<T>> for UnsafeCell<As>
where
    As: ArbitraryAs<'a, T> + ?Sized,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<UnsafeCell<T>> {
        As::arbitrary_as(u).map(UnsafeCell::new)
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<UnsafeCell<T>> {
        As::arbitrary_take_rest_as(u).map(UnsafeCell::new)
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        As::try_size_hint_as(depth).unwrap_or_default()
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        Ok(As::size_hint_as(depth))
    }
}
