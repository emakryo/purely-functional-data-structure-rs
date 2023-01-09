# Rust implementation of Purely Functional Data Structure

- https://www.cambridge.org/core/books/purely-functional-data-structures/0409255DA1B48FA731859AC72E34D494
- [JP] https://tatsu-zine.com/books/purely-functional-data-structures

## Limitations

- Large input cases stack overflow due to missing tail call optimization
- Some algorithms are absent because polymorphic recursion is not allowed
  - `ImplicitQueue`
  - `SimpleCatenableDeque`
