
use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div, Neg};

/////////////////////////////////////////////////////////////////////////////////////////////////////
// A linked list of type constructors with a Head end followed by a trailing Tail.
// HList, similar to linked lists, but instead of having a single element at each node, they have a 
// type constructor that can be used to construct a list of any length.  The type constructor HCons
// takes a head and a tail and returns a new list. The tail is itself an HList, and the head can be
// any type. Recursively, the tail can be an HCons or an HNil. 
// 
// This allows us to construct lists of arbitrary length and type.
//
// The trait HListOps provides functions for accessing and manipulating HLists. The head and tail 
// functions allow us to access the head and tail of an HList, respectively. The uncons and cons 
// functions allows to deconstruct and construct HLists, respectively.
// pub trait HList { type Item<'a>; }
pub trait HList { type Item<'a>: HList; }

// HCons is a type constructor that takes a head and a tail and returns a new list.
// The tail is itself an HList, and the head can be any type. Recursively, the tail
// can be an HCons or an HNil. This allows us to construct lists of arbitrary length.
pub struct HCons<H, T: HList> { pub head: H, pub tail: T }
impl<H, T: HList> HList
for HCons<H, T>
where for<'a> <T as HList>::Item<'a>: HList {
    type Item<'a> = HCons<H, <T as HList>::Item<'a>>;
}

// cons
pub trait HListCons<'a, H> { type TypeCtor: HList;
    fn cons(self, head: H) -> <Self::TypeCtor as HListable<'a>>::Output where <Self as HListCons<'a, H>>::TypeCtor: for<'b> HListable<'a>;
}

pub struct HNil;
impl HList 
for HNil { type Item<'a> = Self; }

// Provides a type constructor for HLists.
pub trait HListable<'a> { type Output; }
impl<'a, T> HListable<'a> for &'a T { type Output = T; }
impl<'a, T> HListable<'a> for &'a mut T { type Output = T; }


pub trait HListOps {
    type Head;
    type Tail: HList;

    fn head(&self) -> &Self::Head;
    fn tail(&self) -> &Self::Tail;
}
impl<H, T: HList> HListOps for HCons<H, T> {
    type Head = H;
    type Tail = T;
    fn head(&self) -> &Self::Head { &self.head }
    fn tail(&self) -> &Self::Tail { &self.tail }
}
impl<H, T: HList> HListOps for HListEnum<H, T> {
    type Head = H;
    type Tail = T;
    fn head(&self) -> &Self::Head {
        match self {
            HListEnum::HCons(h, _) => h,
            HListEnum::HNil => panic!(),  // This should never happen
        }
    }
    fn tail(&self) -> &Self::Tail {
        match self {
            HListEnum::HCons(_, t) => t,
            HListEnum::HNil => panic!(),  // This should never happen. TODO: Is there a better way to handle this?
        }
    }
}

pub trait HListOpsMut {
    type Head;
    type Tail: HList;

    fn head_mut(&mut self) -> &mut Self::Head;
    fn tail_mut(&mut self) -> &mut Self::Tail;
}
impl<H, T: HList> HListOpsMut for HCons<H, T> {
    type Head = H;
    type Tail = T;
    fn head_mut(&mut self) -> &mut Self::Head { &mut self.head }
    fn tail_mut(&mut self) -> &mut Self::Tail { &mut self.tail }
}

// uncons
pub trait HListUncons {
    type Head;
    type Tail: HList;

    fn uncons(self) -> (Self::Head, Self::Tail);
}
impl<H, T: HList> HListUncons for HCons<H, T> {
    type Head = H;
    type Tail = T;
    fn uncons(self) -> (Self::Head, Self::Tail) { (self.head, self.tail) }
}

// HConsTuple is a type constructor that takes a head and a tail and returns a new list.
pub struct HConsTuple<H, T: HList>(pub H, pub T);
impl<H, T: HList> HList for HConsTuple<H, T> where for<'a> <T as HList>::Item<'a>: HList {
    type Item<'a> = HConsTuple<H, <T as HList>::Item<'a>>;
}

// Enumerated HList
pub enum HListEnum<H, T: HList> { HCons(H, T), HNil }  
struct HListEnum_<H, T>(PhantomData<(H, T)>);
impl<'a, H, T> HListable<'a> for HListEnum_<H, T> where for<'b> <T as HList>::Item<'b>: HList, T: HList { 
    type Output = HListEnum<H, <T as HList>::Item<'a>>; 
}
impl<'a> HListable<'a> for HNil {
    type Output = HListEnum<HNil, HNil>;
}

// Allow us to use the iterator functions such as map, fold, filter, etc. on HLists.
pub trait HListIter<'a> { type TypeCtor: HList;
    fn next(&mut self) -> Option<<Self::TypeCtor as HListable<'a>>::Output> where <Self as HListIter<'a>>::TypeCtor: HListable<'a>; 
}
pub trait HListIntoIter<'a> { type TypeCtor: HList;
    fn into_iter(self) -> <Self::TypeCtor as HListable<'a>>::Output where <Self as HListIntoIter<'a>>::TypeCtor: HListable<'a>;
}

