//! Applying type-directed strategies across heterogeneous lists.

use crate::hlist::{HCons, HList, HNil};
use crate::strategy::Strategy;

/// Map one heterogeneous strategy across an owned HList.
pub trait HMap<S>: HList {
    type Output: HList;

    #[must_use]
    fn hmap(self, strategy: &S) -> Self::Output;
}

impl<S> HMap<S> for HNil {
    type Output = HNil;

    fn hmap(self, _strategy: &S) -> Self::Output {
        HNil
    }
}

impl<H, T, S> HMap<S> for HCons<H, T>
where
    T: HList + HMap<S>,
    S: Strategy<H>,
{
    type Output = HCons<<S as Strategy<H>>::Output, <T as HMap<S>>::Output>;

    fn hmap(self, strategy: &S) -> Self::Output {
        HCons {
            head: strategy.apply(self.head),
            tail: self.tail.hmap(strategy),
        }
    }
}

/// Borrowed and mutable HList mapping derived from the HList GAT views.
pub trait HMapRefExt: HList {
    #[must_use]
    fn hmap_ref<'a, S>(&'a self, strategy: &S) -> <Self::Refs<'a> as HMap<S>>::Output
    where
        Self::Refs<'a>: HMap<S>,
    {
        self.refs().hmap(strategy)
    }

    #[must_use]
    fn hmap_mut<'a, S>(&'a mut self, strategy: &S) -> <Self::Muts<'a> as HMap<S>>::Output
    where
        Self::Muts<'a>: HMap<S>,
    {
        self.muts().hmap(strategy)
    }
}

impl<L: HList> HMapRefExt for L {}
