
use std::{mem, cmp};
use std::any::{Any, TypeId};
use std::marker::PhantomData;

use std::ops::Deref;
use std::ops::DerefMut;
use std::fmt::Debug;
use std::sync::Arc;
use std::rc::Rc;


use syn::token::Ref;
use std::iter::DoubleEndedIterator;

// ---------------------------------------------------------------------------------------- //

/// Sealing trait
pub trait Sealed {}
impl<T> Sealed for T {} 

/// Represents types that have a phantom lifetime.
pub trait PhantomLifetime<'a, T: 'a + Clone> {
    type Phantom<'p>: PhantomLifetime<'p, T> where T: 'p;
}
impl<'a, T: 'a + Clone> PhantomLifetime<'a, T> for &'a mut dyn Sealed {
    type Phantom<'p> = &'p mut dyn Sealed where T: 'p;
}
impl<'a, T: 'a + Clone> PhantomLifetime<'a, T> for Box<dyn Sealed> {
    type Phantom<'p> = Box<dyn Sealed> where T: 'p;
}
/// Represents types that have a specific lifetime.
pub trait Lifetime<'a, T: 'a + Clone> {}
impl<'a, T: 'a + Clone> Lifetime<'a, T> for &'a dyn Sealed {}
impl<'a, T: 'a + Clone> Lifetime<'a, T> for &'a mut dyn Sealed {}
impl<'a, T: 'a + Clone> Lifetime<'a, T> for Box<dyn Sealed> {}

/// Represents types that have certain bounds.
pub trait Bounds<'a, T: 'a + Clone> {}
impl<'a, T: 'a + Clone> Bounds<'a, T> for &'a T {}
impl<'a, T: 'a + Clone> Bounds<'a, T> for &'a mut T {}
impl<'a, T: 'a + Clone> Bounds<'a, T> for Box<T> {}

/// Represents types that have both a specific lifetime and certain bounds.
pub trait LifetimeBounds<'a, T: 'a + Clone>: Lifetime<'a, T> + Bounds<'a, T> {}
impl<'a, T: 'a + Clone> LifetimeBounds<'a, T> for &'a mut T where &'a mut T: Lifetime<'a, T> {}
impl<'a, T: 'a + Clone> LifetimeBounds<'a, T> for Box<T> where Box<T>: Lifetime<'a, T> {}

/// Represents types that have a specific lifetime, certain bounds, and a phantom lifetime.
pub trait LifetimeBoundsPhantom<'a, T: 'a + Clone>: LifetimeBounds<'a, T> + PhantomLifetime<'a, T> {} 
impl<'a, T: 'a + Clone> LifetimeBoundsPhantom<'a, T> for &'a mut T where &'a mut T: LifetimeBounds<'a, T> + PhantomLifetime<'a, T> {}
impl<'a, T: 'a + Clone> LifetimeBoundsPhantom<'a, T> for Box<T> where Box<T>: LifetimeBounds<'a, T> + PhantomLifetime<'a, T> {}

// ---------------------------------------------------------------------------------------- //

// Kind: Lifetime -> Type   
pub trait LifeType<'a> { type Output; }
impl<'a, T> LifeType<'a> for &'a T { type Output = T; }
impl<'a, T> LifeType<'a> for &'a mut T { type Output = T; }
impl<'a, T> LifeType<'a> for Box<T> { type Output = T; }

pub enum NodeEnum<Ref, T> where Ref: RcLike<Node<Ref, T>> {
    Node(Node<Ref, T>),
    Nil,
}

pub trait RcLike<T> { fn new(data: T) -> Self::Output;
    type Output;
}
struct Rc_; struct Arc_;
// For RcLike to Rc
impl<T> RcLike<T> for Rc_ { fn new(data: T) -> Self::Output { Rc::new(data) }
    type Output = Rc<T>;
}
// For RcLike to Arc
impl<T> RcLike<T> for Arc_ { fn new(data: T) -> Self::Output { Arc::new(data) }
    type Output = Arc<T>;
}

