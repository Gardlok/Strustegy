

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

// A Strategy for the new pointer type 
trait PtrStrat { type Pointer<T>: Deref<Target=T> + Sized;  
    fn new<T>(obj: T) -> Self::Pointer<T>;
}

// A Function that takes a closure and returns the closure
// This is used to create a new function that has a lifetime
// that is related to the lifetime of the Id.
// Example: let my_fn = new_fn::<'id, _>(|| println!("Hello World"));
fn _fn<'id, F>(f: F) -> F where F: FnMut() + 'id { f }


// List Component
struct List<T, P: PtrStrat>(P::Pointer<Elem<Ctor<T, P>>>); 
//
// DeReference the pointer to the list
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

// Pointer constructor for a linked list (or stepper)
struct Ctor<T, P: PtrStrat> {
    here: T,
    next: P::Pointer<Elem<Ctor<T, P>>>,
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
// Constructor function for the list type
fn list_rc<T>(here: T, next: ListRc<T>) -> ListRc<T> {
    List(Rc::new(Elem::Cons(Ctor { here, next: next.0 })))
}
// Get the head of the list
//
// This function is recursive and uses pattern matching to get the head of the list.
fn head_rc<T>(list: &ListRc<T>) -> Option<&T> {
    // To dereference the pointer, we use the deref method from the Deref trait
    // The deref method is automatically called when we use the * operator
    match &*list.0 {
        // The head of the list is the first element of the constructor
        Elem::Cons(ctor) => Some(&ctor.here),
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
        Elem::Cons(ctor) => Some(List(ctor.next.clone())),
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
        Elem::Cons(ctor) => 1 + len_rc(&List(ctor.next.clone())),
        Elem::Nil => 0,
    }
}



// Function Proxy 
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
    type Func<'a, T> = Box<dyn Fn(&'a T) -> T + 'a> where T: 'a;
}

//

struct Strat<T>(T, bool); 
trait Strategic {
    type Strategy;
    type Pointer<T>: Deref<Target=T> + Sized;  
    fn new<T>(obj: T) -> Self::Pointer<T>;
}
impl<T> Deref for Strat<T> { 
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Because Count 
impl<T> Strategic for Count<T> {
    type Strategy = Strat<T>;
    type Pointer<T2> = Strat<T2>;
    fn new<T3>(obj: T3) -> Self::Pointer<T3> { 
        Strat(obj, false)
    }
}

fn apply<S: Strategic + Applicable>(s: S) -> Option<S::Output> {  
    s.apply()
}

trait Applicable {
    type Output;
    fn apply(&self) -> Option<Self::Output>;
    fn inc(&self);
}

struct Count<T>(T, u32);
impl<T> Applicable for Count<T> {
    type Output = u32;
    fn apply(&self) -> Option<Self::Output> {  
        Some(self.1)
    }
    fn inc(&self) { Some(self.1 + 1); }
}
impl<T> Deref for Count<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        let _ = &self.inc();
        &self.0
    }
}

// Test Applicable
#[test]
fn test_applicable() {
 
    let count = Count(0, 0);
    count.inc();
 
    let result = apply(count);
    assert_eq!(result, Some(1));
    
}





#[test]
fn this () {
    fn generic_closure<F: Fn(i32)>(f: F) {  
        f(0);
        f(1);
    }

    generic_closure(|x| println!("{}", x)); // A
    generic_closure(|x| { // B
        let y = x + 2;
        println!("{}", y);
    });

    fn closure_object(f: &dyn Fn(i32)) {
        f(0);
        f(1);
    }
    closure_object(&|x| println!("{}", x));
    closure_object(&|x| {
        let y = x + 2;
        println!("{}", y);
    });

    generic_closure((&|x| { 
        let y = x + 2;
        println!("{}", y);
    }) as &dyn Fn(_));

}

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

