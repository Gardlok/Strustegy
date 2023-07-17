
use std::rc::Rc;
use std::ops::Deref;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::fmt::{Debug, Formatter};
use std::fmt::{Display, Error};
use std::ops::Add;

pub trait PtrStrat { type Pointer<T>: Deref<Target=T> + Sized;  
    fn new<T>(obj: T) -> Self::Pointer<T>;
}

#[derive(Debug, PartialEq)]
pub struct List<T, P: PtrStrat>(P::Pointer<Elem<Ctor<T, P>>>); 
//
impl<T, P: PtrStrat> Deref for List<T, P> {
    type Target = Elem<Ctor<T, P>>;

    // Dereference the pointer to the target type
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

#[derive(Debug)]
pub enum Elem<T> {
    Cons(T),
    Nil
}
impl<T> Elem<T> {
    pub fn new_cons(t: T) -> Self { Elem::Cons(t) }
    pub fn new_nil() -> Self { Elem::Nil }
}
pub struct Ctor<T, P: PtrStrat> {
    pub cur: T,
    pub nex: P::Pointer<Elem<Ctor<T, P>>>,
}
//
impl<T, P: PtrStrat> Ctor<T, P> {
    pub fn new(cur: T, nex: P::Pointer<Elem<Ctor<T, P>>>) -> Self {
        Ctor { cur, nex }
    }
}
impl<T, P: PtrStrat> Deref for Ctor<T, P> {
    type Target = T;

    // Dereference the pointer to the target type
    fn deref(&self) -> &Self::Target {
        &self.cur
    }
}

// PartialEq implementation for the Ctor type
impl<T: PartialEq, P: PtrStrat> PartialEq for Ctor<T, P> {
    fn eq(&self, other: &Self) -> bool {
        self.cur == other.cur
    }
}

// PartialEq implementation for the Elem type
impl<T: PartialEq> PartialEq for Elem<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Elem::Cons(x), Elem::Cons(y)) => x == y,
            (Elem::Nil, Elem::Nil) => true,
            _ => false,
        }
    }
}

impl<T: Debug> Debug for ListRcWrapper<T> where Ctor<T, RcPointerStrategy>: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T: Debug> Debug for Ctor<T, RcPointerStrategy> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ctor {{ cur: {:?}, nex: {:?} }}", self.cur, self.nex)
    }
}