// A node is a reference counted list element with a next pointer
pub struct Node<Ref, T> where Ref: RcLike<Node<Ref, T>>,
{
    pub elem: T,
    pub next: Option<<Ref as RcLike<Node<Ref, T>>>::Output>,
    pub prev: Option<<Ref as RcLike<Node<Ref, T>>>::Output>,
}
// The list itself is a reference counted pointer to a node
pub struct List<Ref, T> where Ref: RcLike<Node<Ref, T>>,
{
    pub head: Option<<Ref as RcLike<Node<Ref, T>>>::Output>
}

// List implementation takes a reference counted pointer type and a type for the elements
impl<'a, Ref, T, RefNode> List<Ref, T> where Ref: RcLike<Node<Ref, T>, Output=RefNode>,

    // The reference counted pointer type must implement Deref to a Node
    RefNode: Deref<Target=Node<Ref, T>> + 'a,
    RefNode: Clone,
{
    fn new() -> Self { 
        List { head: None }
    }

    // Push a new element onto the list and return a new list, reusing the existing nodes
    fn push(&self, elem: T) -> Self {
        List {
            head: Some(Ref::new(Node {
                elem: elem,
                next: self.head.clone(),
                prev: None,
            }))
        }
    }

    // Return a new list with the first element removed
    fn tail(&self) -> Self {
        List { head: self.head.as_ref().and_then(|head| head.next.clone()) }
    }

    // Return a reference to the first element
    fn head(&'a self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.elem) 
    }

    // Return a reference to the last element
    fn last(&'a self) -> Option<&T> {
        self.head.as_ref().map(|head| {
            let mut node = head;
            while let Some(ref next) = node.next {
                node = next;
            }
            &node.elem
        })
    }

    // Return a reference to the nth element
    fn nth(&'a self, n: usize) -> Option<&T> {
        let mut node = self.head.as_ref();
        for _ in 0..n {
            node = node.and_then(|node| node.next.as_ref());
        }
        node.map(|node| &node.elem)
    }

    // Return a reference to the nth element from the end
    fn nth_from_end(&'a self, n: usize) -> Option<&T> {
        let mut node = self.head.as_ref();
        let mut nth_node = self.head.as_ref();
        for _ in 0..n {
            node = node.and_then(|node| node.next.as_ref());
        }
        while let Some(ref next) = node.and_then(|node| node.next.as_ref()) {
            node = Some(next);
            nth_node = nth_node.and_then(|node| node.next.as_ref());
        }
        nth_node.map(|node| &node.elem)
    }

}

struct Ref_<T>(PhantomData<T>);    
struct RefMut_<T>(PhantomData<T>); 
impl<'a, T: 'a> LifeType<'a> for Ref_<T> { type Output = &'a T; }
impl<'a, T: 'a> LifeType<'a> for RefMut_<T> { type Output = &'a mut T; }
fn _hack_ref<'a, T>(v: &'a T) -> <Ref_<T> as LifeType<'a>>::Output { v }
fn _hack_ref_mut<'a, T>(v: &'a mut T) -> <RefMut_<T> as LifeType<'a>>::Output { v }

// RefIter is a trait that yields references to a type.
trait RefIterator { 
    type TypeCtor;
    
    fn next<'a>(&'a mut self) -> Option<<Self::TypeCtor as LifeType<'a>>::Output>
        // Lifetime of the yielded type binds to the lifetime of the iterator
        where Self::TypeCtor: LifeType<'a>;
}

// Iter is an iterator that yields references to a slice of T (i.e. &[T])
struct Iter<'a, T: 'a> { slice: &'a [T] }
impl<'x, T> RefIterator for Iter<'x, T> {
    type TypeCtor = Ref_<T>;
    fn next<'a>(&'a mut self) -> Option<<Self::TypeCtor as LifeType<'a>>::Output>
        where Self::TypeCtor: LifeType<'a>
    {
        if self.slice.is_empty() {
            None
        } else {
            let (l, r) = mem::replace(&mut self.slice, &mut []).split_at(1);
            self.slice = r;
            Some(_hack_ref(&l[0]))
        }
    }
}

