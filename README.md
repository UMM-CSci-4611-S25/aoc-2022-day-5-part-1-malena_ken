[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/LQwv5Hzc)
# Advent of Code 2022, Day 5, Parts 1 and 2 <!-- omit from toc -->

- [Problem statement](#problem-statement)
  - [Part 1](#part-1)
  - [Part 2](#part-2)
- [Sketch of key data structures and trait implementations](#sketch-of-key-data-structures-and-trait-implementations)
  - [Add error variants as you go](#add-error-variants-as-you-go)
  - [Parsing stacks and instructions](#parsing-stacks-and-instructions)
  - [Implementing `PartialEq<Vec<char>> for Stack`](#implementing-partialeqvecchar-for-stack)

---

Here we'll do another simple Rust exercise based on an Advent of Code problem,
this time from 2022, [Day 5: Supply Stacks](https://adventofcode.com/2022/day/5).

> [!IMPORTANT]
> I've simplified the inputs on this problem so the parsing will be a lot easier.
> Make sure you base your parsing on the input structure in the provided `input.txt`
> file and as described in the problem descriptions below.

We'll sketch out the primary data structures and traits in class together, and
then you'll work in small groups sort to finish up both Parts 1 and 2. Below are the
problem statements, followed by a sketch of the key data structures, etc.

The project is organized so that there are two binaries in the `bin` directory:

- `part1.rs`
- `part2.rs`

Both have a set of commented out unit tests that you should uncomment and get to pass.

You should be able to run a given part with something like

```bash
cargo run --bin part1
```

replacing `part1` with `part2` as appropriate.

You should be able to run all the tests with `cargo test`.

## Problem statement

The following problem statements are taken directly from
[the Advent of Code problem statements](https://adventofcode.com/2022/day/5).
Typically part 2 of Advent of Code problems is not visible until you've completed part 1,
so I'm cheating slightly by sharing the problem statement of part 2 with everyone.

> [!IMPORTANT]
> I've modified the input examples to use the simplified input format in this version
> of the problem statements.

### Part 1

The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.

The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.

The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.

They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:

```text
1 Z N
2 M C D
3 P

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
```

In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.

Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:

```text
1 Z N D
2 M C
3 P
```

In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:

```text
1
2 M C
3 P D N Z
```

Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:

```text
1 C M
2
3 P D N Z
```

Finally, one crate is moved from stack 1 to stack 2:

```text
1 C
2 M
3 P D N Z
```

The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.

After the rearrangement procedure completes, what crate ends up on top of each stack?

### Part 2

As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.

Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.

The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.

Again considering the example above, the crates begin in the same configuration:

```text
1 Z N
2 M C D
3 P
```

Moving a single crate from stack 2 to stack 1 behaves the same as before:

```text
1 Z N D
2 M C
3 P
```

However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this new configuration:

```text
1
2 M C
3 P Z N D
```

Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:

```text
1 M C
2
3 P Z N D
```

Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets moved:

```text
1 M
2 C
3 P Z N D
```

In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.

Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies. After the rearrangement procedure completes, what crate ends up on top of each stack?

---

## Sketch of key data structures and trait implementations

In `part1.rs` we have 

- The key data structures (`Stacks`, `Stack`, `CraneInstructions`, and `CraneInstruction`)
- Stubs for two error types (`ParseError` and `CraneError`)
- Stubs for some of the key `Stacks` methods (`apply_instruction`, `apply_instructions`, and `tops_string`) that are needed in `main` and the tests.
  - You'll almost certainly want more methods in various types, so feel free to
    create them as you go.
- Stubs for some of the important trait implementations used in parsing the input.
- Stub for `impl PartialEq<Vec<char>> for Stack` so we can compare a `Stack` and
  a `Vec<char>` in the tests.

### Add error variants as you go

The two stubs for error types (`ParseError` and `CraneError`) start out as empty
`enums`. You'll want to add variants to capture various potential error cases as you
discover them. The stack specification lines, for example, are supposed to start with
a number. If parsing that bit of the string to an integer fails, you might want to capture
that in something like a `StackIdParseError` error:

```rust
enum ParseError {
    StackIdParseError,
}
```

If you really want to get fancy, you might even capture the parsing error so that gets
reported back to the user:

```rust
enum ParseError {
    StackIdParseError(ParseIntError),
}
```

### Parsing stacks and instructions

You'll almost certainly want to use functions such as `.lines()`
and `split_ascii_whitespace()` to aid in the parsing.

Make sure you check that strings you get after you split stack lines all have single
characters, and return an appropriate variant of `ParseError` if they don't.

Parsing the crane instructions is a little odd since the first, third, and fifth
element in an instruction don't add any content since they're always the same. Probably the
easist solution is to just reach into the results of splitting that line and getting
items 1, 3, and 5, which contain the bits that you want. For something fancier (which
you can totally ignore) I actually took advantage of the fact that all those indices
were odd and use `filter` to get those elements from the line and parsed them all in
one go with `map`.

### Implementing `PartialEq<Vec<char>> for Stack`

In the tests, it's nice if you can directly compare a vector of characters and a
`Stack` with something like:

```rust
assert_eq!(stacks.stack[0], vec!['Z', 'N'])
```

This won't work "out of the box" because the types don't match. We can, however,

```rust
impl PartialEq<Vec<char>> for Stack
```

which gives us a `.eq()` method on stacks, so we can say things like `stack.eq(vec)`.
The cool thing is that this also lets us write `stack == vec`, or even the `assert_eq!()`
above.

This isn't something we need to actually solve the problem, but it makes it a lot
nicer to write the tests, so I figured I'd include it.