impl<T: Clone> Deref for ListRcWrapper<T> {
    type Target = ListRc<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// 
#[derive(Debug, PartialEq)]
pub struct RcPointerStrategy;
impl PtrStrat for RcPointerStrategy { type Pointer<T> = Rc<T>; 
    fn new<T>(obj: T) -> Rc<T> {
        Rc::new(obj)
    }
}

pub type ListRc<T> = List<T, RcPointerStrategy>;
fn list_rc<T>(cur: T, nex: ListRc<T>) -> ListRc<T> {
    List(Rc::new(Elem::Cons(Ctor { cur, nex: nex.0 })))
}
// Get the head of the list
//
// 
fn head_rc<T>(list: &ListRc<T>) -> Option<&T> {
    match &*list.0 {
        Elem::Cons(ctor) => Some(&ctor.cur),
        Elem::Nil => None,
    }
}
// Get the tail of the list
//
//
fn tail_rc<T>(list: &ListRc<T>) -> Option<ListRc<T>> {
    match &*list.0 { 
        Elem::Cons(ctor) => Some(List(ctor.nex.clone())),
        Elem::Nil => None,
    }
}
// Get the length of the list
//
//
fn len_rc<T>(list: &ListRc<T>) -> usize {
    match &*list.0 { 
        // The length of the list is the length of the tail plus one
        Elem::Cons(ctor) => 1 + len_rc(&List(ctor.nex.clone())),
        Elem::Nil => 0,
    }
}

// Apply a function to each element of the list and return boolean if all elements
// return true.
//
// This function is recursive and uses pattern matching to apply a function to each
// element of the list and return boolean if all elements return true.
fn all_rc<T>(list: &ListRc<T>, func: impl Fn(&T) -> bool) -> bool {
    match &*list.0 { 
        // The purpose of the all function is to return true if the function returns
        // true for the head and the all function returns true for the tail.
        // Example: all(&List(Cons(Ctor { cur: 1, nex: Cons(Ctor { cur: 2, nex: Nil }) })), |x| *x > 0) == true
        Elem::Cons(ctor) => func(&ctor.cur) && all_rc(&List(ctor.nex.clone()), func),
        Elem::Nil => true,
    }
}

// Apply a function to each element of the list and return boolean if any element
// returns true.
//
// If any element returns true.
fn any_rc<T>(list: &ListRc<T>, func: impl Fn(&T) -> bool) -> bool {
    match &*list.0 { 
        Elem::Cons(ctor) => func(&ctor.cur) || any_rc(&List(ctor.nex.clone()), func),
        Elem::Nil => false,
    }
}

// Apply a function to each element
//
// Recursively apply a function to each element of the list and return a new list.
fn map_rc<T, U>(list: &ListRc<T>, func: impl Fn(&T) -> U) -> ListRc<U> {
    match &*list.0 { 
        // The map function returns a new list with the head of the list mapped
        // to the function and the tail of the list mapped to the function.
        Elem::Cons(ctor) => list_rc(func(&ctor.cur), map_rc(&List(ctor.nex.clone()), func)),
        Elem::Nil => List(Rc::new(Elem::Nil)),
    }
}

// Append two lists together.
//
// This function is recursive and uses pattern matching to append two lists together.
fn append_rc<T: Clone>(list1: &ListRc<T>, list2: &ListRc<T>) -> ListRc<T> {
    match &*list1.0 { 
        // The append function returns a new list with the head of the first list
        // and the tail of the first list appended to the second list.
        Elem::Cons(ctor) => list_rc(ctor.cur.clone(), append_rc(&List(ctor.nex.clone()), list2)),
        Elem::Nil => clone_rc(list2),
    }
}

// create a list from a vector
//
fn from_vec_rc<T: Clone>(vec: Vec<T>) -> ListRc<T> {
    match vec.len() {
        0 => List(Rc::new(Elem::Nil)),
        _ => list_rc(vec[0].clone(), from_vec_rc(vec[1..].to_vec())),
    }
}

// create a vector from a list
//
fn to_vec_rc<T: Clone>(list: &ListRc<T>) -> Vec<T> {
    match &*list.0 {
        Elem::Cons(ctor) => {
            let mut vec = to_vec_rc(&List(ctor.nex.clone()));
            vec.insert(0, ctor.cur.clone());
            vec
        }
        Elem::Nil => vec![],
    }
}

// Clone a list using Rc
//
fn clone_rc<T: Clone>(list: &ListRc<T>) -> ListRc<T> {
    match &*list.0 {
        Elem::Cons(ctor) => list_rc(ctor.cur.clone(), clone_rc(&List(ctor.nex.clone()))),
        Elem::Nil => List(Rc::new(Elem::Nil)),
    }
}

// Count the number of elements in a list that return true for a given function.
//
fn count_rc<T: Clone>(list: &ListRc<T>, func: impl Fn(&T) -> bool) -> usize {
    match &*list.0 {
        Elem::Cons(ctor) => {
            let mut count = 0;
            if func(&ctor.cur) {
                count += 1;
            }
            count += count_rc(&List(ctor.nex.clone()), func);
            count
        }
        Elem::Nil => 0,
    }
}

// Filter a list using a given function.
//
fn filter_rc<T: Clone>(list: &ListRc<T>, func: impl Fn(&T) -> bool) -> ListRc<T> {
    match &*list.0 {
        Elem::Cons(ctor) => {
            if func(&ctor.cur) {
                list_rc(ctor.cur.clone(), filter_rc(&List(ctor.nex.clone()), func))
            } else {
                filter_rc(&List(ctor.nex.clone()), func)
            }
        }
        Elem::Nil => List(Rc::new(Elem::Nil)),
    }
}

// --------------------------------------------------------------------------------

#[derive(PartialEq)]
pub struct ListRcWrapper<T>(ListRc<T>);  
impl<T: Clone> ListRcWrapper<T> {
    pub fn new() -> Self {
        ListRcWrapper(List(Rc::new(Elem::Nil)))
    }

    // head of the list
    pub fn head(&self) -> Option<&T> {
        head_rc(&self.0)
    }

    // Return the tail of the list
    pub fn tail(&self) -> Option<ListRcWrapper<T>> {

        // 1. If the list is not empty, return the tail of the list
        // 2. If the list is empty, return None
        if match &*self.0.0 {
            Elem::Cons(ctor) => true, 
            Elem::Nil => false,
        } {
            // safe to unwrap because of match above
            Some(ListRcWrapper(tail_rc(&self.0).unwrap())) 
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        len_rc(&self.0)
    }
    pub fn all(&self, func: impl Fn(&T) -> bool) -> bool {
        all_rc(&self.0, func)
    }
    pub fn any(&self, func: impl Fn(&T) -> bool) -> bool {
        any_rc(&self.0, func)
    }
    pub fn map<U>(&self, func: impl Fn(&T) -> U) -> ListRc<U> {
        map_rc(&self.0, func)
    }
    pub fn from_vec(vec: Vec<T>) -> Self {
        ListRcWrapper(from_vec_rc(vec))
    }
    pub fn to_vec(&self) -> Vec<T> {
        to_vec_rc(&self.0)  
    }
    pub fn clone(&self) -> Self {
        ListRcWrapper(clone_rc(&self.0))
    }

    // Get a count of the number of elements in the list that return true for the given function.
    pub fn count(&self, func: impl Fn(&T) -> bool) -> usize {
        count_rc(&self.0, func)
    }

    // Get a list of the elements in the list that return true for the given function.
    pub fn filter(&self, func: impl Fn(&T) -> bool) -> ListRcWrapper<T> {
        ListRcWrapper(filter_rc(&self.0, func))
    }

}

// Adding lists together
impl<T: Clone> Add<ListRcWrapper<T>> for ListRcWrapper<T> {
    type Output = ListRcWrapper<T>;
    fn add(self, other: ListRcWrapper<T>) -> ListRcWrapper<T> {
        ListRcWrapper(append_rc(&self.0, &other.0))
    }
}
impl<'a, T: Clone> Add<&ListRcWrapper<T>> for &'a ListRcWrapper<T> {
    type Output = ListRcWrapper<T>;
    fn add(self, other: &ListRcWrapper<T>) -> ListRcWrapper<T> {
        ListRcWrapper(append_rc(&self.0, &other.0))
    }
}

// Display
impl<T: Clone + Display> Display for ListRcWrapper<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.to_vec().iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(", "))
    }
}

