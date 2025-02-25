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
}

impl Stacks {
    /// Apply a single instruction to the set of stacks in `self`.
    /// Return the new set of stacks, or a `CraneError` if the instruction
    /// is invalid.
    fn apply_instruction(mut self, instruction: &CraneInstruction) -> Result<Self, CraneError> {
        todo!()
    }

    /// Perform each of these instructions in order on the set of stacks
    /// in `self`. Return the new set of stacks, or a `CraneError` if
    /// any of the instructions are invalid.
    fn apply_instructions(self, instructions: &CraneInstructions) -> Result<Self, CraneError> {
        todo!()
    }

    /// Return a string containing the top character of each stack in order.
    /// The stacks should all be non-empty; if any is empty return a `CraneError`.
    fn tops_string(&self) -> Result<String, CraneError> {
        todo!()
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
        todo!()
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

// Implementing `PartialEq<Vec<char>> for Stack` here allows us to
// say things like `vec!['A', 'B', 'C'] == stack`. This is useful
// for testing, where we might want to compare a `Stack` to a `Vec<char>`
// using something like ``assert_eq!(stack, vec!['A', 'B', 'C'])`.
impl PartialEq<Vec<char>> for Stack {
    fn eq(&self, other: &Vec<char>) -> bool {
        todo!()
    }
}

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
        todo!()
    }
}

struct CraneInstructions {
    instructions: Vec<CraneInstruction>,
}

impl FromStr for CraneInstructions {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

// Don't consider these tests complete or comprehensive. They're just a starting point,
// and you should add more tests to make sure your code works as expected. They all
// start out with the `#[ignore]` attribute, so you'll need to remove that to run them
// as you implement them.
#[cfg(test)]
mod tests {
    use super::*;

    // Test that we can parse stacks correctly.
    #[test]
    #[ignore = "We haven't implemented stack parsing yet"]
    fn test_from_str() {
        // The `\` at the end of the line escapes the newline and all following whitespace.
        let input = "1 Z N\n\
                           2 M C D\n\
                           3 P";
        println!("{input}");
        #[allow(clippy::unwrap_used)]
        let stacks: Stacks = input.parse().unwrap();
        assert_eq!(2, stacks.stacks[0].len());
        // The implementation of `PartialEq<Vec<char>>` above is what allows
        // us to compare a `Stack` to a `Vec<char>` here and in other tests.
        assert_eq!(stacks.stacks[0], vec!['Z', 'N']);
        assert_eq!(3, stacks.stacks[1].len());
        assert_eq!(stacks.stacks[1], vec!['M', 'C', 'D']);
        assert_eq!(1, stacks.stacks[2].len());
        assert_eq!(stacks.stacks[2], vec!['P']);
    }

    // Test that we can parse instructions correctly.
    #[test]
    #[ignore = "We haven't implemented instruction parsing yet"]
    fn test_instruction_parsing() {
        let input = "move 1 from 2 to 1\nmove 3 from 1 to 3";
        let instructions: CraneInstructions = input.parse().unwrap();
        assert_eq!(2, instructions.instructions.len());
        assert_eq!(1, instructions.instructions[0].num_to_move);
        assert_eq!(1, instructions.instructions[0].to_stack);
        assert_eq!(2, instructions.instructions[0].from_stack);
        assert_eq!(3, instructions.instructions[1].num_to_move);
        assert_eq!(3, instructions.instructions[1].to_stack);
        assert_eq!(1, instructions.instructions[1].from_stack);
    }

    // You probably want some tests that check that `apply_instruction` works as expected.
    // You might want to test that it moves the right number of items, that it moves them
    // from the right stack, that it moves them to the right stack, and that it doesn't
    // move items from an empty stack. Below is a simple test that checks that the
    // instruction `move 2 from 0 to 1` moves two items from stack 0 to stack 1, but you
    // probably want more than that.

    // Test that the instruction `move 2 from 0 to 1` works as expected with non-empty
    // stacks.
    #[test]
    #[ignore = "We haven't implemented the `apply_instruction` method yet"]
    fn test_apply_instruction() {
        let stacks = Stacks {
            stacks: [
                Stack {
                    stack: vec!['A', 'B', 'C'],
                },
                Stack {
                    stack: vec!['D', 'E', 'F'],
                },
                Stack {
                    stack: vec!['G', 'H', 'I'],
                },
                Stack { stack: Vec::new() },
                Stack { stack: Vec::new() },
                Stack { stack: Vec::new() },
                Stack { stack: Vec::new() },
                Stack { stack: Vec::new() },
                Stack { stack: Vec::new() },
            ],
        };

        let instruction = CraneInstruction {
            num_to_move: 2,
            from_stack: 0,
            to_stack: 1,
        };

        let new_stacks = stacks
            .apply_instruction(&instruction)
            .expect("Failed to apply instruction");

        assert_eq!(new_stacks.stacks[0], vec!['A']);
        assert_eq!(new_stacks.stacks[1], vec!['D', 'E', 'F', 'C', 'B']);
    }

    // This essentially runs `main()` and checks that the results are correct for part 1.
    #[test]
    #[ignore = "We haven't implemented the `apply_instructions` method yet"]
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

        assert_eq!("SBPQRSCDF", stack_tops);
    }
}