// HListMapper: Apply a function to each element of an HList.
// Example:
// impl Mapper for i32 {
//     type Output = i32;
//     fn map(self) -> Self::Output {
//         self + 1
//     }
// }
// And then we can use it like this:
// let hlist = HCons { head: 1, tail: HCons { head: 2, tail: HCons { head: 3, tail: HNil } } };
// let hlist = hlist.map();
// assert_eq!(hlist.head, 2);
// assert_eq!(hlist.tail.head, 3);
// assert_eq!(hlist.tail.tail.head, 4);
// assert_eq!(hlist.tail.tail.tail, HNil);
//
// Note that the Mapper trait is not limited to functions that take a single argument. It can be used with any function that implements
// the Mapper trait. For example, we can use it with a function that takes two arguments:
// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }
// impl Mapper for i32 {
//     type Output = i32;
//     fn map(self) -> Self::Output {
//         add(self, 1)
//     }
// }
// And then we can use it like this:
// let hlist = HCons { head: 1, tail: HCons { head: 2, tail: HCons { head: 3, tail: HNil } } };
// let hlist = hlist.map(); 
// assert_eq!(hlist.head, 2);
// assert_eq!(hlist.tail.head, 3);
// assert_eq!(hlist.tail.tail.head, 4);
// assert_eq!(hlist.tail.tail.tail, HNil);
//
pub trait Mapper {
    type Output;
    fn map(self) -> Self::Output;
}
pub trait HListMapper {
    type Output: HList;
    fn map(self) -> Self::Output;
}
impl HListMapper for HNil {
    type Output = HNil;
    fn map(self) -> Self::Output {
        HNil
    }
}
impl<H, T> HListMapper for HCons<H, T>
where
    H: Mapper,
    T: HList + HListMapper,
{
    type Output = HCons<H::Output, T::Output>;
    fn map(self) -> Self::Output {
        HCons {
            head: self.head.map(),
            tail: self.tail.map(),
        }
    }
}

// HlistFoler: Apply a function to each element of an HList.
// Example:
// impl Folder for i32 {
//     type Output = i32;
//     fn fold(self, acc: i32) -> Self::Output {
//         self + acc
//     }
// }
// And then we can use it like this:
// let hlist = HCons { head: 1, tail: HCons { head: 2, tail: HCons { head: 3, tail: HNil } } };
// let hlist = hlist.fold(0);
// assert_eq!(hlist, 6);
//
// Note that the Folder trait is not limited to functions that take a single argument. It can be used with any function that implements
// the Folder trait. For example, we can use it with a function that takes two arguments:
// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }
// impl Folder for i32 {
//     type Output = i32;
//     fn fold(self, acc: i32) -> Self::Output {
//         add(self, acc)
//     }
// }
// And then we can use it like this:
// let hlist = HCons { head: 1, tail: HCons { head: 2, tail: HCons { head: 3, tail: HNil } } };
// let hlist = hlist.fold(0);
// assert_eq!(hlist, 6);
//
pub trait Folder {
    type Output;
    fn fold(self, acc: Self::Output) -> Self::Output;
}
pub trait HListFolder {
    type Output;
    fn fold(self, acc: Self::Output) -> Self::Output;
}
impl HListFolder for HNil {
    type Output = HNil;
    fn fold(self, acc: Self::Output) -> Self::Output {
        HNil
    }
}
impl<H, T> HListFolder for HCons<H, T>
where
    H: Folder,
    T: HList + HListFolder, <T as HListFolder>::Output: HList
{
    type Output = HCons<H::Output, T::Output>;
    fn fold(self, acc: Self::Output) -> Self::Output {
        HCons {
            head: self.head.fold(acc.head),
            tail: self.tail.fold(acc.tail),
        }
    }
}

// hlist macro // 
#[macro_export]
macro_rules! hlist {
    () => { HNil };
    ($head:expr) => { HCons { head: $head, tail: HNil } };
    ($head:expr, $($tail:expr),+) => { HCons { head: $head, tail: hlist!($($tail),+) } };
}

