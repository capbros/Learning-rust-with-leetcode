# LeetCode Rust Solutions

This workspace is set up as a small Rust library for LeetCode-style solutions.

## Layout

- `src/lib.rs` exposes the solution modules.
- `src/problems/` contains one file per problem.
- Each problem module can include its own unit tests.

## Adding a new problem

1. Create `src/problems/<problem_name>.rs`.
2. Add `pub mod <problem_name>;` to `src/problems/mod.rs`.
3. Implement the solution and add tests in the same module.
4. Run `cargo test`.
