// YOU SHOULD JUST COPY OVER YOUR SOLUTION
// FROM PART 1 AND USE THAT AS THE START OF PART 2.

use std::{fs, str::FromStr};

static INPUT_FILE: &str = "input.txt";

fn main() {
    let contents =
        fs::read_to_string(INPUT_FILE).expect(&format!("Failed to open file '{INPUT_FILE}'"));

    // This splits the input into two parts, the text before the blank
    // line (`stack_config`) and the part after the blank line (`instructions`).
    let (stack_config, instructions) = contents
        .split_once("\n\n")
        .expect("There was no blank line in the input");

    // The `.parse()` call actually calls the appropriate `from_str()`, which
    // in this case is in the `impl FromStr for Stacks` block.
    let stacks: Stacks = stack_config
        .parse()
        .expect("Failed to parse stack configuration");

    // This `.parse()` call uses the implementation of `from_str()`
    // in the `impl FromStr for CraneInstructions` block.
    let instructions: CraneInstructions = instructions
        .parse()
        .expect("Failed to parse crane instructions");

    // Run all the instructions, returning the final `Stacks` state.
    let final_state = stacks
        .apply_instructions(&instructions)
        .expect("Applying an instruction set failed");

    // Get the top of the stacks and print that out.
    println!(
        "The top of the stacks is {}",
        final_state
            .tops_string()
            .expect("Tried to take the top of an empty stack")
    );
}

#[derive(Debug)]
pub enum ParseError {
    // Add different variants as you discover different kinds of parsing errors.
    // This could include things like too many stacks, illegal strings on a stack, etc.
    InvalidInstruction,
}

const NUM_STACKS: usize = 9;

#[derive(Debug, Default)]
pub struct Stacks {
    stacks: [Stack; NUM_STACKS],
}

#[derive(Debug)]
enum CraneError {
    // Add different variants as you discover different kinds of errors
    // that can occur when applying a crane instruction.
    // This could include things like trying to move from an empty stack,
    // trying to get the top of an empty stack, etc.
    InvalidStack,
    EmptyStack,
    InvalidMove(Stacks, CraneInstruction),
}

impl Stacks {
    /// Apply a single instruction to the set of stacks in `self`.
    /// Return the new set of stacks, or a `CraneError` if the instruction
    /// is invalid.
    fn apply_instruction(mut self, instruction: &CraneInstruction) -> Result<Self, CraneError> {
        // Throw error if the stack moving from or moving to does not exist
        if instruction.from_stack >= NUM_STACKS || instruction.to_stack >= NUM_STACKS {
            return Err(CraneError::InvalidStack);
        }

        if self.stacks[instruction.from_stack].len() < instruction.num_to_move {
            return Err(CraneError::InvalidMove(self, *instruction));
        }

        // collects the crates to move as a vector of characters.
        let crates_to_move: Vec<char> = self.stacks[instruction.from_stack]
            .stack
            .drain(self.stacks[instruction.from_stack].stack.len() - instruction.num_to_move..)
            .collect();
        self.stacks[instruction.to_stack]
            .stack
            //.extend(crates_to_move);
            .extend(crates_to_move.iter().rev());
        Ok(self)
    }

    /// Perform each of these instructions in order on the set of stacks
    /// in `self`. Return the new set of stacks, or a `CraneError` if
    /// any of the instructions are invalid.
    fn apply_instructions(self, instructions: &CraneInstructions) -> Result<Self, CraneError> {
        let mut new_stacks = self;

        // for each instruction in the instructions, apply the instruction to the stacks
        for instruction in &instructions.instructions {
            new_stacks = new_stacks.apply_instruction(instruction)?;
        }
        Ok(new_stacks)
    }

    /// Return a string containing the top character of each stack in order.
    /// The stacks should all be non-empty; if any is empty return a `CraneError`.
    fn tops_string(&self) -> Result<String, CraneError> {
        let mut tops = String::new();

        // for each stack in the stacks, get the top character and add it to the tops string
        for stack in &self.stacks {
            if stack.stack.is_empty() {
                return Err(CraneError::EmptyStack);
            }
            tops.push(stack.stack[stack.stack.len() - 1]);
        }
        Ok(tops)
    }
}