// Integration tests
#[cfg(test)]
mod test_integration {
    use super::*;
    use std::any::TypeId;

    #[test]
    fn test_basics() {
        let mut hlist = hlist!(0, 1, 2);
        assert_eq!(hlist.head, 0);
        assert_eq!(hlist.tail.head, 1);
        assert_eq!(hlist.tail.tail.head, 2);

        hlist.head = 42;
        assert_eq!(hlist.head, 42);

        hlist.head = 0;
        hlist.tail.head = 42;
        assert_eq!(hlist.head, 0);
        assert_eq!(hlist.tail.head, 42);

        let mut hlist2 = hlist!(0, "Hello".to_string(), [2]);
        assert_eq!(hlist2.head, 0);
        assert_eq!(hlist2.tail.head, "Hello".to_string());
        assert_eq!(hlist2.tail.tail.head, [2]);

        // anonymous types
        let mut f = |x: &mut HCons<i32, HCons<String, HCons<[i32; 1], HNil>>>| {
            x.head = 42;
            x.tail.head = "Hello".to_string();
            x.tail.tail.head = [2];
        };

        f(&mut hlist2);
        assert!(hlist2.head == 42);
        assert!(f(&mut hlist2) == ());
        assert_eq!(hlist2.head + 1, 43);
        assert_eq!(hlist2.head - 5, 37);
        assert_eq!(hlist2.head * 2, 84);
        assert_eq!(hlist2.head / 2, 21);
        assert_eq!(hlist2.head % 2, 0);
        assert_eq!(hlist2.head << 1, 84);
        assert_eq!(hlist2.head >> 1, 21);

    }


    #[test]
    fn dozen_deep() {
        let hlist = hlist!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9); 
        assert_eq!(hlist.tail.head, 1);
        assert_eq!(hlist.tail.tail.head, 2);
        assert_eq!(hlist.tail.tail.tail.head, 3);
        assert_eq!(hlist.tail.tail.tail.tail.head, 4);
        assert_eq!(hlist.tail.tail.tail.tail.tail.head, 5);
        assert_eq!(hlist.tail.tail.tail.tail.tail.tail.head, 6);
        assert_eq!(hlist.tail.tail.tail.tail.tail.tail.tail.head, 7);
        assert_eq!(hlist.tail.tail.tail.tail.tail.tail.tail.tail.head, 8);
        assert_eq!(hlist.tail.tail.tail.tail.tail.tail.tail.tail.tail.head, 9);
    }
    
    #[test]
    fn test_nestive() {
        let hlist: HCons<u32, HCons<u32, HNil>> = HCons { head: 0, tail: HCons { head: 1, tail: HNil } };
        let hlist: HCons<u32, HCons<u32, HCons<u32, HNil>>> = HCons { head: 2, tail: hlist };
        let hlist: HCons<u32, HCons<u32, HCons<u32, HCons<u32, HNil>>>> = HCons { head: 3, tail: hlist };
        assert_eq!(hlist.head, 3);
        assert_eq!(hlist.tail.head, 2);
        assert_eq!(hlist.tail.tail.head, 0);
        assert_eq!(hlist.tail.tail.tail.head, 1);
    }

    // Uncons and cons additional testing
    #[test]
    fn test_uncons() {
        let hlist = hlist!(0, 1, 2);
        let (head, tail) = hlist.uncons();
        assert_eq!(head, 0);
        assert_eq!(tail.head, 1);
        assert_eq!(tail.tail.head, 2);
    }
    


    #[test]
    fn test_map() {

        impl Mapper for i32 {
            type Output = i32;
            fn map(self) -> Self::Output {
                self + 1
            }
        }
        
        impl Mapper for f64 {
            type Output = f64;
            fn map(self) -> Self::Output {
                self * 2.0
            }
        }

        let hlist: HCons<i32, HCons<f64, HNil>> = hlist!(0, 1.0);
        let hlist_mapped = hlist.map();
        assert_eq!(hlist_mapped.head, 1);
        assert_eq!(hlist_mapped.tail.head, 2.0);
    }



#[test]
fn test_fold() {
    
    // Folder implementation for i32
    impl Folder for i32 {
        type Output = i32;
        fn fold(self, acc: Self::Output) -> Self::Output {
            self + acc
        }
    }

    let hlist = hlist!(1, 2, 3);
    let sum = hlist.fold(0);  // Initial value is 0
    assert_eq!(sum, 6);
}





}