// IterMut is an iterator that yields mutable references to a slice of T (i.e. &mut [T])
struct IterMut<'a, T: 'a> { slice: &'a mut [T] }
impl<'x, T> RefIterator for IterMut<'x, T> {  
    type TypeCtor = RefMut_<T>;
    fn next<'a>(&'a mut self) -> Option<<Self::TypeCtor as LifeType<'a>>::Output>
        where Self::TypeCtor: LifeType<'a>
    {
        if self.slice.is_empty() { 
            None
        } else {
            let (l, r) = mem::replace(&mut self.slice, &mut []).split_at_mut(1);
            self.slice = r;
            Some(_hack_ref_mut(&mut l[0]))
        }
    }
}

pub struct Cursor<'a> { _marker: PhantomData<&'a ()> }
impl<'a> Cursor<'a> {
    pub fn new() -> Self { Cursor { _marker: PhantomData } }
}
struct Cur_<T>(PhantomData<T>);
impl<'a> LifeType<'a> for Cursor<'a> { type Output = Cursor<'a>; }
impl<'a, T> LifeType<'a> for Cur_<T> { type Output = Cursor<'a>; }
fn _cur<'a, T>(v: Cursor<'a>) -> <Cur_<T> as LifeType<'a>>::Output { v }


// // Apply a list of type constructors to an argument.
fn map1<'a, A, F: for<'b> ApplyTo<'b, A>>(funcs: Vec<F>, arg: &'a A) -> Vec<<F as ApplyTo<'_, A>>::Output> {  
    funcs.into_iter().map(|f| f.apply(arg)).collect()
}

