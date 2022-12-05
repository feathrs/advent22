#![feature(iter_array_chunks)]
// Nightly only, because of part 2.
// For stable, use chunks from Itertools.

use std::collections::HashSet;
use std::io::{self, prelude::*};

fn remap_byte(input: u8) -> u8 {
    match input {
        65..=90 => input - 65 + 27, // Constant folding means I don't have to do the math myself.
        97..=122 => input - 96,
        _ => panic!("Bad byte (not [a-zA-Z])"),
    }
}

fn main() {
    let stdin = io::stdin().lock(); // As usual, read from stdin.
    println!(
        "{}",
        stdin
            .lines()
            .map(Result::unwrap)
            .map(|v| v.bytes().collect::<HashSet<_>>())
            // Part 1:
            // .map(|v| {
            //     let (left, right) = v.split_at(v.len() / 2); // The input is ascii with a known mid-point, so this is safe.
            //     left.bytes()
            //         .collect::<HashSet<_>>()
            //         .intersection(&right.bytes().collect())
            //         .map(|&v| remap_byte(v) as i64) // Need to cast for the later sum
            //         .next()
            //         .expect("Must have at least one common element")
            // })
            .array_chunks::<3>()
            .map(|v| *v
                .into_iter()
                .reduce(|left, right| left.intersection(&right).cloned().collect())
                .unwrap()
                .iter()
                .next()
                .unwrap())
            .map(|v| remap_byte(v) as i64)
            .sum::<i64>()
    )
}
