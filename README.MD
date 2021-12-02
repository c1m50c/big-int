# **big-int**

[![Build](https://github.com/c1m50c/big-int/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/c1m50c/big-int/actions/workflows/build.yml)

Library implementing a **big int**, which is a numerically limitless integer.
For example the Rust built-in type `i32` can only range from `-2147483648 | -(2^31)` to `2147483648 | (2 ^ 31)`,
with **big int** this range is virtually non-existent.


## **Testing**
```bash
$ cd big-int
$ cargo test
...
test result: ok # If things go well test should pass.
```