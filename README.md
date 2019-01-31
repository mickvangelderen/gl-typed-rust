If you're looking for a safe OpenGL abstraction, check out
[glium](https://github.com/glium/glium). This project is for people who want
total control over what happens and easily see what OpenGL functions are being
called.

## Goals

1. Implement OpenGL API with more type safety.
2. Everything *must* be zero-cost.
3. Rename OpenGL identifiers to comply with Rust's naming conventions.

Safety is not on the list because I don't trust myself enough to make that
judgement. It may be possible to make the API safe without violating the
zero-cost constraint.

I think it is impossible to prevent all OpenGL errors from occurring (like glium
promises) under the zero-cost constraint.