impl FromStr for Stacks {
    type Err = ParseError;

    // You probably want to use `s.lines()` to create an iterator over the lines (one per stack).
    // Then for each line:
    //   (a) extract the number at the front as the stack number
    //   (b) extract the following characters as the stack contents
    // The function `split_ascii_whitespace()` should prove useful.
    // Note that the stack numbers start at 1 and you'll need the indices
    // in `Stacks::stacks` to start at 0.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // create a new Stacks struct
        let mut stacks = Stacks::default();

        // for each line in the input string, parse the stack number and stack contents
        for line in s.lines() {
            let mut parts = line.split_ascii_whitespace(); // split the line into parts
            let stack_num = parts // get the stack number
                .next() // get the first part of the line
                .expect("No stack number found") // if there is no stack number, return an error
                .parse::<usize>() // parse the stack number as a usize
                .expect("Failed to parse stack number")
                - 1; // subtract 1 from the stack number to get the index
            let stack_contents = parts.collect::<String>(); // get the stack contents
            stacks.stacks[stack_num].stack = stack_contents.chars().collect(); // set the stack contents
        }
        Ok(stacks)
    }
}

#[derive(Debug, Default)]
pub struct Stack {
    stack: Vec<char>,
}

impl Stack {
    pub fn len(&self) -> usize {
        self.stack.len()
    }
}

impl FromStr for Stack {
    type Err = ParseError;

    // collects the characters in the input string as a vector of characters
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Stack::default();
        stack.stack = s.chars().collect();
        Ok(stack)
    }
}

// Implementing `PartialEq<Vec<char>> for Stack` here allows us to
// say things like `vec!['A', 'B', 'C'] == stack`. This is useful
// for testing, where we might want to compare a `Stack` to a `Vec<char>`
// using something like ``assert_eq!(stack, vec!['A', 'B', 'C'])`.
impl PartialEq<Vec<char>> for Stack {
    fn eq(&self, other: &Vec<char>) -> bool {
        self.stack == *other
    }
}

#[derive(Debug, Clone, Copy)]
struct CraneInstruction {
    num_to_move: usize,
    from_stack: usize,
    to_stack: usize,
}

impl FromStr for CraneInstruction {
    type Err = ParseError;

    // The instruction specification lines have the form
    //     move 13 from 8 to 7
    // All we need to capture are the three numbers, which happen to
    // be in the odd positions in the input line. I used a `filter` statement
    // to extract those three items from the list, which I could
    // then parse into `usize` using a `map` statement. You could also just
    // "reach" into the split string directly if you find that easier.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<usize> = s
            .split_ascii_whitespace()
            .filter_map(|part| part.parse().ok())
            .collect();

        if parts.len() != 3 {
            return Err(ParseError::InvalidInstruction);
        }

        Ok(CraneInstruction {
            num_to_move: parts[0],
            from_stack: parts[1] - 1,
            to_stack: parts[2] - 1,
        })
    }
}

struct CraneInstructions {
    instructions: Vec<CraneInstruction>,
}

impl FromStr for CraneInstructions {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<CraneInstruction>, ParseError>>()?;
        Ok(CraneInstructions { instructions })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // This essentially runs `main()` and checks that the results are correct for part 2.

    #[test]
    // #[ignore = "We haven't implemented the `apply_instructions` method yet"]
    fn test_part_1() {
        let contents =
            fs::read_to_string(INPUT_FILE).expect(&format!("Failed to open file '{INPUT_FILE}'"));

        let (stack_config, instructions) = contents
            .split_once("\n\n")
            .expect("There was no blank line in the input");

        let stacks: Stacks = stack_config
            .parse()
            .expect("Failed to parse stack configuration");

        let instructions: CraneInstructions = instructions
            .parse()
            .expect("Failed to parse crane instructions");

        let final_state = stacks
            .apply_instructions(&instructions)
            .expect("Applying an instruction set failed");

        let stack_tops = final_state
            .tops_string()
            .expect("Tried to take the top of an empty stack");

        assert_eq!("RGLVRCQSB", stack_tops); // This is different from part 1 bec ability to grab multiple crates at once
    }
}
