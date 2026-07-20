use std::hint::black_box;

use strustegy::prelude::*;

fn main() {
    let values = hlist![
        1_u32, 2_u32, 3_u32, 4_u32, 5_u32, 6_u32, 7_u32, 8_u32, 9_u32, 10_u32, 11_u32, 12_u32,
        13_u32, 14_u32, 15_u32, 16_u32, 17_u32, 18_u32, 19_u32, 20_u32, 21_u32, 22_u32, 23_u32,
        24_u32, 25_u32, 26_u32, 27_u32, 28_u32, 29_u32, 30_u32, 31_u32, 32_u32
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
        .then(Identity)
        .then(Identity);
    let _ = black_box(values.hmap(&pipeline));
}
