
use std::marker::PhantomData;
use std::ops::Deref;


pub struct Map<I, F> {
    iter: I,
    f: F,
}
// GAT Trait 
//
pub trait Gat<'a> { 
	type Item;
}
// Superceding Trait 
//
pub trait Super<'a> {
    type Super: 'a;
    
    fn super_(&'a self) -> &'a Self::Super;
}
// Scoping Object 
//
pub struct Scope<'a, T>(PhantomData<&'a mut T>);
impl<'a, T> Scope<'a, T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}


// Lending Iterator
//                   
pub trait LendingIterator<'a> {
    type Item;

    fn next(&'a mut self, f: &'a dyn Fn(&mut Self, &mut Self::Item)) -> Option<Self::Item>;  
}

// ContextExtendingIterator - 
//
pub trait ContextExtendingIterator<'a>
where
    Self: Sized,
{
    type Item;
    type Gats: 'a + for<'b> Gat<'b, Item = Self::Item>;
    fn gats(&'a self) -> Self::Gats;
    fn map<'b, G>(self, f: G) -> Map<Self, G> 
    where
        G: FnMut(&mut Self::Item),
        Self: Sized,
    {
        Map { iter: self, f }
    }
}
impl<'a, I, F> ContextExtendingIterator<'a> for Map<I, F>  
where
    I: ContextExtendingIterator<'a>,
    F: FnMut(&mut I::Item),

{
    type Item = I::Item;
    type Gats = I::Gats;

    fn gats(&'a self) -> Self::Gats {
        self.iter.gats()
    }
    fn map<'b, G>(self, f: G) -> Map<Self, G> 
    where
        G: FnMut(&mut Self::Item),
        Self: Sized,
    {
        Map { iter: self, f }
    }
}

// OperationExtendingIterator 
//
pub trait OperationExtendingIterator<'a>
where
    Self: Sized,
{
    type Item;
    type Gats: 'a + for<'b> Gat<'b, Item = Self::Item>;
    fn gats(&'a self) -> Self::Gats;  
    fn map<'b, G>(self, f: G) -> Map<Self, G> 
    where
        G: FnMut(&mut Self::Item),
        Self: Sized,
    {
        Map { iter: self, f }
    }
}
impl<'a, I, F> OperationExtendingIterator<'a> for Map<I, F>  
where
    I: OperationExtendingIterator<'a>,
    F: FnMut(&mut I::Item),

{
    type Item = I::Item;
    type Gats = I::Gats;

    fn gats(&'a self) -> Self::Gats {
        self.iter.gats()
    }
    fn map<'b, G>(self, f: G) -> Map<Self, G> 
    where
        G: FnMut(&mut Self::Item),
        Self: Sized,
    {
        Map { iter: self, f }
    }
}