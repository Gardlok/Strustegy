
use std::marker::PhantomData;

// HListable is a trait that allows us to convert a tuple into an HList.  
pub trait HListable<'a> { type Output; }
impl<'a, T> HListable<'a> for &'a T { type Output = T; }
impl<'a, T> HListable<'a> for &'a mut T { type Output = T; }

// Heterogeneous list of type constructors. 
pub trait HList { type Item<'a>; }

// HNil is a terminator for HLists. It has no fields. 
pub struct HNil;
impl HList 
for HNil { type Item<'a> = Self; }

// HCons is a type constructor that takes a head and a tail and returns a new list.
// The tail is itself an HList, and the head can be any type. Recursively, the tail
// can be an HCons or an HNil. This allows us to construct lists of arbitrary length.
pub struct HCons<H, T: HList> { head: H, tail: T }
impl<H, T: HList> HList
for HCons<H, T>
where for<'a> <T as HList>::Item<'a>: HList {
    // The Item type is an HCons of the head and the tail's Item type.
    // This is the recursive definition of an HList. 
    type Item<'a> = HCons<H, <T as HList>::Item<'a>>;
}

// HListOps is a trait that provides common operations on HLists. 
pub trait HListOps {
    type Head;
    type Tail: HList;

    fn head(&self) -> &Self::Head;
    fn tail(&self) -> &Self::Tail;
}

// For HCons, the head is the first element of the list and the tail is the rest of the list.
impl<H, T: HList> HListOps for HCons<H, T> {
    type Head = H;
    type Tail = T;
    fn head(&self) -> &Self::Head { &self.head }
    fn tail(&self) -> &Self::Tail { &self.tail }
}

// For HNil, the head and tail are both HNil.
impl<H, T: HList> HListOps for HListEnum<H, T> {
    type Head = H;
    type Tail = T;
    fn head(&self) -> &Self::Head {
        match self {
            HListEnum::HCons(h, _) => h,
            HListEnum::HNil => panic!(),
        }
    }
    fn tail(&self) -> &Self::Tail {
        match self {
            HListEnum::HCons(_, t) => t,
            HListEnum::HNil => panic!(),
        }
    }
}

// A function that takes an HListable and returns an HList, which is the HListable's Output type.
fn _hlist<'a, T: HList + HListable<'a, Output = T>>(v: T) -> <T as HListable<'a>>::Output { v }  

pub struct HCons_<H, T: HList> { head: H, tail: T }
impl<H, T: HList> HList for HCons_<H, T> where for<'a> <T as HList>::Item<'a>: HList {
    type Item<'a> = HCons_<H, <T as HList>::Item<'a>>;
}

// HNil (HList terminator)
pub struct HNil_;
impl HList for HNil_ { type Item<'a> = Self; }

// Identifiers for HList operations
struct Head_<H, T>(PhantomData<(H, T)>);  
impl<'a, H, T> HListable<'a> for Head_<H, T> { type Output = H; }

struct Tail_<H, T>(PhantomData<(H, T)>);
impl<'a, H, T> HListable<'a> for Tail_<H, T> { type Output = T; }



// HListable trait for HCons_ and HNil_. 
impl<'a, H, T: HList + HListable<'a, Output = T>> HListable<'a> for HCons_<H, T> {
    type Output = HListEnum<H, T>;
}

// HListable trait for HNil. 
impl<'a> HListable<'a> for HNil_ {
    type Output = HListEnum<HNil, HNil>;
}

// HListable trait for HCons.
impl<'a, H, T: HList + HListable<'a, Output = T>> HListable<'a> for HCons<H, T> {
    type Output = HListEnum<H, T>;
}


// HListEnum is a type constructor that returns a new list. 
pub enum HListEnum<H, T: HList> { HCons(H, T), HNil }  
struct HListEnum_<H, T>(PhantomData<(H, T)>);
impl<'a, H, T> HListable<'a> for HListEnum_<H, T> where for<'b> <T as HList>::Item<'b>: HList, T: HList {
     
    type Output = HListEnum<H, <T as HList>::Item<'a>>;
}

// HListable trait for HNil. This allows us to use HNil in functions that take a type that implements HListable.
impl<'a> HListable<'a> for HNil {
    type Output = HListEnum<HNil, HNil>;
}
// To implement HList for HListEnum, we need to implement the Item type. 
impl<H, T: HList> HList for HListEnum<H, T> where for<'a> <T as HList>::Item<'a>: HList {
    type Item<'a> = HListEnum<H, <T as HList>::Item<'a>>;
}   

// HListable trait for HCons. This allows us to use HCons in functions that take a type that implements HListable.
impl<'a, H, T> HListable<'a> for HCons<H, T> where for<'b> <T as HList>::Item<'b>: HList, T: HList {
    type Output = HListEnum<H, <T as HList>::Item<'a>>;
}

// HLEval is a trait that provides evaluation of HLists. 
pub trait HLEval<'a> {
    type TypeCtor: HList;
    fn eval(&self) -> <Self::TypeCtor as HListable<'a>>::Output where <Self as HLEval<'a>>::TypeCtor: HListable<'a>;
}

