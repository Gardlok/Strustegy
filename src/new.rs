

// New Strategy Idiom (WIP: Still being conceptualized) //
//////////////////////////////////////////////////////////

use std::rc::Rc;
use std::ops::Deref;
use std::ops::DerefMut;
use std::cell::RefCell;
use std::marker::PhantomData;

// An Id used to relate components via lifetimes/ownership
type Id<'id> = PhantomData<::std::cell::OnceCell<&'id mut ()>>; 
fn new_id<'id>() -> Id<'id> { Id::default() } 
fn new_id_ref<'id, 'a>(id: &'a Id<'id>) -> &'a Id<'id> { id }
fn new_id_mut<'id, 'a>(id: &'a mut Id<'id>) -> &'a mut Id<'id> { id }
fn new_id_box<'id>(id: Box<Id<'id>>) -> Box<Id<'id>> { id }
fn new_id_rc<'id>(id: Rc<Id<'id>>) -> Rc<Id<'id>> { id }
fn new_id_refcell<'id>(id: RefCell<Id<'id>>) -> RefCell<Id<'id>> { id }


// A Strategy for the new pointer type 
trait PtrStrat { type Pointer<T>: Deref<Target=T> + Sized;  
    fn new<T>(obj: T) -> Self::Pointer<T>;
}

// A Function that takes a closure and returns the closure
// This is used to create a new function that has a lifetime
// that is related to the lifetime of the Id.
// Example: let my_fn = new_fn::<'id, _>(|| println!("Hello World"));
fn _fn<'id, F>(f: F) -> F where F: FnMut() + 'id { f }

// List Component - This type is a linked list that holds a type. It is used to
// create a list of types that are related to each other via lifetimes.
//
struct List<T, P: PtrStrat>(P::Pointer<Elem<Ctor<T, P>>>); 
//
impl<T, P: PtrStrat> Deref for List<T, P> {
    type Target = Elem<Ctor<T, P>>;

    // Dereference the pointer to the target type
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

// Constructor function for a type that holds a type. Thi Nil constructor is for 
// provides a way to stop the recursion, and some level of type safety.
enum Elem<T> {
    Cons(T),
    Nil
}

// Constructor function for a type that holds a type. This is the constructor
// for the Cons type. It takes a type and a pointer to the next Cons type.
// Example: let my_struct = Ctor::new(5, &my_struct); 
//
struct Ctor<T, P: PtrStrat> {
    cur: T,
    nex: P::Pointer<Elem<Ctor<T, P>>>,
}
//
impl<T, P: PtrStrat> Ctor<T, P> {
    fn new(cur: T, nex: P::Pointer<Elem<Ctor<T, P>>>) -> Self {
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


// Function Proxy - This type is a proxy for a function. It allows for a function
// to be passed around without knowing the type of the function, arguments, or
// return type. 
//
trait FuncProxy {
    type Func<'a, T>: Fn(&'a T) -> T where T: 'a;
}

// Call the function by taking the proxy, the function, and the arguments.
// Example: let _ = call_func(&my_struct, func, &5);
fn call_func<'a, F: FuncProxy, T>(proxy: &F, func: F::Func<'a, T>, arg: &'a T) -> T {
    func(arg)
}

// Define a proxy for the function. This allows for the function to be
// passed around without knowing the type of the function.
// Example: let my_struct = NewFuncProxy1;
// Example: let func = Box::new(|x: &i32| *x * 2);
// Example: let result = call_func(&my_struct, func, &5);
struct NewFuncProxy1;
impl FuncProxy for NewFuncProxy1 {

    // The function is a trait object that implements the Fn trait. The 'a is a 
    // lifetime specifier that means that the function must live at least as 
    // long as the argument.
    type Func<'a, T> = Box<dyn Fn(&'a T) -> T + 'a> where T: 'a;  
}

//--------------------------------------------------------------------------------
// A Strategy for the new type - (Rc smart pointer in this case) 
// 
struct RcPointerStrategy;
impl PtrStrat for RcPointerStrategy { type Pointer<T> = Rc<T>;
    fn new<T>(obj: T) -> Rc<T> {
        Rc::new(obj)
    }
}

// A List type for the new strategy
type ListRc<T> = List<T, RcPointerStrategy>;
//  
// Constructor function for the list type
fn list_rc<T>(cur: T, nex: ListRc<T>) -> ListRc<T> {
    List(Rc::new(Elem::Cons(Ctor { cur, nex: nex.0 })))
}
// Get the head of the list
//
// This function is recursive and uses pattern matching to get the head of the list.
fn head_rc<T>(list: &ListRc<T>) -> Option<&T> {
    // To dereference the pointer, we use the deref method from the Deref trait
    // The deref method is automatically called when we use the * operator
    match &*list.0 {
        // The head of the list is the first element of the constructor
        Elem::Cons(ctor) => Some(&ctor.cur),
        Elem::Nil => None,
    }
}
// Get the tail of the list
//
// This function is recursive and uses pattern matching to get the tail of the list.
fn tail_rc<T>(list: &ListRc<T>) -> Option<ListRc<T>> {
    // To dereference the pointer, we use the deref method from the Deref trait
    // The deref method is automatically called when we use the * operator
    match &*list.0 { 
        // The tail of the list is the tail of the constructor
        Elem::Cons(ctor) => Some(List(ctor.nex.clone())),
        Elem::Nil => None,
    }
}
// Get the length of the list
//
// This function is recursive and uses pattern matching to get the length of the list.
fn len_rc<T>(list: &ListRc<T>) -> usize {
    // To dereference the pointer, we use the deref method from the Deref trait
    // The deref method is automatically called when we use the * operator
    match &*list.0 { 
        // The length of the list is the length of the tail plus one
        Elem::Cons(ctor) => 1 + len_rc(&List(ctor.nex.clone())),
        Elem::Nil => 0,
    }
}

// --------------------------------------------------------------------------------

use crate::prelude::{StrategyWithContext, StrategyFn};
use crate::strategy::DynStrategy;


type StratList<'a, T, S> = List<dyn for<'list> StrategyWithContext<'list, T> + 'a, S>;

// A List type for strategies with the same target
type ListStrategy<'a, T> = StratList<'a, T, RcPointerStrategy>;



// Test Test Test // 

// Test the function proxy
#[test] 
fn test_my_struct() {
    let my_struct = NewFuncProxy1;
    let func = Box::new(|x: &i32| *x * 2);
    let result = call_func(&my_struct, func, &5);
    assert_eq!(result, 10);

    let func = Box::new(|x: &i32| *x * 3);
    let result = call_func(&my_struct, func, &5);
    assert_eq!(result, 15);
}

// Test the list
#[test] 
fn test_list_rc() {
    let list = list_rc(1, list_rc(2, list_rc(3, List(Rc::new(Elem::Nil)))));
    assert_eq!(head_rc(&list), Some(&1));
    assert_eq!(len_rc(&list), 3);
    assert_eq!(len_rc(&tail_rc(&list).unwrap()), 2);
}

