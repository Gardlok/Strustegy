use core::mem::size_of;

use strustegy::prelude::*;

#[test]
fn macros_and_cons_construct_the_same_shape() {
    let using_cons = HNil.cons("third").cons(2.5_f64).cons(1_i32);
    let using_macro = hlist![1_i32, 2.5_f64, "third"];

    assert_eq!(using_cons, using_macro);
}

#[test]
fn length_and_type_shape_are_static() {
    type Values = hlist_ty![i32, String, bool];

    let values: Values = hlist![42_i32, String::from("rose"), true];

    assert_eq!(<Values as HList>::LEN, 3);
    assert_eq!(values.len(), 3);
    assert!(!values.is_empty());
    assert!(HNil.is_empty());
}

#[test]
fn shared_and_mutable_views_preserve_identity() {
    let mut values = hlist![10_i32, String::from("rose")];

    {
        let borrowed = values.refs();
        assert!(core::ptr::eq(borrowed.head, &values.head));
        assert!(core::ptr::eq(borrowed.tail.head, &values.tail.head));
    }

    {
        let borrowed = values.muts();
        *borrowed.head += 5;
        borrowed.tail.head.push('!');
    }

    assert_eq!(values, hlist![15_i32, String::from("rose!")]);
}

#[test]
fn nonempty_lists_can_be_decomposed_in_each_ownership_mode() {
    let mut values = hlist![10_i32, String::from("rose")];

    let (head, tail) = values.parts();
    assert_eq!(*head, 10);
    assert_eq!(tail.head, "rose");

    let (head, tail) = values.parts_mut();
    *head = 20;
    tail.head.push('!');

    let (head, tail) = values.into_parts();
    assert_eq!(head, 20);
    assert_eq!(tail, hlist![String::from("rose!")]);
}

#[test]
fn compile_time_indices_read_and_mutate_expected_types() {
    type First = Here;
    type Second = There<Here>;
    type Third = There<There<Here>>;

    let mut values = hlist![10_i32, "rose", 2.5_f64];

    assert_eq!(*values.get_at::<First>(), 10);
    assert_eq!(*values.get_at::<Second>(), "rose");
    assert_eq!(*values.get_at::<Third>(), 2.5);

    *values.get_at_mut::<First>() = 20;
    *values.get_at_mut::<Third>() = 5.0;

    assert_eq!(values, hlist![20_i32, "rose", 5.0_f64]);
}

#[test]
fn structural_markers_have_no_runtime_size() {
    assert_eq!(size_of::<HNil>(), 0);
    assert_eq!(size_of::<Here>(), 0);
    assert_eq!(size_of::<There<Here>>(), 0);
}