// HListIter is a trait that provides iteration over HLists. Providing an iterator for HLists allows us to use them in for loops.
// The iterator is generic over the lifetime of the HList. This allows us to iterate over HLists that contain references.
// The iterator is also generic over the type of the HList. This allows us to iterate over HLists that contain different types.
// Finally, the iterator is generic over the type of the output. This allows us fiddle with the output type of the iterator and
// return a different type than the HList itself. 
pub trait HListIter<'a> {
    type TypeCtor: HList;
    fn next(&mut self) -> Option<<Self::TypeCtor as HListable<'a>>::Output> where <Self as HListIter<'a>>::TypeCtor: HListable<'a>; 
}

// to be iterable, an HList must be able to return an iterator.
pub trait HListIntoIter<'a> {
    type TypeCtor: HList;
    fn into_iter(self) -> <Self::TypeCtor as HListable<'a>>::Output where <Self as HListIntoIter<'a>>::TypeCtor: HListable<'a>;
}

// HListMap is a trait that provides mapping over HLists. Providing a map function for HLists allows us to apply functions to 
// each element of the list. The map function is generic as similar to the iterator, difference being that the map function
// takes a function as an argument.
pub trait HListMap<'a, A> {
    type TypeCtor: HList;
    fn map(&self, arg: &'a A) -> <Self::TypeCtor as HListable<'a>>::Output where <Self as HListMap<'a, A>>::TypeCtor: HListable<'a>;
}

// HListFold is a trait that provides folding over HLists. Providing a fold function for HLists allows us to apply functions to
// each element of the list and accumulate the result. The fold function is generic as similar to the iterator, difference being
// that the fold function takes a function as an argument and an initial value, which is used as the accumulator.
pub trait HListFold<'a, A, B> {
    type TypeCtor: HList;
    fn fold(&self, arg: &'a A, init: B) -> B where <Self as HListFold<'a, A, B>>::TypeCtor: HListable<'a>;
}

// HListFilter is a trait that provides filtering over HLists. Providing a filter function for HLists allows us to apply functions to
// each element of the list and accumulate the result. The filter function is generic as similar to the iterator, difference being
// that the filter function takes a function as an argument and an initial value, which is used as the accumulator.
pub trait HListFilter<'a, A> {
    type TypeCtor: HList;
    fn filter(&self, arg: &'a A) -> <Self::TypeCtor as HListable<'a>>::Output where <Self as HListFilter<'a, A>>::TypeCtor: HListable<'a>;
}

// Macros //


// hlist macro
#[macro_export]
macro_rules! hlist {
    () => { HNil };
    ($head:expr) => { HCons { head: $head, tail: HNil } };
    ($head:expr, $($tail:expr),+) => { HCons { head: $head, tail: hlist!($($tail),+) } };
}

// Macro HListGate is a macro that allows us to use HLists in functions that take a type that implements HListable.
// If the expression is an HList, return it. If the expression is not an HList, wrap it in an HList.
#[macro_export]
macro_rules! HListGate {
    ($e:expr) => { $e };  
    ($e:expr) => { hlist!($e) };  
}

/////////////////////////////////////////////////////////////////////////////////////////////////////









// Integration tests
#[cfg(test)]
mod test_integration {
    use super::*;
    use std::any::TypeId;

    #[test]
    fn test_hlist() {
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

}




// Let's test it out
#[cfg(test)]
mod test_macro {
    use super::*;

#[test]
    fn test_hlist() {
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
    fn test_hlist_nested() {
        let hlist: HCons<u32, HCons<u32, HNil>> = HCons { head: 0, tail: HCons { head: 1, tail: HNil } };
        let hlist: HCons<u32, HCons<u32, HCons<u32, HNil>>> = HCons { head: 2, tail: hlist };
        let hlist: HCons<u32, HCons<u32, HCons<u32, HCons<u32, HNil>>>> = HCons { head: 3, tail: hlist };
        assert_eq!(hlist.head, 3);
        assert_eq!(hlist.tail.head, 2);
        assert_eq!(hlist.tail.tail.head, 0);
        assert_eq!(hlist.tail.tail.tail.head, 1);
    }
}