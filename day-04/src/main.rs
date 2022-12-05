use std::io::{self, prelude::*};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin().lock(); // We know how this one goes by now
    println!(
        "{}",
        stdin
            .lines()
            .map(Result::unwrap)
            .map(|line| {
                let (left, right) = line.split_once(',').unwrap();
                let (l1, l2) = left.split_once('-').unwrap();
                let (r1, r2) = right.split_once('-').unwrap();
                // There were ways to do that homogenously inc. regex but it's not worth it
                match &[l1, l2, r1, r2]
                    .into_iter()
                    .map(<i64 as FromStr>::from_str)
                    .map(Result::unwrap)
                    .collect::<Vec<_>>()[..]
                {
                    // You are like a baby.
                    // Watch this.
                    &[l1, l2, r1, r2] => (l1, l2, r1, r2),
                    _ => panic!("???"),
                }
            })
            .filter(|(l1, l2, r1, r2)| {
                // Ever done intersections in n-space before? This is 1D and very easy.
                // Part 1: (l1 <= r1) && (l2 >= r2) || (r1 <= l1) && (r2 >= l2)
                // If not for the boundaries and symmetries, it could use XNOR instead of AND/OR/AND
                // Fortunately, this second part is... Simpler.
                !((l1 > r2) || (l2 < r1))
            })
            .count()
    )
}
