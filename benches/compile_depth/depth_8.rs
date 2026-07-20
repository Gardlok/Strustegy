use std::hint::black_box;

use strustegy::prelude::*;

fn main() {
    let values = hlist![1_u32, 2_u32, 3_u32, 4_u32, 5_u32, 6_u32, 7_u32, 8_u32];
    let pipeline = Identity
        .then(Identity)
        .then(Identity)
        .then(Identity)
        .then(Identity)
        .then(Identity)
        .then(Identity)
        .then(Identity)
        .then(Identity);
    let _ = black_box(values.hmap(&pipeline));
}
