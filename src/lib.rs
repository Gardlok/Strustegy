
use std::error::Error;
use std::any::Any;



mod prelude {
    pub use crate::OpError;
    pub use crate::bind;
}





// Op Error //
//
#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub struct OpError {
    message: String,
}

impl OpError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for OpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "OpError: {}", self.message)
    }
}

impl Error for OpError {}
 
pub fn error<T>( msg: &str ) -> Result<T, OpError> { Err(OpError::new(msg)) }



// Bind (>>=) - chain operations
// 
pub fn bind<T, U, F>( x: Result<T, OpError>,  f: F ) -> Result<U, OpError> where F: FnOnce(T) -> Result<U, OpError> {
    match x { 
        Ok(x) => f(x), 
        Err(e) => Err(e),
    }
}
//
#[macro_export]
macro_rules! bind {
    ( $x:expr, $f:expr ) => { bind($x, $f) };
    ( $x:expr, $f:expr, $($rest:expr),+ ) => { bind($x, |x| bind!( $f(x), $($rest),+ )) };
}



 



/////////////////////////////////////////////////////////////////////////////////////////////////////
// A linked list of type constructors with a Head end followed by a trailing Tail.
// HList, similar to linked lists, but instead of having a single element at each node, they have a 
// type constructor that can be used to construct a list of any length.  The type constructor HCons
// takes a head and a tail and returns a new list. The tail is itself an HList, and the head can be
// any type. Recursively, the tail can be an HCons or an HNil. 
// 
// This allows us to construct lists of arbitrary length and type.
pub trait HList { type Item<'a>: HList; }

#[derive(Debug, PartialEq)]
pub struct HCons<H, T: HList> { pub head: H, pub tail: T }
impl<H, T: HList> HList
for HCons<H, T>
where for<'a> <T as HList>::Item<'a>: HList {
    type Item<'a> = HCons<H, <T as HList>::Item<'a>>;
}

#[derive(Debug, PartialEq)]
pub struct HNil;
impl HList 
for HNil { type Item<'a> = Self; }

// Provides a type constructor for HLists.
pub trait HListable<'a> { type Output; }

impl<'a, T> HListable<'a> for &'a T { type Output = T; }
impl<'a, T> HListable<'a> for &'a mut T { type Output = T; }
impl<'a> HListable<'a> for HNil { type Output = HNil; }

impl<'a, H, T> HListable<'a> for HCons<H, T> 
where T: HListable<'a> + HList, <T as HListable<'a>>::Output: HList { 
    type Output = HCons<H, <T as HListable<'a>>::Output>; 
}

// cons
pub trait HListCons<'a, H> { type TypeCtor: HList;
    fn cons(self, head: H) -> <Self::TypeCtor as HListable<'a>>::Output where <Self as HListCons<'a, H>>::TypeCtor: for<'b> HListable<'a>;
}

// uncons
pub trait HListUncons { 
    type Head; 
    type Tail: HList;
    fn uncons(self) -> (Self::Head, Self::Tail);
}

impl<H, T: HList> HListUncons for HCons<H, T> { 
    type Head = H; 
    type Tail = T;
    fn uncons(self) -> (Self::Head, Self::Tail) { (self.head, self.tail) }
}


// HListMapper: Apply a function to each element of an HList.
pub trait Mapper {
    type Output;
    fn map(self) -> Self::Output;
}
pub trait HListMapper {
    type Output: HList;
    fn map(self) -> Self::Output;
}
impl HListMapper for HNil {
    type Output = HNil;
    fn map(self) -> Self::Output {
        HNil
    }
}
impl<H, T> HListMapper for HCons<H, T>
where
    H: Mapper,
    T: HList + HListMapper,
{
    type Output = HCons<H::Output, T::Output>;
    fn map(self) -> Self::Output {
        HCons {
            head: self.head.map(),
            tail: self.tail.map(),
        }
    }
}


// hlist macro // 
#[macro_export]
macro_rules! hlist {
    () => {
        HNil
    };
    ($head:expr) => {
        HCons {
            head: $head,
            tail: HNil,
        }
    };
    ($head:expr, $($tail:expr),+ $(,)?) => {
        HCons {
            head: $head,
            tail: hlist!($($tail),+),
        }
    };
}


pub struct ValidityReport {
    valid_count: usize,
    total_count: usize,
}

impl ValidityReport {
    pub fn new(valid_count: usize, total_count: usize) -> Self {
        Self { valid_count, total_count }
    }