// Heterogeneous list - a list that can contain elements of different types.
pub trait HList { type Item<'a>; }
// HNil is a type constructor that returns a new list.
pub struct HNil;
impl HList 
for HNil { type Item<'a> = Self; }
// HCons is a type constructor that takes a head and a tail and returns a new list.
pub struct HCons<H, T: HList> { head: H, tail: T }
impl<H, T: HList> HList
for HCons<H, T>
where for<'a> <T as HList>::Item<'a>: HList {
    // Item is a type constructor that returns the type of the list.
    type Item<'a> = HCons<H, <T as HList>::Item<'a>>;
}

// HListOps is a trait that provides operations on HLists.
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
fn _hack_hlist<'a, T: HList + LifeType<'a, Output = T>>(v: T) -> <T as LifeType<'a>>::Output { v }

pub struct HCons_<H, T: HList> { head: H, tail: T }
impl<H, T: HList> HList for HCons_<H, T> where for<'a> <T as HList>::Item<'a>: HList {
    type Item<'a> = HCons_<H, <T as HList>::Item<'a>>;
}

pub struct HNil_;
impl HList for HNil_ { type Item<'a> = Self; }

struct Head_<H, T>(PhantomData<(H, T)>);  
impl<'a, H, T> LifeType<'a> for Head_<H, T> { type Output = H; }

struct Tail_<H, T>(PhantomData<(H, T)>);
impl<'a, H, T> LifeType<'a> for Tail_<H, T> { type Output = T; }

struct HListEnum_<H, T>(PhantomData<(H, T)>);
impl<'a, H, T> LifeType<'a> for HListEnum_<H, T> where for<'b> <T as HList>::Item<'b>: HList, T: HList {
    type Output = HListEnum<H, <T as HList>::Item<'a>>;
}
pub enum HListEnum<H, T: HList> { HCons(H, T), HNil }
impl<H, T: HList> HList for HListEnum<H, T> where for<'a> <T as HList>::Item<'a>: HList {
    type Item<'a> = HListEnum<H, <T as HList>::Item<'a>>;
}   
impl<'a, H, T: HList + LifeType<'a, Output = T>> LifeType<'a> for HCons<H, T> {
    type Output = HListEnum<H, T>;
}
impl<'a> LifeType<'a> for HNil {
    type Output = HListEnum<HNil, HNil>;
}

pub trait HListIter<'a> {
    type TypeCtor: HList;
    fn next(&mut self) -> Option<<Self::TypeCtor as LifeType<'a>>::Output> where <Self as HListIter<'a>>::TypeCtor: LifeType<'a>; 
}

pub trait HListMap<'a, A> {
    type TypeCtor: HList;
    fn map(&self, arg: &'a A) -> <Self::TypeCtor as LifeType<'a>>::Output where <Self as HListMap<'a, A>>::TypeCtor: LifeType<'a>;
}

// Define a registry for generic types
#[derive(Clone, Copy)]  
struct Identity;
// We had to add the Copy trait to Identity because we want to be able to copy it into the registry.
impl<'a, A:Copy> ApplyTo<'a, A> for Identity {
    type Output = A;
    fn apply(&self, arg: &'a A) -> Self::Output { *arg }
}

// Define a registry for generic types
pub struct TypeRegistry<Ref, T> where Ref: RcLike<Node<Ref, T>> {
    list: List<Ref, T>,
}

impl<'a, Ref, T, RefNode> TypeRegistry<Ref, T> where Ref: RcLike<Node<Ref, T>, Output=RefNode>,
    RefNode: Deref<Target=Node<Ref, T>> + 'a,
    RefNode: Clone,
{
    pub fn new() -> Self {
        TypeRegistry { list: List::new() }
    }

    pub fn register(&self, t: T) -> Self {
        TypeRegistry { list: self.list.push(t) }
    }
}

// Define a registry for generic functions
pub struct FunctionRegistry<Ref, F> where Ref: RcLike<Node<Ref, F>> {
    list: List<Ref, F>,
}

impl<'a, Ref, F, RefNode> FunctionRegistry<Ref, F> where Ref: RcLike<Node<Ref, F>, Output=RefNode>,
    RefNode: Deref<Target=Node<Ref, F>> + 'a,
    RefNode: Clone,
{
    pub fn new() -> Self {
        FunctionRegistry { list: List::new() }
    }

    pub fn register(&self, f: F) -> Self {
        FunctionRegistry { list: self.list.push(f) }
    }
}

fn event_trigger<'a, A, F: for<'b> ApplyTo<'b, A> + Clone>(registry: &FunctionRegistry<Rc_, F>, arg: &'a A) -> Vec<<F as ApplyTo<'a, A>>::Output> {
    let mut node = registry.list.head.as_ref();
    let mut funcs = Vec::new();
    while let Some(ref current_node) = node {
        funcs.push(current_node.elem.clone());
        node = current_node.next.as_ref();
    }
    map1(funcs, arg)
}

// ApplyTo is a trait that provides a function that applies a strategy to a value.
pub trait ApplyTo<'a, A> {
    type Output;
    fn apply(&self, arg: &'a A) -> Self::Output;
}

pub fn apply<'a, A, S>(arg: &'a A, strategy: &'a S) -> <S as ApplyTo<'a, A>>::Output 
where for<'b> <S as ApplyTo<'b, A>>::Output: 'a, S: for<'b> ApplyTo<'b, A>
{
    strategy.apply(arg)
}

pub trait Configure<'a, Ref, T, F> where Ref: RcLike<Node<Ref, T>> + 'a + RcLike<Node<Ref, F>> + RcLike<Node<Ref, F>>, F: for<'b> ApplyTo<'b, T> + Clone + 'a {
    type Output;
    fn configure(&self, type_registry: &TypeRegistry<Ref, T>, function_registry: &FunctionRegistry<Ref, F>) -> Self::Output;
}

pub fn configure<'a, Ref, T, F, C>(type_registry: &TypeRegistry<Ref, T>, function_registry: &FunctionRegistry<Ref, F>, config: &'a C) -> <C as Configure<'a, Ref, T, F>>::Output
where Ref: RcLike<Node<Ref, T>> + 'a + RcLike<Node<Ref, F>> + RcLike<Node<Ref, F>>, F: for<'b> ApplyTo<'b, T> + Clone + 'a, C: Configure<'a, Ref, T, F>
{
    config.configure(type_registry, function_registry)
}

