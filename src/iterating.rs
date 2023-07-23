
// use std::ops::Deref;




// ContextExtendingIterator
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


// Super - The trait that is implemented by the trait that is being extended.
pub trait Super<'a> { type Super: 'a;
    
    fn super_(&'a self) -> &'a Self::Super;  
}

// Generic Associated Type (GAT) - Bound to the lifetime of the trait object.
pub trait GatItem<'a> { type Item; }

pub trait IntoCExtrator<'a> {

    // Two associated types are declared here. The first is Item, which is the type of the
    // iterator's elements. The second is IntoIter, which is the type of the iterator itself,
    // binding the iterator to the lifetime of the trait object.
    type Item;
    type IntoIter: CExtrator<'a, Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter; 
}

// Iter for ContextExtendingIterator is implemented for any type that implements IntoIterator. 
//
pub trait CExtratorIter<'a> {  
    type Item;  
    type Iter: CExtrator<'a, Item = Self::Item>;

    fn iter(&'a self) -> Self::Iter;
}

// Map 
pub struct Map<'a, I, F> where I: CExtrator<'a>,
    F: FnMut(&mut <I as CExtrator<'a>>::Item),
{
    iter: I,
    f: &'a F,
}

// Gat for Map
impl<'a, I, F> GatItem<'a> for Map<'a, I, F>
where
    I: CExtrator<'a>,
    F: FnMut(&mut <I as CExtrator<'a>>::Item),
{
    type Item = <I as CExtrator<'a>>::Item;
}

// Super for Map
impl<'a, I, F> Super<'a> for Map<'a, I, F>
where
    I: CExtrator<'a>,
    F: FnMut(&mut <I as CExtrator<'a>>::Item),
{
    type Super = I::Gats;

    fn super_(&'a self) -> &'a Self::Super {
        self.iter.super_()
    }
}


