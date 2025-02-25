// FEEL FREE TO IGNORE (OR EVEN DELETE) THIS FILE.
// This is just the code I used to rotate the original, more complex
// stack representation into the simpler one we're using now. I'm
// keeping it around in case I need to refer back to it later.

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::{Context, Result};
use std::{
    fmt::{self, Display},
    fs,
    str::FromStr,
};

static INPUT_FILE: &str = "original_input.txt";

const NUM_STACKS: usize = 9;

#[derive(Default, Debug)]
struct Stacks {
    stacks: [Vec<char>; NUM_STACKS],
}

// An alternative approach would be to just do `line.chars()` and
// `line.nth(4)` for each stack in the mapping. That would avoid
// creating the `Vec<char>` for `line` that we have at the moment.
// We'd probably have to special case the first stack, though, and
// that would be ugly, so I'm not sure that wins.
fn extract_stack_elements(line: &str) -> Vec<char> {
    let line = line.chars().collect::<Vec<_>>();
    (0..NUM_STACKS).map(|pos| line[1 + 4 * pos]).collect()
}

impl Stacks {
    // Note that the argument here is `self` and not `&self` because we
    // need to take ownership of this `Stacks` value so we can mutate
    // it in the `fold()` call. Alternatively we could declare this
    // as taking `&mut self`, but the calling function has no need
    // access the "old" value so there's really no need.
    fn push_values_on_stacks(self, line: &[char]) -> Self {
        line.iter()
            .enumerate()
            .filter(|&(_, c)| *c != ' ')
            .fold(self, |mut stacks, (i, c)| {
                stacks.stacks[i].push(*c);
                stacks
            })
    }
}

impl FromStr for Stacks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let stacks = s
            .lines()
            // We reverse the lines because we want the "bottom" lines
            // to be pushed onto the stacks first so those values end
            // up on the bottom of the stacks.
            .rev()
            // We'll just skip the line with the stack numbers since we
            // never use them.
            .skip(1)
            // Convert each line to a `Vec<char>` that holds the 9 elements
            // at each level. We'll put spaces in that `Vec<char>` for stacks
            // that don't have anything at that level.
            .map(extract_stack_elements)
            // "Loop" over each line/level, pushing the non-space values onto
            // the appropriate stacks.
            .fold(Self::default(), |stacks, line| {
                stacks.push_values_on_stacks(&line)
            });

        Ok(stacks)
    }
}

impl Display for Stacks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, stack) in self.stacks.iter().enumerate() {
            write!(f, "{}", i + 1)?;
            for c in stack {
                write!(f, " {c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let (stack_config, _) = contents
        .split_once("\n\n")
        .context("There was no blank line in the input")?;

    let stacks: Stacks = stack_config.parse()?;

    println!("{stacks}");

    Ok(())
}