// PhantomData is invariant in its type parameter, so we need to wrap it in a newtype
#[derive(Copy, Clone)]
pub struct Id<'id>(PhantomData<Id_<'id>>); 
impl<'id> Id<'id> {
    pub fn new() -> Self { Id(PhantomData) }
}
type Id_<'id> = PhantomData<::std::cell::Cell<&'id mut ()>>; 

#[derive(Copy, Clone)]
pub struct Indices<'id> {
    _id: Id<'id>,
    min: usize,
    max: usize,
}
impl<'id> Indices<'id> {
    pub fn iter(&self) -> Indices<'id> { Indices { _id: self._id, min: self.min, max: self.max } }
}
pub fn indices<Array, F, Out, T>(arr: Array, f: F) -> Out
where F: for<'id> FnOnce(Indexer<'id, Array, T>, Indices<'id>) -> Out,
        Array: Deref<Target = [T]>, 
{
    let len = arr.len();
    let indexer = Indexer { _id: Id(PhantomData), arr: arr, _marker: PhantomData };
    let indices = Indices { _id: Id(PhantomData), min: 0, max: len };
    f(indexer, indices)
}
#[derive(Copy, Clone)]
pub struct Index<'id> {
    _id: Id<'id>,
    idx: usize,
} impl<'id> Index<'id> {
    pub fn new(idx: usize) -> Self { Index { _id: Id(PhantomData), idx } }
} impl<'id> Index<'id> {
    pub fn index(&self) -> usize { self.idx }
}
pub struct Indexer<'id, Array: Deref<Target=[T]>, T> { 
    _id: Id<'id>,
    arr: Array,
    _marker: PhantomData<T>,
}
impl<'id, 'a, T> Indexer<'id, &'a [T], T> {
    pub fn get(&self, idx: Index<'id>) -> Option<&'a T> {
        if idx.idx < self.arr.len() {
            Some(&self.arr[idx.idx])
        } else {
            None
        }
    }
}
impl<'id, 'a, T> Indexer<'id, &'a mut [T], T> {
    pub fn get_mut<'b>(&'b mut self, idx: Index<'id>) -> Option<&'b mut T> {
        if idx.idx < self.arr.len() {
            Some(&mut self.arr[idx.idx])
        } else {
            None
        }
    }
}
impl<'id, Array: Deref<Target=[T]>, T> Indexer<'id, Array, T> {
    pub fn len(&self) -> usize { self.arr.len() }
}
impl<'id, Array: Deref<Target=[T]>, T> Indexer<'id, Array, T> {
    pub fn indices(&self) -> Indices<'id> {
        Indices { _id: self._id, min: 0, max: self.arr.len() }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait RcLike2<T> {
    fn new(t: T) -> Self;
    fn borrow(&self) -> &T;
    fn borrow_mut(&mut self) -> &mut T;
    // fn into_inner(self) -> T;
}
impl<T> RcLike2<T> for Rc<T> {
    fn new(t: T) -> Self { Rc::new(t) }
    fn borrow(&self) -> &T { &*self }
    fn borrow_mut(&mut self) -> &mut T { Rc::get_mut(self).unwrap() }
    // fn into_inner(self) -> T { Rc::try_unwrap(self).unwrap() }
}
// pub trait RcLike<T> { fn new(data: T) -> Self::Output;
//     type Output;
// }



















#[macro_export]
macro_rules! hlist {
    () => { HNil };
    ($head:expr) => { HCons { head: $head, tail: HNil } };
    ($head:expr, $($tail:expr),+) => { HCons { head: $head, tail: hlist!($($tail),+) } };
}


// Let's test it out
#[cfg(test)]
mod test_macro {
    use super::*;
    use std::any::TypeId;


