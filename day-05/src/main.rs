// This time, we have a more complex parsing problem.
// Our parser can still operate line-by-line, but it will have two phases
// Phase 1, the drawing: Groups of 3 chars, separated by a single space character /((\[[A-Z]\]| {3}) ?)*/
// Phase 2, the instructions: /move (\d+) from (\d+) to (\d+)/
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::sequence::{preceded, tuple};
use std::io::{self, prelude::*};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin().lock();
    let mut lines = stdin.lines().map(Result::unwrap);
    // First, we'll get the drawing;
    let mut drawing = lines
        .by_ref()
        .take_while(|v| !v.trim().is_empty()) // Take lines until the separating blank line
        .map(|line| {
            // We can cheat at this one a little, because the input is padded on the right.
            line.chars()
                .batching(|chars| {
                    let res = chars.dropping(1).next();
                    chars.dropping(2);
                    res.map(|v| if v == ' ' { None } else { Some(v) }) // Yes, I'm wrapping Option in Option.
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let cols = drawing[0].len();
    // Rotate the containers around, indexing col-row instead of row-col. Drop the last row, which just labels the cols.
    let mut containers = drawing.drain(..drawing.len() - 1).rfold(
        (0..cols).map(|_| Vec::new()).collect::<Vec<_>>(),
        |mut stacks, row| {
            for i in 0..row.len() {
                if let Some(v) = row[i] {
                    stacks[i].push(v);
                }
            }
            stacks
        },
    );

    // Now, start reading the instructions
    for i in lines {
        let (_, (count, from, to)) = tuple::<&str, _, (&str, nom::error::ErrorKind), _>((
            preceded(tag("move "), map_res(digit1, u64::from_str)),
            preceded(tag(" from "), map_res(digit1, usize::from_str)),
            preceded(tag(" to "), map_res(digit1, usize::from_str)),
        ))(&i)
        .map_err(|_| ())
        .unwrap();
        // Part 1: It picks them up one at a time
        // for _ in 0..count {
        //     if let Some(tmp) = containers[from - 1].pop() {
        //         containers[to - 1].push(tmp);
        //     }
        // }

        // Part 2: It picks them up all together.
        let source = &mut containers[from - 1];
        let tmp = source.split_off(source.len() - count as usize);
        containers[to - 1].extend(tmp);
    }

    // And then produce our output
    println!(
        "{}",
        containers
            .iter()
            .filter_map(|v| v.last())
            .collect::<String>()
    )
}