// from Vec
impl<T: Clone> From<Vec<T>> for ListRcWrapper<T> {
    fn from(vec: Vec<T>) -> Self {
        ListRcWrapper::from_vec(vec)
    }
}

// from ListRc
impl<T: Clone> From<ListRc<T>> for ListRcWrapper<T> {
    fn from(list: ListRc<T>) -> Self {
        ListRcWrapper(list)
    }
}

// Build a list from a list of lists
impl<T: Clone> From<Vec<ListRcWrapper<T>>> for ListRcWrapper<T> {
    fn from(lists: Vec<ListRcWrapper<T>>) -> Self {
        lists.into_iter().fold(ListRcWrapper::new(), |acc, x| acc + x)
    }
}

// -------------------------------------------------------------------------------- //

// Test listRcWrapper
#[test]
fn test_list_rc_wrapper() {
    let list: ListRcWrapper<i32> = ListRcWrapper::from_vec(vec![1, 2, 3, 4, 5]);
    let list1: ListRcWrapper<i32> = ListRcWrapper::from_vec(vec![2, 4, 6, 8, 10]);
    let list2: ListRcWrapper<i32> = ListRcWrapper::new();
    assert_eq!(list2.len(), 0);
    assert_eq!(list2.head(), None);
    assert_eq!(list2.tail(), None);
    assert_eq!(list.len(), 5);
    assert_eq!(list.head(), Some(&1));
    assert_eq!(list.tail().expect("tail").len(), 4);
    assert_eq!(list.all(|x| *x < 6), true);
    assert_eq!(list.any(|x| *x > 5), false);
    assert_eq!(list.map(|x| *x * 2), list1.0);
    assert_eq!(list.to_vec(), vec![1, 2, 3, 4, 5]);
}

// Test the list
#[test] 
fn test_list_rc() {
    let list = list_rc(1, list_rc(2, list_rc(3, List(Rc::new(Elem::Nil)))));
    assert_eq!(head_rc(&list), Some(&1));
    assert_eq!(len_rc(&list), 3);
    assert_eq!(len_rc(&tail_rc(&list).unwrap()), 2);
}

// Testing Append
#[test]
fn test_add() {
    let list1: ListRcWrapper<i32> = ListRcWrapper::from_vec(vec![1, 2, 3, 4, 5]);
    let list2: ListRcWrapper<i32> = ListRcWrapper::from_vec(vec![2, 4, 6, 8, 10]);
    let list3: ListRcWrapper<i32> = ListRcWrapper::from_vec(vec![1, 2, 3, 4, 5, 2, 4, 6, 8, 10]);
    assert_eq!(&list1 + &list2, list3);
    assert!(list1 != list2);
}

// Testing count
#[test]
fn test_count() {
    let list1: ListRcWrapper<i32> = ListRcWrapper::from_vec(vec![1, 2, 3, 4, 5]);
    assert_eq!(list1.count(|x| *x % 2 == 0), 2);
    assert_eq!(list1.count(|x| *x % 2 == 1), 3);
}

// Test filter
#[test]
fn test_filter() {
    let list = ListRcWrapper::from_vec(vec![1, 2, 3, 4, 5]);
    let list2 = list.filter(|x| *x % 2 == 0);
    assert_eq!(list2.to_vec(), vec![2, 4]);
}

// Test clone
#[test]
fn test_clone() {
    let list = ListRcWrapper::from_vec(vec![1, 2, 3, 4, 5]);
    let list2 = list.clone();
    assert_eq!(list, list2);
}

// Test display
#[test]
fn test_display() {
    let list = ListRcWrapper::from_vec(vec![1, 2, 3, 4, 5]);
    assert_eq!(format!("{}", list), "1, 2, 3, 4, 5");
}