    // #[test]
    // fn test_node() {
    //     let mut type_registry = TypeRegistry::new();
    //     let mut function_registry = FunctionRegistry::new();
    //     let node = node!(MyNode, i32, |x| x + 1);
    //     let node_ref = type_registry.register(node);
    //     let node_ref2 = type_registry.register(node);
    //     assert!(node_ref == node_ref2);

    // }


   


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
    fn test_hlist_node() {


    }

}



mod test_cursor {
    use super::*;


    #[test]
    fn test_type_constructor() {
        let hlist: HCons<u32, HCons<u32, HNil>> = HCons { head: 0, tail: HCons { head: 1, tail: HNil } };
        let _: <Head_<u32, HCons<u32, HNil>> as LifeType>::Output = hlist.head;
        let _: <Tail_<u32, HCons<u32, HNil>> as LifeType>::Output = hlist.tail;
    }

    #[test]
    fn test_ref() {
        let mut x = 42;
        let r = _hack_ref(&x);
        assert_eq!(*r, 42);
    }

    #[test]
    fn test_ref_mut() {
        let mut x = 42;
        let r = _hack_ref_mut(&mut x);
        assert_eq!(*r, 42);
    }

    // #[test]
    // fn test_cur() {
    //     let mut x = [1, 2, 3, 4];
    //     let cur = Cursor { slice: &mut x, index: 0 };
    //     let cur = _cur::<u8>(cur);
    //     assert_eq!(*cur.slice, [1, 2, 3, 4]);
    //     assert_eq!(cur.index, 0);
    // }


    // #[test]
    // fn test_cur2() {
    //     let mut buf = [0u8; 10];
    //     let mut cur = Cursor { slice: &mut buf, index: 0 };
    //     cur.slice[0] = 1;
    //     cur.slice[1] = 2;
    //     cur.index += 1;
    //     assert_eq!(cur.slice[0], 1);
    //     assert_eq!(cur.slice[1], 2);
    //     assert_eq!(cur.index, 1);

    // }
}


// Unit test for the RefIterator trait and the Iter and IterMut structs
mod test_ref_iter {
    use super::{RefIterator, Iter, IterMut, List, Rc_, Arc_};

    #[test]
    fn test_list_tree() {
        let list: List<Rc_, u32> = List::new().push(0).push(1).push(2).tail();
        assert_eq!(list.head.unwrap().elem, 1); // 1
        let list: List<Arc_, u32> = List::new().push(10).push(11).push(12).tail();
        assert_eq!(list.head.unwrap().elem, 11); // 11
    }


    #[test]
    fn test_iter() {
        let mut iter = Iter { slice: &[1, 2, 3] };
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut iter = IterMut { slice: &mut [1, 2, 3] };
        assert_eq!(iter.next(), Some(&mut 1));

        // One that mutates the value
        if let Some(x) = iter.next() {
            *x = 4;
            assert_eq!(x, &mut 4)
        }
        assert_ne!(iter.slice, &[1, 2, 3]);
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);

    }
}

#[cfg(test)]
mod test_indexer {

    use super::*;

    // test Indexer
    #[test]
    fn test_indexer() {
        let arr = [1, 2, 3];
        let indexer = Indexer::<&[i32], i32> { _id: Id(PhantomData), arr: &arr, _marker: PhantomData };
        let idx = Index { _id: Id(PhantomData), idx: 1 };
        assert_eq!(indexer.get(idx), Some(&2));
        let idx = Index { _id: Id(PhantomData), idx: 3 };
        assert_eq!(indexer.get(idx), None);
    }


    // test Indexer mut
    #[test]
    fn test_indexer_mut() {
        let mut arr = [1, 2, 3];
        let mut indexer = Indexer::<&mut [i32], i32> { _id: Id(PhantomData), arr: &mut arr, _marker: PhantomData };
        let idx = Index { _id: Id(PhantomData), idx: 1 };
        assert_eq!(indexer.get_mut(idx), Some(&mut 2));
        let idx = Index { _id: Id(PhantomData), idx: 3 };
        assert_eq!(indexer.get_mut(idx), None);
    }




