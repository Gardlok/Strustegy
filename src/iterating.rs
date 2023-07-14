
use std::ops::Deref;


// Super Trait
pub trait Super<'a> {
    type Super: 'a;
    
    fn super_(&'a self) -> &'a Self::Super;  
}

// Generic Associated Type
pub trait GatItem<'a> { type Item; }

// Gat for Map
impl<'a, I, F> GatItem<'a> for Map<'a, I, F>
where
    I: CExtrator<'a>,
    F: FnMut(&mut <I as CExtrator<'a>>::Item),
{
    type Item = <I as CExtrator<'a>>::Item;
}

// Lending Iterator
//                   
pub trait LendingIterator<'a> { type Item; 
    // In this case, we are returning a &'a mut T that is a field of the Map. Because of this,
    // it will be dropped when the Map struct is dropped. Therefore, the &'a mut T does not
    // outlive the Box<T>. Drop cleanup, just in case.
    fn next(&mut self, f: &'a dyn Fn(&mut Self, &mut Self::Item)) -> Option<Self::Item>;  
}
//
pub trait IntoCExtrator<'a> {
    type Item;
    type IntoIter: CExtrator<'a, Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter; 
}
// Iter for ContextExtendingIterator
//
pub trait CExtratorIter<'a> {  
    type Item;  
    type Iter: CExtrator<'a, Item = Self::Item>;

    fn iter(&'a self) -> Self::Iter;
}
// ContextExtendingIterator - 
//
pub trait CExtrator<'a>: 'a + for<'b> Super<'b, Super = Self::Gats> + for<'b> GatItem<'b>  
where Self: Sized,
{
    type Item;
    type Gats: 'a + for<'b> GatItem<'b, Item = <Self as CExtrator<'a>>::Item>; 
    type Super: 'a + for<'b> Super<'b, Super = Self::Gats> + for<'b> GatItem<'b, Item = <Self as CExtrator<'a>>::Item>;  

    fn gats(&'a self) -> Self::Gats;

    fn map<'b, G>(self, f: &'a G) -> &'a Map<'a, Self, G> 
    where
        G: FnMut(&mut <Self as CExtrator<'a>>::Item),  
        Self: Sized,
    {
        // It is unsafe because it is up to the caller to ensure that the &'a mut T does not 
        // outlive the Box<T>. In this case, we are returning a &'a mut T that is a field of the 
        // Map. Because of this,  it will be dropped when the Map struct is dropped. Therefore, 
        // the &'a mut T does not outlive the Box<T>. Drop cleanup, just in case.
        Box::leak(Box::new(Map { iter: self, f })) 
    }
}///////////////////////////////////////////////////////////////////
//                     Map Item Cleanup                           //
impl<'a, I, F> Drop for Map<'a, I, F> 
where I: CExtrator<'a>,
    F: FnMut(&mut <I as CExtrator<'a>>::Item) 
{
    fn drop(&mut self) where { self.iter.super_().deref().cleanup() }  
}
// 
pub trait Cleanup<'a> { fn cleanup(&'a mut self); }
impl<'a, T> Cleanup<'a> for &'a T { fn cleanup(&'a mut self) {} }   //
//////////////////////////////////////////////////////////////////////


// Map 
pub struct Map<'a, I, F> where I: CExtrator<'a>,
    F: FnMut(&mut <I as CExtrator<'a>>::Item),
{
    iter: I,
    f: &'a F,
}

pub trait MapTrait<'a> {
    type Item;
    type Super: 'a;
}
impl<'a, I, F> MapTrait<'a> for Map<'a, I, F> where I: CExtrator<'a>,
    F: FnMut(&mut <I as CExtrator<'a>>::Item) {
    type Item = <I as CExtrator<'a>>::Item;
    type Super = <I as CExtrator<'a>>::Super;
}
impl<'a, I, F> Map<'a, I, F> where I: CExtrator<'a, Super = ()>,
    F: FnMut(&mut <I as CExtrator<'a>>::Item) -> <I as CExtrator<'a>>::Super {
    pub fn new(iter: I, f: &'a F) -> Self { Self { iter, f } }  
}
impl<'a, I, F> Deref for Map<'a, I, F> where I: CExtrator<'a>,
    F: FnMut(&mut <I as CExtrator<'a>>::Item) { 
    type Target = I;
    fn deref(&self) -> &Self::Target { &self.iter }
}

// Map Iterator
//
pub fn map<'a, I, F>(iter: I, f: &'a F) -> Map<'a, I, F> where I: CExtrator<'a, Super = ()>,
    F: FnMut(&mut <I as CExtrator<'a>>::Item) {
    Map::new(iter, f)
}
pub fn map2<'a, I, F>(iter: I, f: &'a F) -> Map<'a, I, F> where I: CExtrator<'a, Super = ()>,
    F: FnMut(&mut <I as CExtrator<'a>>::Item) -> <I as CExtrator<'a>>::Super {
    Map::new(iter, f)
}











