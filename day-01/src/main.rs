use std::io;

use itertools::Itertools;
use std::io::prelude::*;
fn main() {
    // Read stdin by lines
    let stdin = io::stdin();
    let lock = stdin.lock();
    println!(
        "{:?}",
        lock.lines()
            .batching(|lines| {
                // Cluster by blank lines
                let mut values = lines
                    .take_while(|r| r.is_ok() && !r.as_ref().unwrap().trim().is_empty())
                    .filter_map(|v| v.ok())
                    .peekable();
                if values.peek().is_none() {
                    None
                } else {
                    Some(
                        values
                            .map(|v| i64::from_str_radix(&v, 10).unwrap()) // Map numbers,
                            .sum::<i64>(), // Aggregate sum
                    )
                }
            })
            // Part 1 can just use .max() here
            // Part 2 is also pretty simple.
            // Put them all in order (n log n)
            // Taking the top k from a collection of n is optimal in n log k
            // Honestly, I'm not too worried about the lack of optimization here. I've seen this one before.
            .sorted()
            // Go from the back, because it's a DoubleEndedIterator and sorted in ascending order
            .rev()
            // Take the top 3
            .take(3)
            // Sum.
            .sum::<i64>()
    )
}
