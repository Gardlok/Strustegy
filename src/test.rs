




// Compare target with partialeq, eq, or cmp
//
pub fn compare<T: PartialEq>(target: &T, value: &T) -> bool {
    target == value
}
pub fn compare_eq<T: Eq>(target: &T, value: &T) -> bool {
    target == value
}
pub fn compare_cmp<T: PartialOrd>(target: &T, value: &T) -> bool {
    target == value
}