    pub fn combine(&self, other: &Self) -> Self {
        Self {
            valid_count: self.valid_count + other.valid_count,
            total_count: self.total_count + other.total_count,
        }
    }

    pub fn validity_percentage(&self) -> f64 {
        if self.total_count == 0 {
            100.0 // or 0.0 depending on how you want to define validity of an empty list
        } else {
            (self.valid_count as f64 / self.total_count as f64) * 100.0
        }
    }
}



// Validaty measures the correctness of an HList.
pub trait Validaty {
    fn validate_with<F>(&self, validator: &F) -> ValidityReport
    where
        F: Fn(&dyn Any) -> bool;
}

impl Validaty for HNil {
    fn validate_with<F>(&self, _validator: &F) -> ValidityReport
    where
        F: Fn(&dyn Any) -> bool,
    {
        ValidityReport::new(0, 0)
    }
}

impl<H, T> Validaty for HCons<H, T>
where
    H: Any + Validaty,
    T: Validaty + HList,
{
    fn validate_with<F>(&self, validator: &F) -> ValidityReport
    where
        F: Fn(&dyn Any) -> bool,
    {
        let head_valid = validator(&self.head as &dyn Any);
        let tail_report = self.tail.validate_with(validator);
        if head_valid {
            tail_report.combine(&ValidityReport::new(1, 1))
        } else {
            tail_report.combine(&ValidityReport::new(0, 1))
        }
    }
}



// Example of a Validaty implementation for specific types

// i32
impl Validaty for i32 {
    fn validate_with<F>(&self, validator: &F) -> ValidityReport
    where
        F: Fn(&dyn Any) -> bool,
    {
        if validator(self as &dyn Any) {
            ValidityReport::new(1, 1)
        } else {
            ValidityReport::new(0, 1)
        }
    }
}

// String
impl Validaty for String {
    fn validate_with<F>(&self, validator: &F) -> ValidityReport
    where
        F: Fn(&dyn Any) -> bool,
    {
        if validator(self as &dyn Any) {
            ValidityReport::new(1, 1)
        } else {
            ValidityReport::new(0, 1)
        }
    }
}





// HListLength measures the length of an HList.
pub trait HListLength {
    fn length() -> usize;
}

impl HListLength for HNil {
    fn length() -> usize {
        0
    }
}

impl<H, T> HListLength for HCons<H, T> 
where
    T: HListLength + HList,
{
    fn length() -> usize {
        1 + T::length()
    }
}








#[cfg(test)]
mod test_integration {
    use super::*;

    #[test]
    fn test_error() {
        assert_eq!(error::<i32>("error"), Err(OpError::new("error")));
    }

    #[test]
    fn test_bind() {
        let x: Result<i32, OpError> = Ok(1);
        let y: Result<i32, OpError> = Ok(2);

        assert_eq!(bind!(x, |x| bind!(y, |y| Ok(x + y))), Ok(3));
        assert_eq!(bind!(error::<i32>("error"), |x| Ok(x + 1)), error("error"));

        assert_eq!(bind!(Ok(1), |x| Ok(x + 1), |x| Ok(x + 1)), Ok(3));
        assert_eq!(bind!(Ok(1), |x| Ok(x + 1), |x| error::<i32>("error")), error("error"));
        assert_eq!(bind!(Ok(1), |x| error::<i32>("error"), |x| Ok(x + 1)), error("error"));
    }  

