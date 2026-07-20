use std::hint::black_box;

use strustegy::prelude::*;

fn main() {
    let values = hlist![
        1_u32, 2_u32, 3_u32, 4_u32, 5_u32, 6_u32, 7_u32, 8_u32, 9_u32, 10_u32, 11_u32, 12_u32,
        13_u32, 14_u32, 15_u32, 16_u32
    ];
    let pipeline = Identity
        .then(Identity)
        .then(Identity)
        .then(Identity)
        .then(Identity)
        .then(Identity)
        .then(Identity)
        .then(Identity)
        .then(Identity)
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
