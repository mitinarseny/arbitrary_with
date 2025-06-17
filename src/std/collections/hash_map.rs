use core::hash::Hash;
use std::collections::HashMap;

use arbitrary::{Arbitrary, MaxRecursionReached, Result, Unstructured};

use crate::{ArbitraryAs, AsWrap};

impl<'a, K, KAs, V, VAs> ArbitraryAs<'a, HashMap<K, V>> for HashMap<KAs, VAs>
where
    KAs: ArbitraryAs<'a, K>,
    K: Eq + Hash,
    VAs: ArbitraryAs<'a, V>,
{
    #[inline]
    fn arbitrary_as(u: &mut Unstructured<'a>) -> Result<HashMap<K, V>> {
        u.arbitrary_iter::<(AsWrap<K, KAs>, AsWrap<V, VAs>)>()?
            .map(|r| r.map(|(k, v)| (k.into_inner(), v.into_inner())))
            .collect()
    }

    #[inline]
    fn arbitrary_take_rest_as(u: Unstructured<'a>) -> Result<HashMap<K, V>> {
        u.arbitrary_take_rest_iter::<(AsWrap<K, KAs>, AsWrap<V, VAs>)>()?
            .map(|r| r.map(|(k, v)| (k.into_inner(), v.into_inner())))
            .collect()
    }

    #[inline]
    fn size_hint_as(depth: usize) -> (usize, Option<usize>) {
        Self::try_size_hint_as(depth).unwrap_or_default()
    }

    #[inline]
    fn try_size_hint_as(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        HashMap::<AsWrap<K, KAs>, AsWrap<V, VAs>>::try_size_hint(depth)
    }
}
