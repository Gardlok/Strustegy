use strustegy::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Convert;

impl Strategy<i32> for Convert {
    type Output = i64;

    fn apply(&self, value: i32) -> Self::Output {
        i64::from(value)
    }
}

impl Strategy<String> for Convert {
    type Output = usize;

    fn apply(&self, value: String) -> Self::Output {
        value.len()
    }
}

#[derive(Debug, Clone, Copy)]
struct Finish;

impl Strategy<i64> for Finish {
    type Output = String;

    fn apply(&self, value: i64) -> Self::Output {
        format!("value:{value}")
    }
}

impl Strategy<usize> for Finish {
    type Output = bool;

    fn apply(&self, value: usize) -> Self::Output {
        value >= 4
    }
}

#[derive(Debug, Clone, Copy)]
struct Describe;

impl Strategy<String> for Describe {
    type Output = usize;

    fn apply(&self, value: String) -> Self::Output {
        value.len()
    }
}

impl Strategy<bool> for Describe {
    type Output = u8;

    fn apply(&self, value: bool) -> Self::Output {
        u8::from(value)
    }
}

#[derive(Debug, Clone, Copy)]
struct InspectBorrowed;

impl Strategy<&i32> for InspectBorrowed {
    type Output = i64;

    fn apply(&self, value: &i32) -> Self::Output {
        i64::from(*value)
    }
}

impl Strategy<&String> for InspectBorrowed {
    type Output = usize;

    fn apply(&self, value: &String) -> Self::Output {
        value.len()
    }
}

#[derive(Debug, Clone, Copy)]
struct MutateBorrowed;

impl Strategy<&mut i32> for MutateBorrowed {
    type Output = ();

    fn apply(&self, value: &mut i32) -> Self::Output {
        *value *= 2;
    }
}

impl Strategy<&mut String> for MutateBorrowed {
    type Output = ();

    fn apply(&self, value: &mut String) -> Self::Output {
        value.push('!');
    }
}

#[test]
fn heterogeneous_mapping_computes_its_output_shape() {
    let values = hlist![10_i32, String::from("rose")];

    let mapped: hlist_ty![i64, usize] = values.hmap(&Convert);

    assert_eq!(mapped, hlist![10_i64, 4_usize]);
}

#[test]
fn identity_mapping_preserves_a_representative_list() {
    let values = hlist![10_i32, String::from("rose"), true];

    assert_eq!(values.clone().hmap(&Identity), values);
}

#[test]
fn fluent_composition_matches_sequential_mapping() {
    let values = hlist![10_i32, String::from("rose")];
    let pipeline = Convert.then(Finish);

    let composed = values.clone().hmap(&pipeline);
    let sequential = values.hmap(&Convert).hmap(&Finish);

    assert_eq!(composed, sequential);
    assert_eq!(composed, hlist![String::from("value:10"), true]);
}

#[test]
fn deeper_composition_remains_type_directed() {
    let values = hlist![10_i32, String::from("rose")];
    let pipeline = Convert.then(Finish).then(Describe);

    assert_eq!(values.hmap(&pipeline), hlist![8_usize, 1_u8]);
}

#[test]
fn borrowed_mapping_does_not_consume_or_copy_owned_values() {
    let values = hlist![10_i32, String::from("rose")];

    let mapped = values.hmap_ref(&InspectBorrowed);

    assert_eq!(mapped, hlist![10_i64, 4_usize]);
    assert_eq!(values, hlist![10_i32, String::from("rose")]);
}

#[test]
fn mutable_mapping_updates_the_original_list() {
    let mut values = hlist![10_i32, String::from("rose")];

    let results = values.hmap_mut(&MutateBorrowed);

    assert_eq!(results, hlist![(), ()]);
    assert_eq!(values, hlist![20_i32, String::from("rose!")]);
}
