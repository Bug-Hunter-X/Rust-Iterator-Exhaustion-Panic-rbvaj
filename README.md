# Rust Iterator Exhaustion

This repository demonstrates a common error in Rust programming involving iterators and how to avoid panics caused by iterator exhaustion.

The `bug.rs` file contains code that attempts to access elements from an iterator after it has been exhausted. This results in a runtime panic.

The `bugSolution.rs` file shows how to correctly handle iterator exhaustion, providing a robust solution.

## How to run the code

1. Clone the repository
2. Navigate to the repository directory
3. Compile and run the code using the Rust compiler: `rustc bug.rs && ./bug` (for the buggy version) and `rustc bugSolution.rs && ./bugSolution` (for the solution)

## Understanding the issue

Iterators in Rust are consumed as they are used. Once an iterator is exhausted, attempting to access further elements using methods like `next()` will result in a panic.  The solution involves carefully considering how the iterator is used and how to handle the end of the iteration gracefully.