    #[test]
    fn test_hlist() {
        let hlist: HCons<u32, HCons<u32, HNil>> = HCons { head: 0, tail: HCons { head: 1, tail: HNil } };
        assert_eq!(hlist.head, 0);
        assert_eq!(hlist.tail.head, 1);
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

    


    // #[test]
    // fn test_indices<'a>() {
    //     // use indexing::indices;
    
    //     let arr1: &[u32] = &[1, 2, 3, 4, 5];
    //     let arr2: &[u32] = &[1, 2, 3, 4, 5, 6];

    //     let mut it = indices(arr1, |arr, mut it| {
    //         let a = it.next().unwrap();
    //         let b = it.next_back().unwrap();
    //         (arr.get(a).unwrap(), arr.get(b).unwrap()) 
    //     });
    //     let x = it.0;
    //     let y = it.1;
    //     assert!(x == &1 && y == &5);
    //     assert_eq!(indices(arr1, |arr, mut it| {
    //         let a = it.next().unwrap();
    //         let b = it.next_back().unwrap();
    //         (arr.get(a).unwrap(), arr.get(b).unwrap())
    //     }), (&1, &5));

    //     let (x, y) = indices(arr1, |arr, mut it| {
    //         let a = it.next().unwrap();
    //         let b = it.next_back().unwrap();
    //         (arr.get(a).unwrap(), arr.get(b).unwrap())
            
    //     });
    //     assert!(x == &1 && y == &5);
    //     assert_eq!(indices(arr1, |arr, mut it| {
    //         let a = it.next().unwrap();
    //         let b = it.next_back().unwrap();
    //         (arr.get(a).unwrap(), arr.get(b).unwrap())
    //     }), (&1, &5));

    // }



    // fn test_all() {
    //     // use indexing::indices;
    
    //     let arr1: &[u32] = &[1, 2, 3, 4, 5];
    //     let arr2: &[u32] = &[10, 20, 30];
    
    //     // concurrent iteration (hardest thing to do with iterators)
    //     indices(arr1, |arr1, it1| {
    //         indices(arr2, move |arr2, it2| {
    //             for (i, j) in it1.zip(it2) {
    
    //                 assert!(arr1.get(i).unwrap() + arr2.get(j).unwrap() == 11);
    //                 assert!(arr1.get(i).unwrap() + arr2.get(j).unwrap() == 21);
    //             }
    //         });
    //     });
    
    //     // can hold onto the indices for later, as long they stay in the closure
    //     let _a = indices(arr1, |arr, mut it| {
    //         let a = it.next().unwrap();
    //         let b = it.next_back().unwrap();
    
    //         (arr.get(a).unwrap(), arr.get(b).unwrap())
    //     });
    
    //     // can get references out, just not indices
    //     let (x, y) = indices(arr1, |arr, mut it| {
    //         let a = it.next().unwrap();
    //         let b = it.next_back().unwrap();
    //         (arr.get(a).unwrap(), arr.get(b).unwrap())
    //     });
    //     assert!(x == &1 && y == &5);
    
    // }




}
#[cfg(test)]
#[test]
fn test_eventer() {
    // Create a registry of generic types
    let type_registry: TypeRegistry<Rc_, i32> = TypeRegistry::new();
    let type_registry = type_registry.register(42);
    type_registry.register(43);
    // assert!(type_registry.list.head.as_ref().unwrap().elem == 42); 
    // assert!(type_registry.list.head.as_ref().unwrap().next.as_ref().unwrap().elem == 42);


    // Create a registry of generic functions
    let function_registry: FunctionRegistry<Rc_, Identity> = FunctionRegistry::new();
    let function_registry = function_registry.register(Identity);
    function_registry.register(Identity);
    assert!(function_registry.list.head.as_ref().unwrap().elem.apply(&42) == 42);
    let function_registry = function_registry.register(Identity);   
    assert!(function_registry.list.head.as_ref().unwrap().elem.apply(&42) == 42);
}