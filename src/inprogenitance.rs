
use std::{mem, cmp, clone};
use std::marker::PhantomData;
use std::ops::Deref;
use std::fmt::Debug;
use std::sync::Arc;
use std::rc::Rc;





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

/// Represens types that have certain bounds.
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

// Kind: Lifetime -> Type   
pub trait LifeType<'a> { type Output; }
impl<'a, T> LifeType<'a> for &'a T { type Output = T; }
impl<'a, T> LifeType<'a> for &'a mut T { type Output = T; }


// -----------

// A node in the list.
fn clone_node<'a, Ref, T: Clone, RefNode>(node: &Node<Ref, T>) -> Node<Ref, T> where Ref: RcLike<Node<Ref, T>, Output=RefNode>,
    RefNode: Deref<Target=Node<Ref, T>> + 'a,
    RefNode: Clone,
{ 
    Node { elem: node.elem.clone(), 
           next: node.next.clone(), 
           prev: node.prev.clone() 
    } 
}

pub trait RcLike<T> { fn new(data: T) -> Self::Output;
    type Output;
}
impl<T> RcLike<T> for Rc<T> { fn new(data: T) -> Self::Output { Rc::new(data) } 
    type Output = Rc<T>; 
}
impl<T> RcLike<T> for Arc<T> { fn new(data: T) -> Self::Output { Arc::new(data) }
    type Output = Arc<T>;
}

#[derive(PartialEq, Debug)]
struct Rc_; struct Arc_; 
// For RcLike to Rc
impl<T> RcLike<T> for Rc_ { fn new(data: T) -> Self::Output { Rc::new(data) } 
    type Output = Rc<T>; 
}
// For RcLike to Arc
impl<T> RcLike<T> for Arc_ { fn new(data: T) -> Self::Output { Arc::new(data) }
    type Output = Arc<T>;
}

// The RcLike trait is implemented for any type that can be cloned into a reference counted pointer
struct Ref_<T>(PhantomData<T>); 
impl<'a, T: 'a> LifeType<'a> for Ref_<T> { type Output = &'a T; }
// They type returned is a reference to the type parameterized by the lifetime.
fn _ref<'a, T>(v: &'a T) -> <Ref_<T> as LifeType<'a>>::Output { v }

// When we want to return a mutable reference, it's actually a reference to a
// mutable type, so we need a new type for that. Balance is restored.
struct RefMut_<T>(PhantomData<T>); 
impl<'a, T: 'a> LifeType<'a> for RefMut_<T> { type Output = &'a mut T; }
//
fn _ref_mut<'a, T>(v: &'a mut T) -> <RefMut_<T> as LifeType<'a>>::Output { v }

// RefIterator, needing a new name, is a special kind of iterator that yields
// references to a type that is parameterized by the lifetime of the iterator.
trait RefIterator { 
    // TypeCtor is a type constructor that takes a lifetime and returns a type
    // that binds to that lifetime. Similar to a higher kinded type, and very
    // similar to a type constructor in Haskell. Simply put, magic.
    type TypeCtor;
    
    // Our next method takes a mutable reference to self and returns an Option
    // of the type constructor. The type constructor is parameterized by the
    // lifetime of the iterator, and the output of the type constructor is
    // bound to the parameterized lifetime.
    fn next<'a>(&'a mut self) -> Option<<Self::TypeCtor as LifeType<'a>>::Output>
        // Binds the lifetime of the output to the lifetime of the iterator
        where Self::TypeCtor: LifeType<'a>;
}

// IterMut is an iterator that yields mutable references to a slice of T (i.e. &mut [T])
struct IterMut<'a, T: 'a> { slice: &'a mut [T] }
// 
impl<'x, T> RefIterator for IterMut<'x, T> {  
    // Mutable reference type
    type TypeCtor = RefMut_<T>;
    fn next<'a>(&'a mut self) -> Option<<Self::TypeCtor as LifeType<'a>>::Output>
        //
        where Self::TypeCtor: LifeType<'a> 
    {
        // Same as above, but we return a mutable reference instead of an immutable one.
        if self.slice.is_empty() { 
            None
        } else {
            let (l, r) = mem::replace(&mut self.slice, &mut []).split_at_mut(1); 
            self.slice = r;
            Some(_ref_mut(&mut l[0]))
        }
    }
}

// The List struct is a doubly linked list
pub struct List<'a, Ref, T, C> where Ref: RcLike<Node<Ref, T>>, C: Fn(&'a T) -> bool {
    head: Option<Ref>,
    tail: Option<Ref>,
    func: C,
    _phantom: PhantomData<&'a T>,
}
impl<'a, Ref, T, C> List<'a, Ref, T, C> where Ref: RcLike<Node<Ref, T>>, C: Fn(&'a T) -> bool {
    pub fn new(func: C) -> Self {
        List { head: None, tail: None, func, _phantom: PhantomData }
    }
}

// The Node struct is a node in a doubly linked list
pub struct Node<Ref, T> where Ref: RcLike<Node<Ref, T>>,
{
    // The element stored in the node is a pointer (RcLike) to the node itself
    pub elem: T, 
    pub next: Option<<Ref as RcLike<Node<Ref, T>>>::Output>, 
    pub prev: Option<<Ref as RcLike<Node<Ref, T>>>::Output>,
}

// Cloning Node is actually to clone the reference counted pointer
impl<'a, Ref, T: Clone, RefNode> Clone for Node<Ref, T> where Ref: RcLike<Node<Ref, T>, Output=RefNode>,
    RefNode: Deref<Target=Node<Ref, T>> + 'a,
    RefNode: Clone,
{ 
    fn clone(&self) -> Self { 
        clone_node(self)
    }

    fn clone_from(&mut self, source: &Self)
    where
        Self: Sized, 
    {
        *self = clone_node(source);
    } 
}



