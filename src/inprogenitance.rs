


// Rust does not support inheritance, but it has trait objects that can be used to achieve similar, or even better, functionality.

// Introducing, Inprogenitance. Experimental and incomplete.

use std::any::Any;
use std::marker::PhantomData;

pub trait Inprogenitance {
    type Progeny<'a>: 'a where Self: 'a;

    fn root<'a>(&'a self) -> Self::Progeny<'a> where Self: 'a;
    fn progenitor<'a>(&'a self, progeny: Self::Progeny<'a>) -> Option<Self::Progeny<'a>>;
    // fn perform_operation<'a>(&'a self, progeny: Self::Progeny<'a>) -> Self::Progeny<'a>; 
}

// Progenitor
pub trait Progenitor<'a, T: 'a + Clone, R: Clone> {
    fn progeny(&self) -> Vec<Progeny<'a, T, R>>;
}

// Progenation
pub trait Progenation<'a, T: 'a + Clone, R: Clone> {
    fn progenate(&self, progeny: Progeny<'a, T, R>) -> Self;
}

pub struct Progeny<'a, T, R> 
where
    T: 'a + Clone,
    R: 'a + Clone,
{
    pub(crate) value: &'a T,
    pub(crate) progenitor: Option<&'a dyn Progenitor<'a, T, R>>, 
    pub(crate) operations: Vec<Box<dyn Fn(&'a T) -> R>>,  
    pub(crate) result: Option<R>,
}

#[derive(Debug)]
pub struct InprogenitanceImpl<'a, T: Clone, R: Clone> {
    pub value: T,
    pub progeny: Vec<Progeny<'a, T, R>>, 
    pub _marker: PhantomData<&'a T>,
}

impl<'a, T: Clone, R: Clone> InprogenitanceImpl<'a, T, R> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            progeny: vec![],
            _marker: PhantomData,
        }
    }

    pub fn progeny(&self) -> Vec<Progeny<'a, T, R>> {
        self.progeny.clone()
    }

    pub fn progenate(&mut self, progeny: Progeny<'a, T, R>) {
        self.progeny.push(progeny);
    }

    pub fn value(&self) -> T {
        self.value.clone()
    }

    pub fn result(&self) -> Option<R> {
        None
    }
}


// MyInprogenitanceBuilder is a builder for InprogenitanceImpl.
pub struct MyInprogenitanceBuilder<'a, T: 'a + Clone, R: Clone> {
    pub value: Option<T>,
    pub progeny: Option<Progeny<'a, T, Option<R>>>,
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

// MyInprogenitanceBuilder strategy




// MyInprogenitance is a wrapper around InprogenitanceImpl that implements Progenitor, Operation, Result, and Value.
// It is the only way to access InprogenitanceImpl. Which is useful for encapsulation.
pub struct MyInprogenitance<'a, T: 'a + Clone, R: Clone> {
    pub(crate)inprogenitance: InprogenitanceImpl<'a, T, R>,
}

impl<'a, T: Clone, R> MyInprogenitance<'a, T, R>
where
    T: 'a,
    R: 'a + Clone,
{
    pub fn new() -> MyInprogenitanceBuilder<'a, T, R> {
        MyInprogenitanceBuilder::new()
    }

    pub fn progeny(&self) -> Vec<Progeny<'a, T, R>> {
        self.inprogenitance.progeny()
    }

    pub fn progenate(&mut self, progeny: Progeny<'a, T, R>) {
        self.inprogenitance.progenate(progeny);
    }

    pub fn value(&self) -> T {
        self.inprogenitance.value()
    }

    pub fn result(&self) -> Option<R> {
        self.inprogenitance.result()
    }



    pub fn root(&'a self) -> Progeny<'a, T, R> {
        if let Some(progeny) = self.inprogenitance.progeny().first() {
            progeny.clone()
        } else {
            Progeny {
                value: &self.inprogenitance.value,
                progenitor: None,
                operations: vec![],
                result: None,
            }
        }
    }

}


// compute the result of the progeny.
pub trait Result<'a, T: 'a + Clone, R: Clone> {
    fn compute_result(&self, progeny: Progeny<'a, T, R>) -> R;
}
impl<'a, T: Clone, R: Clone> Result<'a, T, R> for MyInprogenitance<'a, T, R> {
    fn compute_result(&self, progeny: Progeny<'a, T, R>) -> R {
        progeny.result.unwrap()
    }
}

// Combinator //
pub trait Combinator<'a, T: 'a + Clone, R: Clone> {
    fn combine(&self, progeny: Progeny<'a, T, R>) -> Progeny<'a, T, R>;
}
impl<'a, T: Clone, R: Clone> Combinator<'a, T, R> for MyInprogenitance<'a, T, R> {
    fn combine(&self, progeny: Progeny<'a, T, R>) -> Progeny<'a, T, R> {
        progeny
    }
}

// A trait that associates a progenitor with a value.
pub trait Value<'a, T: 'a + Clone, R> {
    fn value(&self) -> T;
}


// blanket implementation for Value
impl<'a, T: 'a + Clone, R: Clone> Value<'a, T, R> for InprogenitanceImpl<'a, T, R> {
    fn value(&self) -> T {
        self.value.clone()
    }
}

// Deref and DerefMut //
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

// Debuggers, Cloners, and Displayers //
use std::fmt::Debug;
impl<'a, T: 'a + Clone + Debug, R: Clone + Debug> Debug for MyInprogenitance<'a, T, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MyInprogenitance")
            .field("value", &self.value)
            .field("progeny", &self.progeny)
            .finish()
    }
}
// 
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
// 
impl<'a, T: Clone + PartialEq, R: Clone> PartialEq for Progeny<'a, T, R> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

// Clone for Progeny
impl<'a, T: Clone, R: Clone> Clone for Progeny<'a, T, R> {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            progenitor: self.progenitor.clone(),
            operations: vec![],
            result: self.result.clone(),
        }
    }
}

// possible implementation of Progenitor
impl<'a, T: Clone, R: Clone> Progenitor<'a, T, R> for MyInprogenitance<'a, T, R> {
    fn progeny(&self) -> Vec<Progeny<'a, T, R>> {
        self.progeny.clone()
    }
}




















