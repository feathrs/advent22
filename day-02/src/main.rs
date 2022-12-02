use std::io::{self, prelude::*};

// Part 1
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn code_to_choice(code: &str) -> Choice {
    match code {
        "X" => Choice::Rock,
        "Y" => Choice::Paper,
        "Z" => Choice::Scissors,
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissors,
        _ => panic!("Bad choice"),
    }
}

fn pair_to_points((left, right): (Choice, Choice)) -> i64 {
    // I know what you're thinking: What?
    // I need to know how many times I need to rotate my input forward to get the opponent's.
    // None? Draw.
    // Once? Win.
    // Twice? Loss.
    // Start by taking the difference between the right and left; this maps it into a range of -2 (1-3) ~ 2 (3-1)
    // Offset this so that the +1 case (2-1, 3-2, 1-3) is at 2 mod 3 (2, 5)
    // This also means that the +0 case (2-2, 3-3, 1-1) is at 1 mod 3 (4)
    // And the -1/+2 case (1-2, 2-3, 3-1) is at 0 mod 3 (3, 6)
    let rpoints = right as i64;
    rpoints + 3 * ((rpoints - (left as i64) + 4) % 3)
}

// Part 2
enum Decision {
    Lose = -1,
    Draw = 0,
    Win = 1,
}

fn code_to_decision(code: &str) -> Decision {
    match code {
        "X" => Decision::Lose,
        "Y" => Decision::Draw,
        "Z" => Decision::Win,
        _ => panic!("Bad decision"),
    }
}

fn decision_pair_to_points((left, decision): (Choice, Decision)) -> i64 {
    // We can do this one with even more raw math.
    // In fact, because we don't have to calculate if we won or lost, it's even simpler*
    (decision as i64 + 1) * 3 // Take the value of our win/loss directly
        + (left as i64 + decision as i64 + 2) % 3 + 1 // Offset the value of our play, bring the range to [2~6] to avoid negatives, wrap and re-offset
}

fn main() {
    let stdin = io::stdin().lock();
    println!(
        "{}",
        stdin
            .lines() // Take by lines
            .map(|line| {
                let tmp_line = line.unwrap(); // Lifetimes.
                let (left, right) = tmp_line.split_once(' ').unwrap(); // Split the input

                // Part 1: (code_to_choice(left), code_to_choice(right)) // Convert the inputs into sane data
                (code_to_choice(left), code_to_decision(right))
            })
            // Part 1: .map(pair_to_points)
            .map(decision_pair_to_points)
            .sum::<i64>()
    )
}
