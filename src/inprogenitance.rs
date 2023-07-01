


// Rust does not support inheritance, but it has trait objects that can be used to achieve similar, or even better, functionality.

// Introducing, Inprogenitance. Experimental and incomplete.

use std::any::Any;
use std::marker::PhantomData;

pub trait Inprogenitance {
    type Progeny<'a>: 'a where Self: 'a;

    fn root<'a>(&'a self) -> Self::Progeny<'a>;
    fn progenitor<'a>(&'a self, progeny: Self::Progeny<'a>) -> Option<Self::Progeny<'a>>;
    // fn perform_operation<'a>(&'a self, progeny: Self::Progeny<'a>) -> Self::Progeny<'a>; 
}

pub struct Progeny<'a, T, R> 
where
    T: 'a + Clone,
    R: 'a + Clone,
{
    value: &'a T,
    progenitor: Option<&'a dyn Progenitor<'a, T, R>>,
    operations: Vec<Box<dyn Fn(&'a T) -> R>>,  
    result: Option<R>,
}

#[derive(Debug)]
pub struct InprogenitanceImpl<'a, T: Clone, R: Clone> {
    value: T,
    progeny: Vec<Progeny<'a, T, R>>, 
    _marker: PhantomData<&'a T>,
}










// MyInprogenitanceBuilder is a builder for InprogenitanceImpl.
pub struct MyInprogenitanceBuilder<'a, T: 'a + Clone, R: Clone> {
    value: Option<T>,
    progeny: Option<Progeny<'a, T, Option<R>>>,
}
impl<'a, T: Clone, R> MyInprogenitanceBuilder<'a, T, R>
where
    T: 'a,
    R: 'a + Clone,
 {
    pub fn new() -> Self {
        Self {
            value: None,
            progeny: None,
        }
    }

    pub fn value(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }

    pub fn progeny(mut self, progeny: Progeny<'a, T, R>) -> Self {  
        self
    }

    pub fn build(self) -> Option<MyInprogenitance<'a, T, R>> {
        if self.value.is_some() {
            Some(MyInprogenitance {
                inprogenitance: InprogenitanceImpl {
                    value: self.value.unwrap(),
                    progeny: vec![],
                    _marker: PhantomData,
                },
            })
        } else {
            None
        }
    }
}

// MyInprogenitance is a wrapper around InprogenitanceImpl that implements Progenitor, Operation, Result, and Value.
// It is the only way to access InprogenitanceImpl. Which is useful for encapsulation.
pub struct MyInprogenitance<'a, T: 'a + Clone, R: Clone> {
    inprogenitance: InprogenitanceImpl<'a, T, R>,
}
impl<'a, T: Clone, R> MyInprogenitance<'a, T, R>
where
    T: 'a,
    R: 'a + Clone,
{
    pub fn new() -> MyInprogenitanceBuilder<'a, T, R> {
        MyInprogenitanceBuilder::new()
    }
}
// A trait that associates a vec of progeny with a progenitor.
pub trait Progenitor<'a, T: 'a + Clone, R: Clone> { // T: Clone
    fn progeny(&self) -> Vec<Progeny<'a, T, R>>; // Vec<Progeny>
}
// A trait that associates a progenitor with a result.
pub trait Result<'a, T: 'a + Clone, R> {
    fn result(&self) -> Option<R>;
}
// A trait that associates a progenitor with a value.
pub trait Value<'a, T: 'a + Clone, R> {
    fn value(&self) -> T;
}

// blanket implementation for Result
impl<'a, T: 'a + Clone, R: Clone> Result<'a, T, R> for InprogenitanceImpl<'a, T, R> {
    fn result(&self) -> Option<R> {
        None
    }
}
// blanket implementation for Value
impl<'a, T: 'a + Clone, R: Clone> Value<'a, T, R> for InprogenitanceImpl<'a, T, R> {
    fn value(&self) -> T {
        self.value.clone()
    }
}

// Implementations for MyInprogenitance //
use std::ops::Deref;
impl<'a, T: Clone, R: Clone> Deref for MyInprogenitance<'a, T, R> {
    type Target = InprogenitanceImpl<'a, T, R>;

    fn deref(&self) -> &Self::Target {
        &self.inprogenitance
    }
}
use std::ops::DerefMut;
impl<'a, T: Clone, R: Clone> DerefMut for MyInprogenitance<'a, T, R> 
where
    T: 'a,
    R: 'a + Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inprogenitance
    }
}
use std::fmt::Debug;
// MyInprogenitance implements Debug
impl<'a, T: 'a + Clone + Debug, R: Clone + Debug> Debug for MyInprogenitance<'a, T, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MyInprogenitance")
            .field("value", &self.value)
            .field("progeny", &self.progeny)
            .finish()
    }
}
// Progeny implements Debug
impl<'a, T: 'a + Clone + Debug, R: Clone + Debug> Debug for Progeny<'a, T, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match &self.result {
            Some(result) => format!("{:?}", result),
            None => String::from("None"),
        };
        f.debug_struct("Progeny")
            .field("value", &self.value)
            .field("result", &result)
            // TODO: progeny
            .finish()
    }
}
// impl for T = i32 and R = bool is provided to support equality checks.
impl<'a, T: Clone + PartialEq, R: Clone> PartialEq for Progeny<'a, T, R> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

// A new gat that provides a mutable reference to a progeny. 
pub struct ProgenyMut<'a, T: 'a + Clone, R: Clone> {
    progeny: &'a mut Progeny<'a, T, R>,
}
// A new gat that provides a mutable reference to a progenitor.
pub struct InprogenitanceMut<'a, T: 'a + Clone, R: Clone> {
    inprogenitance: &'a mut InprogenitanceImpl<'a, T, R>,
}

// MyInprogenitance pre







// Test code
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inprogenitance() {




        // Progeny<'a, T: Clone, R: Clone>
        let progeny: Progeny<'_, &i32, bool> = Progeny {
            value: &&1,
            progenitor: None,
            operations: vec![],  
            result: None, 
        };
        

        let inprogenitance: InprogenitanceImpl<'_, &i32, bool> = InprogenitanceImpl {
            value: &1,
            progeny: vec![progeny],
            _marker: PhantomData,
        };

        // create an instance 
        let mut my_inprogenitance: MyInprogenitance<'_, &i32, bool> = MyInprogenitance { inprogenitance };

        // test equality
        assert_eq!(my_inprogenitance.progeny.pop(), Some(Progeny {
            value: &&1,
            progenitor: None,
            operations: vec![],  
            result: None, 
        }));

        // test equality
        assert_eq!(my_inprogenitance.value, &1);





    }


}



