    #[test]
    fn test_basics() {
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

        // anon types with operators
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
    fn dozen_deep() {
        let hlist = hlist!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9); 
        assert_eq!(hlist.tail.head, 1);
        assert_eq!(hlist.tail.tail.head, 2);
        assert_eq!(hlist.tail.tail.tail.head, 3);
        assert_eq!(hlist.tail.tail.tail.tail.head, 4);
        assert_eq!(hlist.tail.tail.tail.tail.tail.head, 5);
        assert_eq!(hlist.tail.tail.tail.tail.tail.tail.head, 6);
        assert_eq!(hlist.tail.tail.tail.tail.tail.tail.tail.head, 7);
        assert_eq!(hlist.tail.tail.tail.tail.tail.tail.tail.tail.head, 8);
        assert_eq!(hlist.tail.tail.tail.tail.tail.tail.tail.tail.tail.head, 9);
    }
    
    #[test]
    fn test_nestive() {
        let hlist: HCons<u32, HCons<u32, HNil>> = HCons { head: 0, tail: HCons { head: 1, tail: HNil } };
        let hlist: HCons<u32, HCons<u32, HCons<u32, HNil>>> = HCons { head: 2, tail: hlist };
        let hlist: HCons<u32, HCons<u32, HCons<u32, HCons<u32, HNil>>>> = HCons { head: 3, tail: hlist };
        assert_eq!(hlist.head, 3);
        assert_eq!(hlist.tail.head, 2);
        assert_eq!(hlist.tail.tail.head, 0);
        assert_eq!(hlist.tail.tail.tail.head, 1);
    }

    // Uncons and cons additional testing
    #[test]
    fn test_uncons() {
        let hlist = hlist!(0, 1, 2);
        let (head, tail) = hlist.uncons();
        assert_eq!(head, 0);
        assert_eq!(tail.head, 1);
        assert_eq!(tail.tail.head, 2);
    }
    


    #[test]
    fn test_map() {

        impl Mapper for i32 {
            type Output = i32;
            fn map(self) -> Self::Output {
                self + 1
            }
        }
        
        impl Mapper for f64 {
            type Output = f64;
            fn map(self) -> Self::Output {
                self * 2.0
            }
        }

        let hlist: HCons<i32, HCons<f64, HNil>> = hlist!(0, 1.0);
        let hlist_mapped = hlist.map();
        assert_eq!(hlist_mapped.head, 1);
        assert_eq!(hlist_mapped.tail.head, 2.0);
    }

    #[test]
    fn test_validity() {
        let hlist = hlist!(42, -1, "Hello".to_string());  // TODO: add more types e.g: &str, bool, etc.
        let validity_report = hlist.validate_with(&|x: &dyn Any| x.downcast_ref::<i32>().map_or(false, |&num| num >= 0));
        let validity_percentage = validity_report.validity_percentage();
        assert_eq!(validity_percentage, 50.0);

        


    }

    use std::any::Any;

    #[test]
    fn test_dynamic_validity() {
        let hlist = hlist!(42, -1, "Hello".to_string());

        // Define a custom validation function for each type
        let int_validator = |x: &dyn Any| x.downcast_ref::<i32>().map_or(false, |&num| num >= 0);
        let string_validator = |x: &dyn Any| x.downcast_ref::<String>().map_or(false, |str| !str.is_empty());
        
        // Use the custom validation function
        let validity_report_int = hlist.validate_with(&int_validator);
        let validity_percentage_int = validity_report_int.validity_percentage();
        println!("Integer Validity: {}%", validity_percentage_int);
        
        let validity_report_string = hlist.validate_with(&string_validator);
        let validity_percentage_string = validity_report_string.validity_percentage();
        println!("String Validity: {}%", validity_percentage_string);
    }

}














