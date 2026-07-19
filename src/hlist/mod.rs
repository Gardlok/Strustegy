//! A small heterogeneous-list substrate for static strategy pipelines.

use core::marker::PhantomData;

/// The empty heterogeneous list.
#[must_use = "an HList value has no effect unless it is used"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HNil;

/// A heterogeneous list node containing a head and another HList as its tail.
#[must_use = "an HList value has no effect unless it is used"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HCons<H, T: HList> {
    pub head: H,
    pub tail: T,
}

/// The structural contract shared by all heterogeneous lists.
pub trait HList: Sized {
    /// Number of elements encoded by this concrete list type.
    const LEN: usize;

    /// The same list shape containing shared references.
    type Refs<'a>: HList
    where
        Self: 'a;

    /// The same list shape containing mutable references.
    type Muts<'a>: HList
    where
        Self: 'a;

    /// Lend every element through a shared reference.
    #[must_use]
    fn refs(&self) -> Self::Refs<'_>;

    /// Lend every element through a mutable reference.
    #[must_use]
    fn muts(&mut self) -> Self::Muts<'_>;

    /// Prepend a value to this list.
    fn cons<H>(self, head: H) -> HCons<H, Self> {
        HCons { head, tail: self }
    }

    /// Return the statically known list length.
    fn len(&self) -> usize {
        Self::LEN
    }

    /// Return whether this list is empty.
    fn is_empty(&self) -> bool {
        Self::LEN == 0
    }
}

impl HList for HNil {
    const LEN: usize = 0;

    type Refs<'a> = HNil;
    type Muts<'a> = HNil;

    fn refs(&self) -> Self::Refs<'_> {
        HNil
    }

    fn muts(&mut self) -> Self::Muts<'_> {
        HNil
    }
}

impl<H, T> HList for HCons<H, T>
where
    T: HList,
{
    const LEN: usize = 1 + T::LEN;

    type Refs<'a>
        = HCons<&'a H, T::Refs<'a>>
    where
        Self: 'a;

    type Muts<'a>
        = HCons<&'a mut H, T::Muts<'a>>
    where
        Self: 'a;

    fn refs(&self) -> Self::Refs<'_> {
        HCons {
            head: &self.head,
            tail: self.tail.refs(),
        }
    }

    fn muts(&mut self) -> Self::Muts<'_> {
        HCons {
            head: &mut self.head,
            tail: self.tail.muts(),
        }
    }
}

/// Operations available only for non-empty HLists.
pub trait NonEmptyHList: HList {
    type Head;
    type Tail: HList;

    /// Consume the list and separate its head from its tail.
    fn into_parts(self) -> (Self::Head, Self::Tail);

    /// Borrow the head and tail independently.
    fn parts(&self) -> (&Self::Head, &Self::Tail);

    /// Mutably borrow the head and tail independently.
    fn parts_mut(&mut self) -> (&mut Self::Head, &mut Self::Tail);
}

impl<H, T> NonEmptyHList for HCons<H, T>
where
    T: HList,
{
    type Head = H;
    type Tail = T;

    fn into_parts(self) -> (Self::Head, Self::Tail) {
        (self.head, self.tail)
    }

    fn parts(&self) -> (&Self::Head, &Self::Tail) {
        (&self.head, &self.tail)
    }

    fn parts_mut(&mut self) -> (&mut Self::Head, &mut Self::Tail) {
        (&mut self.head, &mut self.tail)
    }
}

/// Type-level index selecting the current HList head.
pub enum Here {}

/// Type-level index selecting an element within the current HList tail.
pub struct There<I>(PhantomData<fn() -> I>);

/// Compile-time indexed HList access.
///
/// An out-of-bounds position is a type error rather than a runtime panic.
///
/// ```compile_fail
/// use strustegy::{Get, Here, There, hlist};
///
/// type Fourth = There<There<There<Here>>>;
/// let values = hlist![1_u8, 2_u16, 3_u32];
/// let _ = Get::<Fourth>::get(&values);
/// ```
pub trait Get<I>: HList {
    type Output;

    fn get(&self) -> &Self::Output;

    fn get_mut(&mut self) -> &mut Self::Output;
}

impl<H, T> Get<Here> for HCons<H, T>
where
    T: HList,
{
    type Output = H;

    fn get(&self) -> &Self::Output {
        &self.head
    }

    fn get_mut(&mut self) -> &mut Self::Output {
        &mut self.head
    }
}

impl<H, T, I> Get<There<I>> for HCons<H, T>
where
    T: Get<I>,
{
    type Output = <T as Get<I>>::Output;

    fn get(&self) -> &Self::Output {
        self.tail.get()
    }

    fn get_mut(&mut self) -> &mut Self::Output {
        self.tail.get_mut()
    }
}

/// Ergonomic method syntax for compile-time indexed access.
pub trait GetExt: HList {
    fn get_at<I>(&self) -> &<Self as Get<I>>::Output
    where
        Self: Get<I>,
    {
        <Self as Get<I>>::get(self)
    }

    fn get_at_mut<I>(&mut self) -> &mut <Self as Get<I>>::Output
    where
        Self: Get<I>,
    {
        <Self as Get<I>>::get_mut(self)
    }
}

impl<L: HList> GetExt for L {}

/// Construct an HList value.
#[macro_export]
macro_rules! hlist {
    () => {
        $crate::hlist::HNil
    };

    ($head:expr $(, $tail:expr)* $(,)?) => {
        $crate::hlist::HCons {
            head: $head,
            tail: $crate::hlist![$($tail),*],
        }
    };
}

/// Destructure an HList using ordinary Rust patterns.
///
/// ```
/// use strustegy::{hlist, hlist_pat};
///
/// let hlist_pat![number, mut name, _] =
///     hlist![7_u8, String::from("rose"), true];
/// name.push('!');
///
/// assert_eq!(number, 7);
/// assert_eq!(name, "rose!");
/// ```
#[macro_export]
macro_rules! hlist_pat {
    () => {
        $crate::hlist::HNil
    };

    ($head:pat $(, $tail:pat)* $(,)?) => {
        $crate::hlist::HCons {
            head: $head,
            tail: $crate::hlist_pat![$($tail),*],
        }
    };
}

/// Construct an HList type.
#[macro_export]
macro_rules! hlist_ty {
    () => {
        $crate::hlist::HNil
    };

    ($head:ty $(, $tail:ty)* $(,)?) => {
        $crate::hlist::HCons<
            $head,
            $crate::hlist_ty![$($tail),*]
        >
    };
}
